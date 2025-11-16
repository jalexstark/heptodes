//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::base::TEval;
use crate::threes::RatQuadOoeSubclassed;
use crate::{
   default_unit_sigma, is_default_unit_sigma, q_mat_power_to_weighted, q_mat_weighted_to_power,
   q_reduce, rat_quad_expand_power, rat_quad_power_eval, Curve, CurveEval, CurveMatrix,
   CurveTransform, QMat, ZebraixAngle,
};
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use zvx_base::{is_default, CubicHomog, CubicPath, HyperbolicPath, RatQuadHomog};

#[cfg(test)]
use crate::RatQuadHomogWrapped;
#[cfg(test)]
use approx::assert_abs_diff_eq;

// const fn scale_3(x: &[f64; 3], s: f64) -> [f64; 3] {
//    [s * x[0], s * x[1], s * x[2]]
// }

// const fn displace_3(p: &mut [[f64; 2]; 3], d: [f64; 2]) {
//    p[0][0] += d[0];
//    p[1][0] += d[0];
//    p[2][0] += d[0];
//    p[0][1] += d[1];
//    p[1][1] += d[1];
//    p[2][1] += d[1];
//}

// // Update by adding 3x1 vector multiplied by 2-D point.
// const fn mul_add_3_1_2(p: &mut [[f64; 2]; 3], v: &[f64; 3], m: [f64; 2]) {
//    p[0][0] += v[0] * m[0];
//    p[1][0] += v[1] * m[0];
//    p[2][0] += v[2] * m[0];
//    p[0][1] += v[0] * m[1];
//    p[1][1] += v[1] * m[1];
//    p[2][1] += v[2] * m[1];
// }

