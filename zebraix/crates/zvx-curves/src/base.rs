//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::threes::RatQuadOoeSubclassed;
use crate::threes::TEval;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use zvx_base::default_unit_f64;
use zvx_base::is_default;
use zvx_base::is_default_unit_f64;
use zvx_base::CubicPath;
use zvx_base::HyperbolicPath;

const fn scale_3(x: &[f64; 3], s: f64) -> [f64; 3] {
   [s * x[0], s * x[1], s * x[2]]
}

// Sigma is a bilinear transformation of `t` that does not change the end-points of the
// curve. Thus conversion to a path does not generally need to involve sigma.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Curve<T: Default + PartialEq> {
   #[serde(skip_serializing_if = "is_default")]
   pub path: T,
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

pub trait CurveEval {
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]>;
   fn eval_with_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ZebraixAngle {
   Quadrant(f64),
   Radians(f64),
   TanHalf(f64),
}

impl ZebraixAngle {
   #[inline]
   #[must_use]
   pub fn in_radians(&self) -> f64 {
      match self {
         Self::Quadrant(q) => 0.5 * q * std::f64::consts::PI,
         Self::Radians(r) => *r,
         Self::TanHalf(t) => 2.0 * t.atan(),
      }
   }

   // This is really not good. We should deal with half the opening angle, or otherwise we get
   // strangeness as regards interpretation of angles (such as subtracting 2 pi from angle.
   #[inline]
   #[must_use]
   pub fn cos(&self) -> f64 {
      match self {
         Self::Quadrant(_) => self.in_radians().cos(),
         Self::Radians(r) => r.cos(),
         Self::TanHalf(t) => (1.0 - t * t) / (1.0 + t * t),
      }
   }
}

impl Default for ZebraixAngle {
   fn default() -> Self {
      Self::Quadrant(1.0)
   }
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Clone)]
pub struct FourPointRatQuad {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 4],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadPolyPath {
   pub r: [f64; 2], // Range.
   pub a: [f64; 3], // Denominator, as a[2] * t^2 + a[1] * t... .
   pub b: [f64; 3], // Numerator for x component.
   pub c: [f64; 3], // Numerator for y component.
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct RegularizedRatQuadPath {
   pub range_bound: f64, // Range is [-range_bound, range_bound].
   pub a_0: f64,         // Denominator, as a[2] * t^2 + a[1] * t... .
   pub a_2: f64,
   pub b: [f64; 3], // Numerator for x component.
   pub c: [f64; 3], // Numerator for y component.
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct ThreePointAngleRepr {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 3],
   #[serde(skip_serializing_if = "is_default")]
   pub angle: ZebraixAngle,
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

// As yet little used.
#[derive(Debug, Deserialize, Serialize, DefaultFromSerde, PartialEq, Clone)]
pub struct MidDiffCubiLinearRepr {
   pub r: [f64; 2], // Range.
   pub x: [f64; 4],
   pub y: [f64; 4],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

// Recreate as specified Cubic or SpecifiedCubiLinear when reworking managed curves.
//
// #[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
// pub enum CubiLinear {
//    #[default]
//    Nothing,
//    FourPoint(Curve<CubicPath>),
//    MidDiff(MidDiffCubiLinearRepr),
// }

#[derive(Debug, Serialize, Default, PartialEq, Clone)]
pub enum BaseRatQuad {
   #[default]
   Nothing,
   RationalPoly(Curve<RatQuadPolyPath>),
   RegularizedSymmetric(Curve<RegularizedRatQuadPath>), // SymmetricRange with zero middle denominator coefficient.
}

#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub enum SpecifiedRatQuad {
   #[default]
   None, // For, say, polynomial directly specified.
   FourPoint(FourPointRatQuad),
   ThreePointAngle(ThreePointAngleRepr), // Form p, angle, sigma.
}

impl TEval for RatQuadPolyPath {
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());

      for item in t {
         let denom_reciprocal = 1.0 / self.a[2].mul_add(*item, self.a[1]).mul_add(*item, self.a[0]);
         ret_val.push([
            self.b[2].mul_add(*item, self.b[1]).mul_add(*item, self.b[0]) * denom_reciprocal,
            self.c[2].mul_add(*item, self.c[1]).mul_add(*item, self.c[0]) * denom_reciprocal,
         ]);
      }

      ret_val
   }
}

