--------------------------------------------------------------------------------

Heptodes documents and other content in `doc` directories are licensed under the
[Creative Commons Attribution 4.0 License](CC BY 4.0 license).

Source code licensed and code samples are licensed under the
[Apache 2.0 License].

The CC BY 4.0 license requires attribution. When samples, examples, figures,
tables, or other excerpts, are used in a tutorial, or a subdivision thereof, it
is sufficient to provide the complete source and license information once. This
must be close to the beginning, such as in an early acknowledgments slide. If
this is done, only short notes are required to be placed with each usage, such
as in figure captions.

[Creative Commons Attribution 4.0 License]: https://creativecommons.org/licenses/by/4.0/legalcode
[Apache 2.0 License]: https://www.apache.org/licenses/LICENSE-2.0

--------------------------------------------------------------------------------

<!-- md-formatter off (Document metadata) -->

---
title: Drawable and SVG Design Doc
author:
- J. Alex Stark
date: 2022
...

<!-- md-formatter on -->

# Task

## General purpose

Not yet incorporated from Cairo:

Context and general:

*   set_miter_limit
*   tolerance
*   clipping / paint
*   pattern fill / source
*   page
*   antialias

Fonts:

*   font_options
*   font_matrix
*   select_font / show_text ("toy" font Cairo)
*   scaled_font (at least not API - may be used for extent calc, etc)
*   font_extents

Path:

*   glyph_path(&self, glyphs: &[Glyph])

To have rough equivalent:

Context:

*   set_source_rgb
*   set_line_width

Font:

*   set_font_size(&self, size: f64)
*   set_font_face(&self, font_face: &FontFace)
*   show_glyphs(&self, glyphs: &[Glyph]) -> Result<(), Error>
*   show_text_glyphs(...) -> Result<(), Error>
*   text_extents(&self, text: &str) -> Result<TextExtents, Error>
*   glyph_extents(&self, glyphs: &[Glyph]) -> Result<TextExtents, Error>

Path:

*   Ellipse arc, ellipse. Most common RQS. arc(&self, xc: f64, yc: f64, radius:
    f64, angle1: f64, angle2: f64)
*   Bézier cubic. curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64,
    y3: f64)
*   Straight line. line_to(&self, x: f64, y: f64)
*   rectangle(&self, x: f64, y: f64, width: f64, height: f64)
*   text_path(&self, str_: &str)

Also:

*   path_extents(&self) -> Result<(f64, f64, f64, f64), Error>

TAGS:

Not as yet supported, but could be enabled for (a) explicit links and
destinations, and (b) for automatic cross-linking in scenarios where the
embedding of the SVG is known, such as a diagram in an auto-converted doc, such
as Salient.

*   tag_begin(&self, tag_name: &str, attributes: &str)
*   tag_end(&self, tag_name: &str)