// Update by adding 3x1 vector multiplied by scalar.
const fn mul_add_3_1_1(p: &mut [f64; 3], v: &[f64; 3], m: f64) {
   p[0] += v[0] * m;
   p[1] += v[1] * m;
   p[2] += v[2] * m;
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Clone)]
pub struct FourPointRatQuad {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 4],
   #[serde(skip_serializing_if = "is_default_unit_sigma", default = "default_unit_sigma")]
   pub sigma: (f64, f64),
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadPolyPath {
   pub r: [f64; 2], // Range.
   pub a: [f64; 3], // Denominator, as a[2] * t^2 + a[1] * t... .
   pub b: [f64; 3], // Numerator for x component.
   pub c: [f64; 3], // Numerator for y component.
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadHomogPower {
   pub r: [f64; 2], // Range.
   pub h: RatQuadHomog,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadHomogWeighted {
   pub r: [f64; 2], // Range.
   pub h: RatQuadHomog,
}

impl From<&RatQuadHomogWeighted> for RatQuadHomogPower {
   fn from(weighted: &RatQuadHomogWeighted) -> Self {
      let r = &weighted.r;

      let tran_q_mat = q_mat_weighted_to_power(r);
      let out_quad_homog = weighted.h.apply_q_mat(&tran_q_mat);

      Self { r: *r, h: out_quad_homog }
   }
}

impl From<&RatQuadHomogPower> for RatQuadHomogWeighted {
   fn from(power: &RatQuadHomogPower) -> Self {
      let r = &power.r;

      let tran_q_mat = q_mat_power_to_weighted(r);
      let out_quad_homog = power.h.apply_q_mat(&tran_q_mat);

      Self { r: *r, h: out_quad_homog }
   }
}

impl From<&RatQuadPolyPath> for RatQuadHomogPower {
   fn from(poly: &RatQuadPolyPath) -> Self {
      Self { r: poly.r, h: RatQuadHomog([poly.b, poly.c, poly.a]) }
   }
}

impl From<&RatQuadHomogPower> for RatQuadPolyPath {
   fn from(homog: &RatQuadHomogPower) -> Self {
      Self { r: homog.r, a: homog.h.0[2], b: homog.h.0[0], c: homog.h.0[1] }
   }
}

impl From<&RatQuadPolyPath> for RatQuadHomogWeighted {
   fn from(poly: &RatQuadPolyPath) -> Self {
      Self { r: poly.r, h: RatQuadHomog([poly.b, poly.c, poly.a]) }
   }
}

impl From<&RatQuadHomogWeighted> for RatQuadPolyPath {
   fn from(homog: &RatQuadHomogWeighted) -> Self {
      Self { r: homog.r, a: homog.h.0[2], b: homog.h.0[0], c: homog.h.0[1] }
   }
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
   #[serde(skip_serializing_if = "is_default_unit_sigma", default = "default_unit_sigma")]
   pub sigma: (f64, f64),
}

#[derive(Serialize, Debug, Default, Clone, PartialEq)]
pub enum SpecifiedRatQuad {
   #[default]
   None, // For, say, polynomial directly specified.
   FourPoint(FourPointRatQuad),
   ThreePointAngle(ThreePointAngleRepr), // Form p, angle, sigma.
}

impl CurveTransform for Curve<RatQuadPolyPath> {
   // Not yet tested.
   fn displace(&mut self, d: [f64; 2]) {
      mul_add_3_1_1(&mut self.path.b, &self.path.a, d[0]);
      mul_add_3_1_1(&mut self.path.c, &self.path.a, d[1]);
   }

   fn bilinear_transform(&mut self, sigma_ratio: (f64, f64)) {
      self.sigma.0 *= sigma_ratio.0;
      self.sigma.1 *= sigma_ratio.1;
   }

   fn raw_change_range(&mut self, new_range: [f64; 2]) {
      self.path.r = new_range;
   }

   // NOTE: Incomplete, not accounting for sigma.
   fn select_range(&mut self, new_range: [f64; 2]) {
      self.path.r = new_range;
   }
}

impl TEval for RatQuadPolyPath {
   #[allow(clippy::suboptimal_flops)]
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let homog = RatQuadHomogPower::from(self);
      q_reduce(&rat_quad_power_eval(&homog.h.0, &rat_quad_expand_power(t)))
   }
}

#[test]
#[allow(clippy::unreadable_literal)]
fn poly_eval_no_bilinear_test() {
   let poly = RatQuadPolyPath {
      r: [-6.0, 14.0],
      a: [689.0243700979973, -9.204745830513218, 1.1505932288141527],
      b: [-865.8010653946342, -49.720819268365744, -6.061056987054066],
      c: [760.1100286299702, 114.48112065443453, -2.033980686204532],
   };
   let homog = RatQuadHomogPower::from(&poly);

   let homog_expect = RatQuadHomogPower {
      r: [-6.0, 14.0],
      h: RatQuadHomog([
         [-865.8010653946342, -49.720819268365744, -6.061056987054066],
         [760.1100286299702, 114.48112065443453, -2.033980686204532],
         [689.0243700979973, -9.204745830513218, 1.1505932288141527],
      ]),
   };

   assert_abs_diff_eq!(
      &RatQuadHomogWrapped::from(&homog.h),
      &RatQuadHomogWrapped::from(&homog_expect.h),
      epsilon = 1.0e-5
   );
}

impl Curve<RatQuadPolyPath> {
   #[inline]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   fn eval_part_quad(b: f64, a: f64, coeffs: &[f64; 3]) -> f64 {
      b * b * coeffs[0] + b * a * coeffs[1] + a * a * coeffs[2]
   }

   // Internal bilinear transform.
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   fn rq_apply_bilinear_unranged(&self, w: f64, x: f64, y: f64, z: f64) -> Self {
      // let norm = 1.0 / ((w * w + x * x) * (y * y + z * z)).sqrt().sqrt();
      // w *= norm;
      // x *= norm;
      // y *= norm;
      // z *= norm;

      let input_path = RatQuadHomogPower::from(&self.path);

      let tran_q_mat: QMat =
         [[z * z, 2.0 * y * z, y * y], [x * z, x * y + w * z, w * y], [x * x, 2.0 * w * x, w * w]];
      let output_homog = input_path.h.apply_q_mat(&tran_q_mat);

      let mut homog_path = RatQuadHomogPower { h: output_homog, r: self.path.r };
      homog_path.h.normalize();

      Self { path: RatQuadPolyPath::from(&homog_path), sigma: self.sigma }
   }

   // Internal bilinear transform.
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   pub fn rq_apply_bilinear(&self, sigma_ratio: (f64, f64)) -> Self {
      let sigma_n = sigma_ratio.0;
      let sigma_d = sigma_ratio.1;
      let p = -self.path.r[0];
      let q = self.path.r[1];

      self.rq_apply_bilinear_unranged(
         sigma_n * q + sigma_d * p,
         (sigma_n - sigma_d) * p * q,
         sigma_n - sigma_d,
         sigma_d * q + sigma_n * p,
      )
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

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::many_single_char_names)]
   // Weighted is stored in same struct as `Curve<RatQuadPolyPath>`, but is really a different
   // representation of a rational quadratic.
   pub fn create_from_weighted(weighted: &Self) -> Result<Self, &'static str> {
      // Get from rat_poly.sigma once confirmed working.
      let sigma = 1.0;
      let v = weighted.path.r[0];
      let w = weighted.path.r[1];
      let a;
      let b;
      let c;
      {
         let h0 = weighted.path.a[0];
         let h1 = 0.5 * sigma * weighted.path.a[1];
         let h2 = sigma * sigma * weighted.path.a[2];
         a = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = weighted.path.b[0];
         let h1 = 0.5 * sigma * weighted.path.b[1];
         let h2 = sigma * sigma * weighted.path.b[2];
         b = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = weighted.path.c[0];
         let h1 = 0.5 * sigma * weighted.path.c[1];
         let h2 = sigma * sigma * weighted.path.c[2];
         c = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }

      Ok(Self { path: RatQuadPolyPath { r: weighted.path.r, a, b, c }, sigma: weighted.sigma })
   }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   pub fn eval_homog(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let homog_path = RatQuadHomogWeighted::from(&RatQuadHomogPower::from(&self.path));
      // let homog_path = RatQuadHomogPower::from(&self.path);
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let p = self.sigma.0 * (*item - self.path.r[0]);
         let q = self.sigma.1 * (self.path.r[1] - *item);
         let b = &homog_path.h.0[0];
         let c = &homog_path.h.0[1];
         let a = &homog_path.h.0[2];
         let rb = Self::eval_part_quad(q, p, b);
         let rc = Self::eval_part_quad(q, p, c);
         let ra = Self::eval_part_quad(q, p, a);
         let div_factor = 1.0 / ra;
         ret_val.push([rb * div_factor, rc * div_factor]);
      }
      ret_val
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   pub fn eval_derivative_scaled(&self, t: &[f64], scale: f64) -> Vec<[f64; 2]> {
      let homog_path = RatQuadHomogWeighted::from(&RatQuadHomogPower::from(&self.path));
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      // dbg!(self.sigma);
      for item in t {
         let p = self.sigma.0 * (*item - self.path.r[0]);
         let q = self.sigma.1 * (self.path.r[1] - *item);
         let b = &homog_path.h.0[0];
         let c = &homog_path.h.0[1];
         let a = &homog_path.h.0[2];
         let expansion_b = [
            a[0] * b[1] - a[1] * b[0],
            2.0 * (a[0] * b[2] - a[2] * b[0]),
            a[1] * b[2] - a[2] * b[1],
         ];
         let expansion_c = [
            a[0] * c[1] - a[1] * c[0],
            2.0 * (a[0] * c[2] - a[2] * c[0]),
            a[1] * c[2] - a[2] * c[1],
         ];
         let rb = Self::eval_part_quad(q, p, &expansion_b);
         let rc = Self::eval_part_quad(q, p, &expansion_c);
         let ra = Self::eval_part_quad(q, p, a);
         let w_minus_v = self.path.r[1] - self.path.r[0];
         let div_factor = self.sigma.0 * self.sigma.1 * w_minus_v * scale / ra / ra;
         // Note that deriv of sigma tran converted cubic's
         // let recip_denom = scale * f0 * f0 / w_minus_v;
         // to
         // let recip_denom = scale * f0 * f0 * f0 * f0 * w_minus_v * self.sigma.0 * self.sigma.1;
         ret_val.push([rb * div_factor, rc * div_factor]);
      }
      ret_val
   }
}