impl Curve<RatQuadPolyPath> {
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   fn rq_apply_bilinear_unranged(&self, w: f64, x: f64, y: f64, z: f64) -> Self {
      // let norm = 1.0 / ((w * w + x * x) * (y * y + z * z)).sqrt().sqrt();
      // w *= norm;
      // x *= norm;
      // y *= norm;
      // z *= norm;

      let a = [
         self.path.a[0] * z * z + self.path.a[1] * x * z + self.path.a[2] * x * x,
         2.0 * self.path.a[0] * y * z
            + self.path.a[1] * (x * y + w * z)
            + 2.0 * self.path.a[2] * w * x,
         self.path.a[0] * y * y + self.path.a[1] * w * y + self.path.a[2] * w * w,
      ];
      let b = [
         self.path.b[0] * z * z + self.path.b[1] * x * z + self.path.b[2] * x * x,
         2.0 * self.path.b[0] * y * z
            + self.path.b[1] * (x * y + w * z)
            + 2.0 * self.path.b[2] * w * x,
         self.path.b[0] * y * y + self.path.b[1] * w * y + self.path.b[2] * w * w,
      ];
      let c = [
         self.path.c[0] * z * z + self.path.c[1] * x * z + self.path.c[2] * x * x,
         2.0 * self.path.c[0] * y * z
            + self.path.c[1] * (x * y + w * z)
            + 2.0 * self.path.c[2] * w * x,
         self.path.c[0] * y * y + self.path.c[1] * w * y + self.path.c[2] * w * w,
      ];

      let f = 1.0 / (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).abs().sqrt();

      Self {
         path: RatQuadPolyPath {
            a: scale_3(&a, f),
            b: scale_3(&b, f),
            c: scale_3(&c, f),
            r: self.path.r,
         },
         sigma: self.sigma,
      }
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   pub fn rq_apply_bilinear(&self, sigma_n: f64, sigma_d: f64) -> Self {
      let p = -self.path.r[0];
      let q = self.path.r[1];

      self.rq_apply_bilinear_unranged(
         sigma_n * q + sigma_d * p,
         (sigma_n - sigma_d) * p * q,
         sigma_n - sigma_d,
         sigma_d * q + sigma_n * p,
      )
   }

   // Only used at present to characterize for case of OOE subtype parabola.
   #[inline]
   #[must_use]
   pub fn rq_characterize_endpoints(&self) -> ([f64; 4], [f64; 4]) {
      let mut x = [0.0; 4];
      let mut y = [0.0; 4];

      let speed_scale = self.path.r[1] - self.path.r[0];
      for (outer, inner, t) in [(0, 1, self.path.r[0]), (3, 2, self.path.r[1])] {
         let recip_a = 1.0 / self.path.a[2].mul_add(t, self.path.a[1]).mul_add(t, self.path.a[0]);
         let b = self.path.b[2].mul_add(t, self.path.b[1]).mul_add(t, self.path.b[0]);
         let c = self.path.c[2].mul_add(t, self.path.c[1]).mul_add(t, self.path.c[0]);
         let da = self.path.a[2].mul_add(2.0 * t, self.path.a[1]) * speed_scale;
         let db = self.path.b[2].mul_add(2.0 * t, self.path.b[1]) * speed_scale;
         let dc = self.path.c[2].mul_add(2.0 * t, self.path.c[1]) * speed_scale;
         x[outer] = b * recip_a;
         y[outer] = c * recip_a;
         x[inner] = (-b * da).mul_add(recip_a, db) * recip_a;
         y[inner] = (-c * da).mul_add(recip_a, dc) * recip_a;
      }
      (x, y)
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::many_single_char_names)]
   pub fn figure_symmetric_range_rat_quad(&self) -> Self {
      // Replace t with t - d.
      let d = 0.5 * (self.path.r[0] + self.path.r[1]);
      let r_half = 0.5 * (self.path.r[1] - self.path.r[0]);

      let a = [
         d * (d * self.path.a[2] + self.path.a[1]) + self.path.a[0],
         2.0 * d * self.path.a[2] + self.path.a[1],
         self.path.a[2],
      ];
      let b = [
         d * (d * self.path.b[2] + self.path.b[1]) + self.path.b[0],
         2.0 * d * self.path.b[2] + self.path.b[1],
         self.path.b[2],
      ];
      let c = [
         d * (d * self.path.c[2] + self.path.c[1]) + self.path.c[0],
         2.0 * d * self.path.c[2] + self.path.c[1],
         self.path.c[2],
      ];

      let r = [-r_half, r_half];
      Self { path: RatQuadPolyPath { r, a, b, c }, sigma: self.sigma }
   }
}

