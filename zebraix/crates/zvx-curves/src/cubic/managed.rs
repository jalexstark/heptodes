// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::CurveTransform;
use serde::Serialize;
use serde_default::DefaultFromSerde;
use zvx_base::CubicHomog;
use zvx_base::{CubicFourPoint, CubicPath};

// // As yet not used.
// #[derive(Debug, Deserialize, Serialize, DefaultFromSerde, PartialEq, Clone)]
// pub struct MidDiffCubiLinearRepr {
//    pub r: [f64; 2], // Range.
//    pub x: [f64; 4],
//    pub y: [f64; 4],
//    #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
//    pub sigma: f64,
// }

// Recreate as specified Cubic or SpecifiedCubiLinear when reworking managed curves.
//
// #[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
// pub enum SpecifiedCubic {
//    #[default]
//    Nothing,
//    FourPoint(Curve<CubicPath>),
//    MidDiff(MidDiffCubiLinearRepr),
// }

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedCubic {
   pub four_point: CubicPath,
   // How originally specified, FourPoint or MidDiff, for plotting and diagnostics only.
   // pub specified: SpecifiedCubic,
   pub canvas_range: [f64; 4],
}

#[allow(clippy::missing_panics_doc)]
impl ManagedCubic {
   #[must_use]
   pub fn create_from_control_points(
      control_points: &CubicFourPoint,
      canvas_range: [f64; 4],
   ) -> Self {
      Self {
         four_point: CubicPath {
            r: control_points.r,
            h: CubicHomog([
               [
                  control_points.h.0[0][0],
                  3.0 * control_points.h.0[0][1],
                  3.0 * control_points.h.0[0][2],
                  control_points.h.0[0][3],
               ],
               [
                  control_points.h.0[1][0],
                  3.0 * control_points.h.0[1][1],
                  3.0 * control_points.h.0[1][2],
                  control_points.h.0[1][3],
               ],
            ]),
            sigma: control_points.sigma,
         },
         canvas_range,
      }
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      self.four_point.displace(d);
   }

   pub fn bilinear_transform(&mut self, sigma_ratio: (f64, f64)) {
      self.four_point.bilinear_transform(sigma_ratio);
   }

   pub fn select_range(&mut self, new_range: [f64; 2]) {
      self.four_point.select_range(new_range);
   }
}