#[test]
#[allow(clippy::unreadable_literal)]
fn create_from_weighted_test() {
   let weighted = Curve {
      path: RatQuadPolyPath {
         r: [-6.0, 14.0],
         a: [1.9641855032959659, 2.0 * 1.388888888888889, 1.9641855032959659],
         b: [-2.946278254943949, 0.0, -3.9283710065919317],
         c: [-2.946278254943949, 2.0 * 0.6944444444444453, 3.9283710065919317],
      },
      sigma: (0.0, 0.0),
   };
   let weighted_really = RatQuadHomogWeighted::from(&weighted.path);

   let powered = RatQuadHomogPower::from(
      &Curve::<RatQuadPolyPath>::create_from_weighted(&weighted).unwrap().path,
   );
   let expected_path = RatQuadHomogPower::from(&RatQuadPolyPath {
      r: [-6.0, 14.0],
      a: [689.0243700979975, -9.204745830513225, 1.1505932288141536],
      b: [-718.8918942063235, 35.35533905932739, -6.874649261535881],
      c: [-319.38251506503764, 140.7473543286449, -0.40679613724090746],
   });
   assert_abs_diff_eq!(
      &RatQuadHomogWrapped::from(&powered.h),
      &RatQuadHomogWrapped::from(&expected_path.h),
      epsilon = 1.0e-5
   );

   // Also test against reference version.
   let reference_power = RatQuadHomogPower::from(&weighted_really);
   assert_abs_diff_eq!(
      &RatQuadHomogWrapped::from(&powered.h),
      &RatQuadHomogWrapped::from(&reference_power.h),
      epsilon = 1.0e-5
   );
}