impl CurveEval for Curve<HyperbolicPath> {
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      self.path.eval_no_bilinear(t)
   }
   fn eval_with_bilinear(&self, _t: &[f64]) -> Vec<[f64; 2]> {
      unimplemented!("It takes time.");
   }
}

impl CurveEval for Curve<RatQuadPolyPath> {
   // This method may lack tests.
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      self.path.eval_no_bilinear(t)
   }
   fn eval_with_bilinear(&self, _t: &[f64]) -> Vec<[f64; 2]> {
      unimplemented!("It takes time.");
   }
}

#[allow(clippy::suboptimal_flops)]
impl From<&HyperbolicPath> for RegularizedRatQuadPath {
   fn from(hyper: &HyperbolicPath) -> Self {
      let lambda = hyper.lambda;
      let mu = hyper.mu;
      let b = [
         lambda * (hyper.offset[0] * lambda + hyper.minus_partial[0] + hyper.plus_partial[0]),
         mu * (hyper.minus_partial[0] - hyper.plus_partial[0]),
         -hyper.offset[0] * mu * mu,
      ];
      let c = [
         lambda * (hyper.offset[1] * lambda + hyper.minus_partial[1] + hyper.plus_partial[1]),
         mu * (hyper.minus_partial[1] - hyper.plus_partial[1]),
         -hyper.offset[1] * mu * mu,
      ];
      let a_0 = lambda * lambda;
      let a_2 = -mu * mu;

      Self { range_bound: hyper.range_bound, a_0, a_2, b, c }
   }
}

impl From<&Curve<HyperbolicPath>> for Curve<RegularizedRatQuadPath> {
   fn from(curve: &Curve<HyperbolicPath>) -> Self {
      Self { path: RegularizedRatQuadPath::from(&curve.path), sigma: curve.sigma }
   }
}

#[allow(clippy::suboptimal_flops)]
impl From<&RegularizedRatQuadPath> for RatQuadPolyPath {
   fn from(regular: &RegularizedRatQuadPath) -> Self {
      Self {
         r: [-regular.range_bound, regular.range_bound],
         a: [regular.a_0, 0.0, regular.a_2],
         b: regular.b,
         c: regular.c,
      }
   }
}

impl From<&Curve<RegularizedRatQuadPath>> for Curve<RatQuadPolyPath> {
   fn from(curve: &Curve<RegularizedRatQuadPath>) -> Self {
      Self { path: RatQuadPolyPath::from(&curve.path), sigma: curve.sigma }
   }
}

impl Curve<RegularizedRatQuadPath> {
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   pub fn convert_to_parabolic(&self) -> Curve<CubicPath> {
      let (x, y) = Into::<Curve<RatQuadPolyPath>>::into(self).rq_characterize_endpoints();
      let f = 1.0 / 3.0;
      let four_c = [
         [x[0], y[0]],
         [x[0] + f * x[1], y[0] + f * y[1]],
         [x[3] - f * x[2], y[3] - f * y[2]],
         [x[3], y[3]],
      ];

      // assert_eq!(self.range_bound, 0.0);
      Curve::<CubicPath> {
         path: CubicPath { r: [-self.path.range_bound, self.path.range_bound], p: four_c },
         sigma: self.sigma,
      }
   }

