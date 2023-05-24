// Copyright 2023 Google LLC
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

pub mod jaywalk_foundation;
pub mod jaywalk_traiting;

use crate::jaywalk_graph::jaywalk_traiting::absent_f64;
use crate::jaywalk_graph::jaywalk_traiting::absent_int32;
use crate::jaywalk_graph::jaywalk_traiting::add_ident_f64;
use crate::jaywalk_graph::jaywalk_traiting::mult_ident_f64;
use crate::jaywalk_graph::jaywalk_traiting::Anchorage;
use crate::jaywalk_graph::jaywalk_traiting::ArrowType;
use crate::jaywalk_graph::jaywalk_traiting::Bidirection;
use crate::jaywalk_graph::jaywalk_traiting::Coord;
use crate::jaywalk_graph::jaywalk_traiting::Finish;
use crate::jaywalk_graph::jaywalk_traiting::JKey;
use crate::jaywalk_graph::jaywalk_traiting::JVec;
use crate::jaywalk_graph::jaywalk_traiting::JaywalkAffine;
use crate::jaywalk_graph::jaywalk_traiting::LineStyle;
use crate::jaywalk_graph::jaywalk_traiting::Shape;
use crate::jaywalk_graph::jaywalk_traiting::StateMark;
use crate::jaywalk_graph::jaywalk_traiting::TMatrix;
use crate::jaywalk_graph::jaywalk_traiting::Yna;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use std::collections::HashMap;

// Fields involving keys are generally not (de-)serialized, at least
// for now.  If we later serialize we should renumber keys based on
// primary and obverse ranks so that there is a canonical keying
// suitable for diffing.

// Base nodes and edges are in two DAGs. Defaults are provided. In
// each case the defaults can be overwritten. The customized versions
// should themselves be based on their corresponding root.
//
// Predefined pseudo-nodes:
//
// default, medium_root.
// bold, bold_root.
// light, light_root.
// waypoint, waypoint_root.
// source, source_root.
// sink, sink_root.
// "ghost" - TBD.
// origin, origin_root. For axes origin.
//
// Predefined pseudo-edges:
//
// default, default_root.
// major_grid, major_grid_root.
// minor_grid, minor_grid_root.

// Nodes for trimming should always be on the outside. In future
// versions we should enforce this.

// In the long run, especially if we add colours, line caps, line
// styles to arrow heads, and generalize a bit more, the derivation
// process will make the structure a bit big. At that point we should
// make a special version of Option<T> called Lazy<T> and fill in the
// optional value lazily, or have Derived<T> with a way of just
// traversing until finding a non-optional in the derivation chain.

// The tricky design decision is in how, say to handle the up-down vs
// sideways treatment of affine values. For example, if a derived
// entity changes the line width but automatically derives the
// pattern, the pattern lengths are calculated sideways from the line
// width, and not drawn unchanged from the base entity, even though
// the pattern was defined there.

// Everything should be based around a "minimal" scale. For example,
// pattern segments are defined as affines on the line (pen
// width). First figure out the pattern desired for a minimally thin
// line. Then figure out how fast to scale. Or consider the other way
// around. First figure out the pattern for a really thick line with
// rounded caps, and then make sure that the pattern does not shrink
// quite so fast when the line is reduced to minimal thickness. An
// example is a dotted line. If the thin line's dots are too close, it
// might look just gray instead of a pattern.

// Phantom nodes early on are transformed into pseudo-nodes and their
// ranks are added to the lists of phantom ranks.