impl CurveEval for Curve<HyperbolicPath> {
   #[allow(clippy::suboptimal_flops)]
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      self.path.eval_no_bilinear(t)
   }

   fn eval_with_bilinear(&self, _t: &[f64]) -> Vec<[f64; 2]> {
      unimplemented!("It takes time.");
   }

   fn characterize_endpoints(&self) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
      todo!();
   }
}

impl CurveEval for Curve<RatQuadPolyPath> {
   // This method may lack tests.
   #[allow(clippy::suboptimal_flops)]
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      self.path.eval_no_bilinear(t)
   }

   fn eval_with_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      // let scratchy_rat_poly = self.rq_apply_bilinear((2.0_f64.sqrt(), 1.0));
      let scratchy_rat_poly = self.rq_apply_bilinear(self.sigma);
      scratchy_rat_poly.path.eval_no_bilinear(t)
   }

   #[inline]
   #[allow(clippy::suboptimal_flops)]
   fn characterize_endpoints(&self) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
      let mut x = [0.0; 4];
      let mut y = [0.0; 4];

      let speed_scale = self.path.r[1] - self.path.r[0];
      for (outer, inner, t) in [(0, 1, self.path.r[0]), (3, 2, self.path.r[1])] {
         let recip_a = 1.0 / ((self.path.a[2] * t + self.path.a[1]) * t + self.path.a[0]);
         let b = (self.path.b[2] * t + self.path.b[1]) * t + self.path.b[0];
         let c = (self.path.c[2] * t + self.path.c[1]) * t + self.path.c[0];
         let da = (self.path.a[2] * 2.0 * t + self.path.a[1]) * speed_scale;
         let db = (self.path.b[2] * 2.0 * t + self.path.b[1]) * speed_scale;
         let dc = (self.path.c[2] * 2.0 * t + self.path.c[1]) * speed_scale;
         x[outer] = b * recip_a;
         y[outer] = c * recip_a;
         x[inner] = (-b * da).mul_add(recip_a, db) * recip_a;
         y[inner] = ((-c * da) * recip_a + dc) * recip_a;
      }
      ([[x[0], y[0]], [x[3], y[3]]], [[x[1], y[1]], [x[2], y[2]]])
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
      let (ends, deltas) = Into::<Curve<RatQuadPolyPath>>::into(self).characterize_endpoints();
      let f = 1.0 / 3.0;
      let four_c = [
         [ends[0][0], ends[0][0] + f * deltas[0][0], ends[1][0] - f * deltas[1][0], ends[1][0]],
         [ends[0][1], ends[0][1] + f * deltas[0][1], ends[1][1] - f * deltas[1][1], ends[1][1]],
      ];
      // let four_c = [
      //    [ends[0][0],ends[0][1]],
      //    [ends[0][0] + f * deltas[0][0], ends[0][1] + f * deltas[0][1]],
      //    [ends[1][0] - f * deltas[1][0], ends[1][1] - f * deltas[1][1]],
      //    [ends[1][0],ends[1][1]],
      // ];

      // assert_eq!(self.range_bound, 0.0);
      Curve::<CubicPath> {
         path: CubicPath {
            r: [-self.path.range_bound, self.path.range_bound],
            h: CubicHomog(four_c),
         },
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
            range: (-self.path.range_bound, self.path.range_bound),
            lambda,
            mu,
            offset,
            plus_partial,
            minus_partial,
         },
         sigma: self.sigma,
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::unnecessary_wraps)]
   #[allow(clippy::missing_errors_doc)]
   fn create_by_raising_to_regularized_symmetric(
      rat_poly_extracted: &Curve<RatQuadPolyPath>,
   ) -> Result<Self, &'static str> {
      let rat_poly = rat_poly_extracted.figure_symmetric_range_rat_quad();

      let r_both = rat_poly.path.r[1];
      let a_s = rat_poly.path.a[2] * r_both * r_both + rat_poly.path.a[0];
      // let a_d = rat_poly.path.a[2] * r * r - rat_poly.path.a[0];
      let combo_s = a_s + rat_poly.path.a[1] * r_both;
      let combo_d = a_s - rat_poly.path.a[1] * r_both;

      let sigma_ratio = (combo_d.abs().sqrt(), combo_s.abs().sqrt());

      let scratchy_rat_poly = rat_poly.rq_apply_bilinear(sigma_ratio);

      let check_poly = scratchy_rat_poly;
      assert!(check_poly.path.a[1].abs() < 0.001);
      Ok(Self {
         path: RegularizedRatQuadPath {
            range_bound: check_poly.path.r[1],
            a_0: check_poly.path.a[0],
            a_2: check_poly.path.a[2],
            b: check_poly.path.b,
            c: check_poly.path.c,
         },
         sigma: check_poly.sigma,
      })
   }
}