   // At present there is no proper testing of s. Manual inspection verifies that negating all
   // a, b and c in the input leaves the output invariant.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[must_use]
   pub fn convert_to_hyperbolic(&self) -> Curve<HyperbolicPath> {
      let s = self.path.a_0.signum();

      let lambda = (s * self.path.a_0).sqrt();
      assert!(-s * self.path.a_2 > 0.0);
      let mu = (-s * self.path.a_2).sqrt();
      let r_lambda = 1.0 / lambda;
      let r_mu = 1.0 / mu;
      let r_a_2 = 1.0 / self.path.a_2;

      let offset = [self.path.b[2] * r_a_2, self.path.c[2] * r_a_2];

      let f = 0.5 * s;
      let plus_partial = [
         f * (self.path.b[0] * r_lambda
            + (-self.path.b[1] + lambda * r_mu * self.path.b[2]) * r_mu),
         f * (self.path.c[0] * r_lambda
            + (-self.path.c[1] + lambda * r_mu * self.path.c[2]) * r_mu),
      ];
      let minus_partial = [
         f * (self.path.b[0] * r_lambda + (self.path.b[1] + lambda * r_mu * self.path.b[2]) * r_mu),
         f * (self.path.c[0] * r_lambda + (self.path.c[1] + lambda * r_mu * self.path.c[2]) * r_mu),
      ];

      Curve::<HyperbolicPath> {
         path: HyperbolicPath {
            range_bound: self.path.range_bound,
            lambda,
            mu,
            offset,
            plus_partial,
            minus_partial,
         },
         sigma: self.sigma,
      }
   }

   // #[must_use]
   // pub fn eval(&self, t: &[f64]) -> Result<Vec<[f64; 2]>, &'static str> {
   //    let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());

   //    //  XXXXXXXXXXXXXXXXX

   //    match self {
   //       Self::RationalPoly(repr) => Ok(*repr),
   //       Self::RegularizedSymmetric(symm) => Ok(Curve::<RatQuadPolyPath> {
   //          r: [-symm.range_bound, symm.range_bound],
   //          a: [symm.a_0, 0.0, symm.a_2],
   //          b: symm.b,
   //          c: symm.c,
   //          sigma: symm.sigma,
   //       }),
   //       Self::Nothing
   //       | Self::FourPoint(_)
   //       | Self::ThreePointAngle(_) => Err("QR not  proper rational poly."),
   //    }

   //    for item in t {
   //       let denom_reciprocal = 1.0 / self.a[2].mul_add(*item, self.a[1]).mul_add(*item, self.a[0]);
   //       ret_val.push([
   //          self.b[2].mul_add(*item, self.b[1]).mul_add(*item, self.b[0]) * denom_reciprocal,
   //          self.c[2].mul_add(*item, self.c[1]).mul_add(*item, self.c[0]) * denom_reciprocal,
   //       ]);
   //    }

   //    ret_val
   // }
}