// Fields involving keys are generally not (de-)serialized, at least
// for now.  If we later serialize we should renumber keys based on
// primary and obverse ranks so that there is a canonical keying
// suitable for diffing.

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct Node {
   // None of the following are derived from a base.
   #[serde(default = "absent_int32")]
   pub primary: i32, // Rank.
   #[serde(default = "absent_int32")]
   pub obverse: i32,
   #[serde(default = "absent_int32")]
   pub orig_primary: i32,
   #[serde(default = "absent_int32")]
   pub orig_obverse: i32,
   /// Quasi-logical location. In simplest layout (primary, obverse)
   /// ranks converted to floating-point. Adjusted by compaction.
   #[serde(default)]
   pub logical_location: Coord,
   /// Location in pre-transformed layout.
   #[serde(default)]
   pub pre_trans_location: Coord,
   /// Location in final layout.
   #[serde(default)]
   pub location: Coord,

   /// The name is more of an ID.
   ///
   /// If not specified, the name is a simplification of the text
   /// field. Spaces and basic special characters are replaced by
   /// '_'. Accented characters (in release version) will be replaced
   /// by unaccented.
   #[serde(default)]
   pub name: String,
   /// Text can have special fields.
   ///
   /// %%, %p, %o, %n: Replaced by '%', (original) primary rank,
   /// (original) obverse rank, name. These are replaced after
   /// resolution of derivation.
   ///
   /// %+, %-: These are handled during derivation. If base has text
   /// that contains "%+", this is replaced with any text in derived
   /// node. If "%-", the derived text is simply dropped. This can be
   /// used for diagnostics.
   #[serde(default)]
   pub text: String,

   #[serde(default)]
   pub base: String,
   /// Not normally set directly. Pseudo-nodes start out in a
   /// separate set, and so this field effectively indicates the
   /// initial membership.
   #[serde(skip)]
   pub pseudo: bool,
   /// A phantom node is one that is not interconnected, but its
   /// primary and obverse ranks leave gaps in the layout.
   #[serde(default)]
   pub phantom: bool,

   #[serde(skip)]
   pub key: JKey,

   #[serde(skip_deserializing)]
   pub child_keys: Vec<JKey>,
   #[serde(skip_deserializing)]
   pub parent_keys: Vec<JKey>,

   // The following derive from base
   /// After derivation this should not be auto.
   #[serde(default)]
   pub show: Yna,
   /// After derivation this should not be auto.
   ///
   /// A trimmed node is constructed as part of the graph, but
   /// trimmed completely so that the layout shrinks.  This is
   /// typically used for extrapolated sources and sinks.
   #[serde(default)]
   pub trim: Yna,
   /// Labels should not be shown if the node is not shown. If auto,
   /// show_label := (show AND NOT trim).
   #[serde(default)]
   pub show_label: Yna,
   /// Outs and ins (edges) are not normally shown if node is
   /// trimmed. The auto setting reflects this.
   ///
   /// If a node is hidden, the in and out edges are not
   /// automatically hidden.
   #[serde(default)]
   pub show_outs: Yna,
   #[serde(default)]
   pub show_ins: Yna,

   #[serde(skip)]
   pub commissioning: StateMark,
   /// Some pseudo nodes and edges should not normally be
   /// overridden. Ordinary doing so produces an error.
   #[serde(default)]
   pub dislike_override: bool,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct Edge {
   // Parent and child nodes are by primary rank. This does not work
   // with lexicographic sort. In general it is better, for input
   // graph, to use names.
   #[serde(default = "absent_int32")]
   pub parent: i32,
   #[serde(default = "absent_int32")]
   pub child: i32,
   // Alternatives.
   #[serde(default)]
   pub parent_name: String,
   #[serde(default)]
   pub child_name: String,

   /// See node name for description of simplification of text. The
   /// edge name is mostly used for creating base edges.
   #[serde(default)]
   pub name: String,
   /// Text can have special fields.
   ///
   /// %%, %n: Replaced by, '%', name. These are replaced after
   /// resolution of derivation.
   ///
   /// %+, %-: These are handled during derivation. If base has text
   /// that contains "%+", this is replaced with any text in derived
   /// node. If "%-", the derived text is simply dropped. This can be
   /// used for diagnostics.
   #[serde(default)]
   pub text: String,

   /// End arrow direction.
   ///
   /// Should be resolved during derivation, and not auto thereafter.
   #[serde(default)]
   pub direction: Bidirection,
   /// Mid-edge arrow direction.
   ///
   /// Should be resolved during derivation, and not auto thereafter.
   #[serde(default)]
   pub mid_direction: Bidirection,

   #[serde(default)]
   pub base: String,
   /// Not normally set directly. Pseudo-nodes start out in a
   /// separate set, and so this field effectively indicates the
   /// initial membership.
   #[serde(skip)]
   pub pseudo: bool,
   // #[serde(default)]
   // pub phantom: bool,
   #[serde(skip)]
   pub key: JKey,
   #[serde(skip)]
   pub parent_key: JKey,
   #[serde(skip)]
   pub child_key: JKey,

   #[serde(default)]
   pub show: Yna,
   #[serde(default)]
   pub show_label: Yna,

   #[serde(skip)]
   pub commissioning: StateMark,
   /// Some pseudo nodes and edges should not normally be
   /// overridden. Ordinary doing so produces an error.
   #[serde(default)]
   pub dislike_override: bool,
}

// TODO: Subtype keys and keyed vectors.

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct JaywalkGraph {
   pub nodes: JVec<Node>,
   #[serde(default)]
   pub edges: JVec<Edge>,
   /// Pseudo-nodes and pseudo-edges are defined in separate sets in
   /// the input graph but are merged into the main sets with a field
   /// marking their pseudo-ness. This is so that there is one set of
   /// keys for nodes and one for edges.
   #[serde(default)]
   pub pseudo_nodes: Vec<Node>,
   #[serde(default)]
   pub pseudo_edges: Vec<Edge>,

   #[serde(skip)]
   pub node_names_to_keys: HashMap<String, JKey>,
   #[serde(skip)]
   pub edges_names_to_keys: HashMap<String, JKey>,

   /// Some ranks are phantoms. These create a kind of padding that
   /// is retained even when the graph is "renumbered" when (a) rank
   /// sparsity is eliminated, and (b) duplicate ranks are
   /// lexicographically split.
   ///
   /// Phantoms are different from trimmed nodes. It is an error to
   /// have trimmed nodes with ranks between non-trimmed nodes and
   /// phantom ranks.
   #[serde(default)]
   pub phantom_primary: Vec<JKey>, // RANKS?
   #[serde(default)]
   pub phantom_obverse: Vec<JKey>,

   #[serde(skip)]
   pub primary_to_key: HashMap<i32, JKey>,
   #[serde(skip)]
   pub obverse_to_key: HashMap<i32, JKey>,

   #[serde(skip)]
   pub active_primary_keys: Vec<JKey>,
   /// There is a set of active nodes, and so the size of the
   /// active_obverse_keys vector matches that of the
   /// active_primary_keys.
   ///
   /// The obverse keys are ordered. The order may be ascending or
   /// descending. This is done so that the extrapolated source and
   /// sink are at corresponding ends. Even if these nodes are not
   /// required, the order is chosen to match. For example, if a
   /// codominance graph is requested, the original obverse ranks are
   /// first flipped and the edges constructed accordingly. When they
   /// are flipped back, the active_obverse_keys vector is not
   /// reversed.
   #[serde(skip)]
   pub active_obverse_keys: Vec<JKey>,
   #[serde(skip)]
   pub trimmed_active_first: i32, // Index into active_primary_keys.
   #[serde(skip)]
   pub trimmed_active_after_last: i32, // Index into / just after active_primary_keys.
   #[serde(skip)]
   pub active_edge_keys: Vec<JKey>, // May have finalized order in future.
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, Default)]
pub struct RenderFont {
   #[serde(default)]
   pub font_family: String,
   #[serde(default)]
   pub font_size: JaywalkAffine, // Affine calculation or maybe pure optional scaling.
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct RenderNode {
   #[serde(default)]
   pub name: String,

   #[serde(default)]
   pub shape: Shape,
   #[serde(default = "absent_f64")]
   pub radius: f64,
   #[serde(default = "absent_f64")]
   pub width: f64,
   #[serde(default = "absent_f64")]
   pub height: f64,

   #[serde(default)]
   pub derive_width: String,
   #[serde(default)]
   pub derive_height: String,

   // Could also have label radius based on line width, or on font
   // size. But separation also can be controlled via margins.
   #[serde(default)]
   pub label_radius: JaywalkAffine, // Affine calculation based on resolved node radius.
   #[serde(default)]
   pub label_anchor: Anchorage,
   #[serde(default)]
   pub label_direction: Anchorage, // Often found from anchor (180 degrees offset).

   #[serde(default)]
   pub finish: Finish,
   #[serde(default)]
   pub line_style: LineStyle,

   #[serde(default)]
   pub font: RenderFont,

   // BASE???
   #[serde(skip)]
   pub key: JKey,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, Default)]