#[allow(clippy::suboptimal_flops)]
impl RatQuadOoeSubclassed {
   fn create_elliptical_or_parabolic(
      poly_curve: &Curve<RatQuadPolyPath>,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let reg_curve =
         Curve::<RegularizedRatQuadPath>::create_by_raising_to_regularized_symmetric(poly_curve)?;

      // fn create_from_regularized(reg_curve: &Curve<RegularizedRatQuadPath>, tolerance: f64) -> Self {
      let mut rat_poly = reg_curve.clone();
      let orig_rat_poly = reg_curve;

      let r = rat_poly.path.range_bound;
      if (rat_poly.path.a_2.abs() * r * r) < (rat_poly.path.a_0.abs() * tolerance) {
         Ok(Self::Parabolic(orig_rat_poly.convert_to_parabolic()))
      } else {
         // Rust clippy effectively makes this check impossible.
         // assert_eq!(rat_poly.path.a_2.signum(), rat_poly.path.a_0.signum());

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

            Ok(Self::Parabolic(orig_rat_poly.convert_to_parabolic()))
         } else {
            // Only outcome that actually uses OOE form.
            Ok(Self::Elliptical(rat_poly))
         }
      }
   }

   fn create_hyperbolic_or_parabolic(
      poly_curve: &Curve<RatQuadPolyPath>,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let reg_curve =
         Curve::<RegularizedRatQuadPath>::create_by_raising_to_regularized_symmetric(poly_curve)?;

      // fn create_from_regularized(reg_curve: &Curve<RegularizedRatQuadPath>, tolerance: f64) -> Self {
      let rat_poly = reg_curve;

      let r = rat_poly.path.range_bound;
      if (rat_poly.path.a_2.abs() * r * r) < (rat_poly.path.a_0.abs() * tolerance) {
         Ok(Self::Parabolic(rat_poly.convert_to_parabolic()))
      } else {
         // Rust clippy effectively makes this check impossible.
         // assert_ne!(rat_poly.path.a_2.signum(), rat_poly.path.a_0.signum());

         let hyperbolic_form = rat_poly.convert_to_hyperbolic();

         Ok(Self::Hyperbolic(hyperbolic_form))
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn create_from_ordinary(
      poly_curve: &Curve<RatQuadPolyPath>,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      // First test "b^2-4ac" to see if denominator has real roots. If it does, create either
      // hyperbolic or parabolic. If no real roots, then elliptical or parabolic.
      if (poly_curve.path.a[1] * poly_curve.path.a[1])
         < (4.0 * poly_curve.path.a[0] * poly_curve.path.a[2])
      {
         Self::create_elliptical_or_parabolic(poly_curve, tolerance)
      } else {
         Self::create_hyperbolic_or_parabolic(poly_curve, tolerance)
      }
   }
}