impl RatQuadOoeSubclassed {
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn create_from_regularized(
      reg_curve: &Curve<RegularizedRatQuadPath>,
      tolerance: f64,
   ) -> Self {
      let mut rat_poly = reg_curve.clone();
      let orig_rat_poly = reg_curve;

      let r = rat_poly.path.range_bound;
      if (rat_poly.path.a_2.abs() * r * r) < (rat_poly.path.a_0.abs() * tolerance) {
         Self::Parabolic(orig_rat_poly.convert_to_parabolic())
      } else if rat_poly.path.a_2.signum() == rat_poly.path.a_0.signum() {
         // TODO: Better handle cases where s or f might be infinite.
         let s = 1.0 / rat_poly.path.a_0;
         let f = 1.0 / rat_poly.path.a_2;
         rat_poly.path.a_0 = 1.0;
         rat_poly.path.a_2 *= s;

         {
            let offset = 0.5 * (s * rat_poly.path.b[0] + f * rat_poly.path.b[2]);
            let even = 0.5 * (s * rat_poly.path.b[0] - f * rat_poly.path.b[2]);
            let odd = rat_poly.path.b[1] * s;
            rat_poly.path.b = [offset, odd, even];
         }
         {
            let offset = 0.5 * (s * rat_poly.path.c[0] + f * rat_poly.path.c[2]);
            let even = 0.5 * (s * rat_poly.path.c[0] - f * rat_poly.path.c[2]);
            let odd = rat_poly.path.c[1] * s;
            rat_poly.path.c = [offset, odd, even];
         }

         let sss = 1.0 / rat_poly.path.a_2.sqrt();
         let (sx, sy) = (0.5 * sss * rat_poly.path.b[1], 0.5 * sss * rat_poly.path.c[1]);
         let (cx, cy) = (rat_poly.path.b[2], rat_poly.path.c[2]);
         let determinant = sx * cy - cx * sy;
         let frobenius_squared = sx * sx + sy * sy + cx * cx + cy * cy;
         if determinant.abs() < (frobenius_squared * tolerance) {
            // From the plotting point of view this is not a degenerate case, but renderers may
            // want the transformation to be invertible.
            //
            // If one singular value is much larger than the other, the frobenius norm
            // (squared) will be approximately the square of larger.  The determinant is their
            // product, and so the condition effectively compares their magnitude (for small
            // tolerances).

            Self::Parabolic(orig_rat_poly.convert_to_parabolic())
         } else {
            // Only outcome that actually uses OOE form.
            Self::Elliptical(rat_poly)
         }
      } else {
         // {
         let hyperbolic_form = orig_rat_poly.convert_to_hyperbolic();
         let poly = orig_rat_poly;
         let reconstructed: Curve<RegularizedRatQuadPath> = From::from(&hyperbolic_form);
         //      Reg_Curve {
         //    r: orig_rat_poly.r,
         //    a_0: orig_rat_poly.a_0,
         //    a_2: orig_rat_poly.a_2,
         //    // a: [orig_rat_poly.a[0], 0.0, orig_rat_poly.a[2]],
         //    b: orig_rat_poly.b,
         //    c: orig_rat_poly.c,
         //    sigma: orig_rat_poly.sigma,
         // };
         println!("a: [{}, {}, {}]", poly.path.a_0, 0.0, poly.path.a_2);
         println!("b: [{}, {}, {}]", poly.path.b[0], poly.path.b[1], poly.path.b[2]);
         println!("c: [{}, {}, {}]", poly.path.c[0], poly.path.c[1], poly.path.c[2]);

         println!("a: [{}, {}, {}]", reconstructed.path.a_0, 0.0, reconstructed.path.a_2);
         println!(
            "b: [{}, {}, {}]",
            reconstructed.path.b[0], reconstructed.path.b[1], reconstructed.path.b[2]
         );
         println!(
            "c: [{}, {}, {}]",
            reconstructed.path.c[0], reconstructed.path.c[1], reconstructed.path.c[2]
         );

         // let reconstructed_b = [
         //    offset[0] * beta * beta + minus_fraction[0] * beta + plus_fraction[0] * beta,
         //    minus_fraction[0] * gamma - plus_fraction[0] * gamma,
         //    -offset[0] * gamma * gamma,
         // ];
         // let reconstructed_c = [
         //    offset[1] * beta * beta + minus_fraction[1] * beta + plus_fraction[1] * beta,
         //    minus_fraction[1] * gamma - plus_fraction[1] * gamma,
         //    -offset[1] * gamma * gamma,
         // ];
         // println!("recon a: [{}, {}, {}]", beta * beta, 0.0, -gamma * gamma);
         // println!(
         //    "recon b: [{}, {}, {}]",
         //    reconstructed_b[0], reconstructed_b[1], reconstructed_b[2]
         // );
         // println!(
         //    "recon c: [{}, {}, {}]",
         //    reconstructed_c[0], reconstructed_c[1], reconstructed_c[2]
         // );
         // }

         Self::Hyperbolic(hyperbolic_form)
      }
   }
}