pub struct RenderArrow {
   #[serde(default)]
   pub finish: Finish,
   #[serde(default)]
   pub arrow_type: ArrowType,
   /// The length is not literally the length, but rather the length
   /// and width scale a size-1 coordinate system on which the arrow
   /// is designed.
   #[serde(default)]
   pub length: JaywalkAffine,
   /// The width is based off, and defaults equal to, the
   /// length.
   ///
   /// See note on the length.
   #[serde(default)]
   pub width: JaywalkAffine, // Affine calculation based (?) off length?
   /// Inner Bézier point in from tip, relative to (0,0)-(1,1) box.
   /// If (0.0, 0.0) pulls from base.
   #[serde(default)]
   pub curly_tip_point: Coord, // For base value try (0.75, 0.25).
   /// Inner Bézier point in from side corner, relative to
   /// (0,0)-(1,1) box.  If (0.0, 0.0) pulls from base.
   #[serde(default)]
   pub curly_side_point: Coord, // For base value try (0.75, 0.25).
   /// Affine calculation based off arrow length. For multi-arrow
   /// heads and edge adjustment.
   #[serde(default)]
   pub tip_to_anchor: JaywalkAffine,
   /// Vector of arrow renderings for arrows after the first.
   /// Recursive (nested) use is an error.
   #[serde(default)]
   pub multi_render: Vec<RenderArrow>,
   /// Offset of arrow location based off of edge length. Arrows are
   /// normally set at the end or mid-point of the edge, with adjustment according to the
   #[serde(default)]
   pub edge_position: JaywalkAffine,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, Default)]
