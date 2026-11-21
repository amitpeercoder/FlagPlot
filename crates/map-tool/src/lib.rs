//! Flag Plot map tool: generates a hex map from Natural Earth coastlines, in
//! the frame a frame file gives or the present map's by default (decisions
//! 0017 and 0199), and renders and checks it.
//!
//! This crate uses floating point, unlike `sim-core`: it is an offline tool
//! and the simulation never runs it. To make the output the same on every
//! machine it uses only `+ - * /`, comparisons, `sqrt`, `floor`/`round` and
//! `abs` on `f64` (all exactly specified by IEEE 754): no `sin`, `cos`,
//! `powf` or other libm calls. A map's factor for longitude is a literal of
//! its frame: 0.978148, the cosine of 12 degrees, for the present map.

pub mod frame;
pub mod generate;
pub mod land;
pub mod projection;
pub mod render;
pub mod shp;
pub mod water;