impl BaseRatQuad {
   #[allow(clippy::match_same_arms)]
   #[allow(clippy::missing_errors_doc)]
   pub fn get_poly_xxx(&self) -> Result<Curve<RatQuadPolyPath>, &'static str> {
      match self {
         Self::RationalPoly(repr) => Ok(repr.clone()),
         Self::RegularizedSymmetric(symm) => Ok(Curve::<RatQuadPolyPath> {
            path: RatQuadPolyPath {
               r: [-symm.path.range_bound, symm.path.range_bound],
               a: [symm.path.a_0, 0.0, symm.path.a_2],
               b: symm.path.b,
               c: symm.path.c,
            },
            sigma: symm.sigma,
         }),
         Self::Nothing => Err("QR not  proper rational poly."),
      }
   }

   #[must_use]
   #[allow(clippy::missing_panics_doc)]
   pub fn eval_quad(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let ret_val = Vec::<[f64; 2]>::with_capacity(t.len());

      assert!(matches!(self, Self::RationalPoly { .. }));
      if let Self::RationalPoly(rat_poly) = self {
         rat_poly.eval_no_bilinear(t)
      } else {
         ret_val
      }
   }

   #[inline]
   // Applies bilinear transformation with factor sigma, preserving the range.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn apply_bilinear(&mut self, sigma_n: f64, sigma_d: f64) -> Result<(), &'static str> {
      match self {
         Self::Nothing => unimplemented!("Nothing form is invalid from construction."),
         Self::RegularizedSymmetric(_) => {
            Err("Applying bilinear to regularized will downgrade it.")
         }
         Self::RationalPoly(rat_poly) => {
            *self = Self::RationalPoly(rat_poly.rq_apply_bilinear(sigma_n, sigma_d));
            Ok(())
         }
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      if let Self::RationalPoly(rat_poly_extracted) = self {
         let rat_poly = rat_poly_extracted.figure_symmetric_range_rat_quad();

         let r_both = rat_poly.path.r[1];
         let a_s = rat_poly.path.a[2] * r_both * r_both + rat_poly.path.a[0];
         // let a_d = rat_poly.path.a[2] * r * r - rat_poly.path.a[0];
         let combo_s = a_s + rat_poly.path.a[1] * r_both;
         let combo_d = a_s - rat_poly.path.a[1] * r_both;

         let sigma_n = combo_d.abs().sqrt();
         let sigma_d = combo_s.abs().sqrt();

         *self = Self::RationalPoly(rat_poly);
         self.apply_bilinear(sigma_n, sigma_d)?;

         if let Self::RationalPoly(check_poly) = self {
            assert!(check_poly.path.a[1].abs() < 0.001);
            *self = Self::RegularizedSymmetric(Curve::<RegularizedRatQuadPath> {
               path: RegularizedRatQuadPath {
                  range_bound: check_poly.path.r[1],
                  a_0: check_poly.path.a[0],
                  a_2: check_poly.path.a[2],
                  b: check_poly.path.b,
                  c: check_poly.path.c,
               },
               sigma: check_poly.sigma,
            });
         } else {
            panic!("Unreachable");
         }

         Ok(())
      } else {
         Err("Can only raise from rat-poly to regularized-symmetric form.")
      }
   }

   // Only used at present to characterize for case of OOE subtype parabola.
   #[inline]
   #[must_use]
   #[allow(clippy::missing_panics_doc)]
   pub fn characterize_endpoints(&self) -> ([f64; 4], [f64; 4]) {
      let x = [0.0; 4];
      let y = [0.0; 4];

      assert!(matches!(self, Self::RationalPoly { .. }));
      if let Self::RationalPoly(rat_poly) = self {
         rat_poly.rq_characterize_endpoints()
      } else {
         (x, y)
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::many_single_char_names)]
   // Weighted is stored in same struct as `Curve<RatQuadPolyPath>`, but is really a different
   // representation of a rational quadratic.
   pub fn create_from_weighted(weighted: &Curve<RatQuadPolyPath>) -> Result<Self, &'static str> {
      // Get from rat_poly.sigma once confirmed working.
      let sigma = 1.0;
      let v = weighted.path.r[0];
      let w = weighted.path.r[1];
      let a;
      let b;
      let c;
      {
         let h0 = weighted.path.a[0];
         let h1 = sigma * weighted.path.a[1];
         let h2 = sigma * sigma * weighted.path.a[2];
         a = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = weighted.path.b[0];
         let h1 = sigma * weighted.path.b[1];
         let h2 = sigma * sigma * weighted.path.b[2];
         b = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = weighted.path.c[0];
         let h1 = sigma * weighted.path.c[1];
         let h2 = sigma * sigma * weighted.path.c[2];
         c = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }

      Ok(Self::RationalPoly(Curve::<RatQuadPolyPath> {
         path: RatQuadPolyPath { r: weighted.path.r, a, b, c },
         sigma: weighted.sigma,
      }))
   }
}

