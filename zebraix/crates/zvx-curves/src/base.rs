//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use zvx_base::is_default;

// Intended for use directly on paths, rather than those wrapped into Curves.
pub trait TEval {
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]>;
}

// Sigma is a bilinear transformation of `t` that does not change the end-points of the
// curve. Thus conversion to a path does not generally need to involve sigma.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Curve<T: Default + PartialEq> {
   #[serde(skip_serializing_if = "is_default")]
   pub path: T,
}

pub trait CurveEval {
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]>;
   fn eval_with_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]>;

   // Positions of start and finish, and and velocity at end points.
   //
   // The velocity vectors are not adjusted for the span of the range. Divide by that span to
   // get the velocity with respect to the linear range traversal.
   //
   // The velocities are not adjusted for sigma. Multiply the start velocity by sigma and divide
   // the finish velocity by sigma to adjust.
   fn characterize_endpoints(&self) -> ([[f64; 2]; 2], [[f64; 2]; 2]);
}

pub trait CurveTransform {
   fn displace(&mut self, d: [f64; 2]);

   fn bilinear_transform(&mut self, sigma_ratio: (f64, f64));

   // TODO: Probably better style to use tuple for range.
   //
   // Redefine current range as new range, not changing the curve.
   fn raw_change_range(&mut self, new_range: [f64; 2]);

   // Sub- or super-select range, based on current range.
   fn select_range(&mut self, new_range: [f64; 2]);
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
