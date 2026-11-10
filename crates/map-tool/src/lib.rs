//! Flag Plot map tool: generates the hex map of the slice from Natural Earth
//! coastlines (decision 0017), and renders and checks it.
//!
//! This crate uses floating point, unlike `sim-core`: it is an offline tool
//! and the simulation never runs it. To make the output the same on every
//! machine it uses only `+ - * /`, comparisons, `sqrt`, `floor`/`round` and
//! `abs` on `f64` (all exactly specified by IEEE 754): no `sin`, `cos`,
//! `powf` or other libm calls. The cosine of 12 degrees is the literal
//! 0.978148.

pub mod generate;
pub mod land;
pub mod projection;
pub mod render;
pub mod shp;
pub mod water;