pub struct RenderEdge {
   #[serde(default)]
   pub name: String,

   #[serde(default = "absent_f64")]
   pub phantom_primary: f64,

   #[serde(default)]
   pub line_width: JaywalkAffine,
   #[serde(default)]
   pub pattern_sep: JaywalkAffine, // Affine calculation based off line width?

   // Get base via key? We want user to be able to specify directly in RenderEdge.
   #[serde(default)]
   pub base_node: String,
   #[serde(skip)]
   pub key: JKey,

   #[serde(default)]
   pub arrow: RenderArrow,
   #[serde(default)]
   pub backwards_arrow: Option<RenderArrow>,

   #[serde(default)]
   pub font: RenderFont,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct LayoutTransformation {
   // #[serde(default)]
   // pub transform_type: TransformType,

   // The target direction, to which the original NE will be rotated.
   #[serde(default)]
   pub rotation: Anchorage,
   // Additional rotation
   #[serde(default = "mult_ident_f64")]
   pub alpha: f64,
   #[serde(default = "add_ident_f64")]
   pub beta: f64,
   // #[serde(default = "add_ident_f64")]
   // pub gamma: f64,
   // #[serde(default = "add_ident_f64")]
   // pub delta: f64,
   #[serde(default = "add_ident_f64")]
   pub n_stretch: f64,

   #[serde(default)]
   pub trans_matrix: TMatrix,
   #[serde(default)]
   pub combined_transformation: TMatrix,
}

impl Default for LayoutTransformation {
   fn default() -> Self {
      LayoutTransformation {
         rotation: Anchorage::default(),
         alpha: mult_ident_f64(),
         beta: add_ident_f64(),
         // gamma: add_ident_f64(),
         // delta: add_ident_f64(),
         n_stretch: add_ident_f64(),
         trans_matrix: TMatrix::default(),
         combined_transformation: TMatrix::default(),
      }
   }
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct JaywalkRender {
   // The layout of, and keys into, these vectors must match the
   // vectors in the main (non-render) graph.
   #[serde(default)]
   pub nodes: JVec<RenderNode>,
   #[serde(default)]
   pub edges: JVec<RenderEdge>,
   //   optional LayoutDirection direction = 1;
   //   // The base grid size is given by the separation of nodes, measured in points.
   //   optional double sep_points =
   //       2;  // Zero will be overridden with program default.
   //   // The source and sink displays are applied iff imputed.
   //   optional double base_margin = 8;
   //   optional double octant_rotation = 9;  // Additional rotation in octants.
   #[serde(default = "mult_ident_f64")]
   pub scale_overall: f64,
   #[serde(default)]
   pub scale_geometry: JaywalkAffine, // Affine calculation based off scale_overall.
   #[serde(default)]
   pub scale_pen: JaywalkAffine, // Affine calculation based off scale_overall.
   #[serde(default)]
   pub scale_fonts: JaywalkAffine, // Affine calculation based off scale_overall.

   #[serde(default)]
   pub node_grid: JaywalkAffine, // Affine calculation based off scale_overall.
   #[serde(default)]
   pub margin: JaywalkAffine, // Affine calculation based off scale_overall.

   #[serde(default)]
   pub transform: LayoutTransformation,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct ZebraixGraph<'a> {
   #[serde(default)]
   pub aux_name: &'a str,

   pub graph: JaywalkGraph,
   pub render: JaywalkRender,
}

// ==================================

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Zebraix2DPoint {
   #[serde(default = "add_ident_f64")]
   pub x: f64,
   #[serde(default = "add_ident_f64")]
   pub y: f64,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Zebraix2DMultPoint {
   #[serde(default = "mult_ident_f64")]
   pub x: f64,
   #[serde(default = "mult_ident_f64")]
   pub y: f64,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Zebraix2DMat {
   #[serde(default = "mult_ident_f64")]
   pub a: f64,
   #[serde(default = "add_ident_f64")]
   pub b: f64,
   #[serde(default = "add_ident_f64")]
   pub c: f64,
   #[serde(default = "mult_ident_f64")]
   pub d: f64,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Zebraix2DAffine {
   #[serde(default)]
   pub scale: Zebraix2DMat,
   #[serde(default)]
   pub offset: Zebraix2DPoint,
}

// Affine transform with additional matrix for, say, margin
// contribution.
// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Zebraix2DDualAffine {
   #[serde(default)]
   pub base: Zebraix2DAffine,
   #[serde(default)]
   pub aux: Zebraix2DMat,
}

// Affine transform defined as sequence of points / value pairs.
// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Zebraix2DSeqAffine {
   #[serde(default)]
   pub pairs: Vec<Zebraix2DPoint>,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Zebraix2DNamedAffine {
   #[serde(default)]
   pub name: String,
   #[serde(default)]
   pub seq: Zebraix2DSeqAffine,
   #[serde(skip, default)]
   pub index: i32,
   #[serde(skip, default)]
   pub dual: Zebraix2DDualAffine,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct ZebraixNameIndex {
   #[serde(default)]
   pub name: String,
   #[serde(skip, default)]
   pub index: i32,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct ZebraixStraight {
   #[serde(default)]
   pub point: [ZebraixNameIndex; 2],
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct ZebraixCubic {
   #[serde(default)]
   pub point: [ZebraixNameIndex; 4],
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct ZebraixRatQuadPlain {
   #[serde(default)]
   pub point: [ZebraixNameIndex; 4],
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct ZebraixRatQuadCorner {
   #[serde(default)]
   pub point: [ZebraixNameIndex; 4],
   #[serde(default = "add_ident_f64")]
   pub corner_cosine: f64,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum ZebraixPolySegment {
   Straight(ZebraixStraight),
   Cubic(ZebraixCubic),
   RatQuadPlain(ZebraixRatQuadPlain),
   RatQuadCorner(ZebraixRatQuadCorner),
}

impl Default for ZebraixPolySegment {
   fn default() -> Self {
      ZebraixPolySegment::Straight(<ZebraixStraight as Default>::default())
   }
}

fn empty_vec() -> &'static Vec<Zebraix2DNamedAffine> {
   static EMPTY_VEC: Vec<Zebraix2DNamedAffine> = Vec::<Zebraix2DNamedAffine>::new();
   &EMPTY_VEC
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum PolyLineForm {
   Open = 0,
   Closed,
}

impl Default for PolyLineForm {
   fn default() -> Self {
      PolyLineForm::Open
   }
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct ZebraixPolySpline<'a> {
   #[serde(skip, default = "empty_vec")]
   pub named_affines: &'a Vec<Zebraix2DNamedAffine>,
   #[serde(default)]
   pub segments: Vec<ZebraixPolySegment>,
   #[serde(default)]
   pub form: PolyLineForm,
}