#[allow(clippy::missing_errors_doc)]
impl Curve<CubicPath> {
   #[inline]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   pub fn eval_part(b: f64, a: f64, coeffs: &[f64; 4], multiplier: f64) -> f64 {
      multiplier
         * (b * b * b * coeffs[0]
            + 3.0 * b * b * a * coeffs[1]
            + 3.0 * b * a * a * coeffs[2]
            + a * a * a * coeffs[3])
   }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   pub fn eval(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let a = self.sigma * (*item - self.path.r[0]);
         let b = self.path.r[1] - *item;
         let f0 = 1.0 / (b + a);
         let recip_denom = f0 * f0 * f0;
         let in_x = [self.path.p[0][0], self.path.p[1][0], self.path.p[2][0], self.path.p[3][0]];
         let in_y = [self.path.p[0][1], self.path.p[1][1], self.path.p[2][1], self.path.p[3][1]];
         let x = Self::eval_part(b, a, &in_x, recip_denom);
         let y = Self::eval_part(b, a, &in_y, recip_denom);
         ret_val.push([x, y]);
      }
      ret_val
   }

   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   pub fn select_range(&mut self, new_range: [f64; 2]) {
      let mut new_x = [0.0; 4];
      let mut new_y = [0.0; 4];

      let a_k = self.sigma * (new_range[0] - self.path.r[0]);
      let b_k = self.path.r[1] - new_range[0];
      let a_l = self.sigma * (new_range[1] - self.path.r[0]);
      let b_l = self.path.r[1] - new_range[1];
      let f0_k = 1.0 / (b_k + a_k);
      let recip_denom_k = f0_k * f0_k * f0_k;
      let f0_l = 1.0 / (b_l + a_l);
      let recip_denom_l = f0_l * f0_l * f0_l;
      let in_x = [self.path.p[0][0], self.path.p[1][0], self.path.p[2][0], self.path.p[3][0]];
      let in_y = [self.path.p[0][1], self.path.p[1][1], self.path.p[2][1], self.path.p[3][1]];
      new_x[0] = Self::eval_part(b_k, a_k, &in_x, recip_denom_k);
      new_y[0] = Self::eval_part(b_k, a_k, &in_y, recip_denom_k);
      new_x[3] = Self::eval_part(b_l, a_l, &in_x, recip_denom_l);
      new_y[3] = Self::eval_part(b_l, a_l, &in_y, recip_denom_l);
      let kl_numerator_k = self.sigma * self.path.r[1] * (new_range[0] - self.path.r[0])
         + self.path.r[0] * (self.path.r[1] - new_range[0]);
      let kl_numerator_l = self.sigma * self.path.r[1] * (new_range[1] - self.path.r[0])
         + self.path.r[0] * (self.path.r[1] - new_range[1]);
      // This is [k, l] bilinearly transformed.
      let selected_range_bilineared = kl_numerator_l / (a_l + b_l) - kl_numerator_k / (a_k + b_k);
      let fudge_k = selected_range_bilineared / (self.path.r[1] - self.path.r[0]);
      let fudge_l = selected_range_bilineared / (self.path.r[1] - self.path.r[0]);
      // assert_eq!(1.0 / f0_k, 0.0);
      let dx_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (in_x[1] - in_x[0])
            + 2.0 * b_k * a_k * (in_x[2] - in_x[1])
            + a_k * a_k * (in_x[3] - in_x[2]));
      new_x[1] = new_x[0] + dx_1;
      let dy_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (in_y[1] - in_y[0])
            + 2.0 * b_k * a_k * (in_y[2] - in_y[1])
            + a_k * a_k * (in_y[3] - in_y[2]));
      new_y[1] = new_y[0] + dy_1;
      let dx_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (in_x[1] - in_x[0])
            + 2.0 * b_l * a_l * (in_x[2] - in_x[1])
            + a_l * a_l * (in_x[3] - in_x[2]));
      new_x[2] = new_x[3] - dx_1;
      let dy_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (in_y[1] - in_y[0])
            + 2.0 * b_l * a_l * (in_y[2] - in_y[1])
            + a_l * a_l * (in_y[3] - in_y[2]));
      new_y[2] = new_y[3] - dy_1;

      self.sigma = (a_l + b_l) / (a_k + b_k);
      self.path.p =
         [[new_x[0], new_y[0]], [new_x[1], new_y[1]], [new_x[2], new_y[2]], [new_x[3], new_y[3]]];
      self.path.r = new_range;
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      self.path.p[0][0] += d[0];
      self.path.p[1][0] += d[0];
      self.path.p[2][0] += d[0];
      self.path.p[3][0] += d[0];
      self.path.p[0][1] += d[1];
      self.path.p[1][1] += d[1];
      self.path.p[2][1] += d[1];
      self.path.p[3][1] += d[1];
   }

   pub fn bilinear_transform(&mut self, sigma: f64) {
      self.sigma *= sigma;
   }

   pub fn adjust_range(&mut self, new_range: [f64; 2]) {
      self.path.r = new_range;
   }
}
