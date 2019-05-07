pub mod circle;
pub mod polygon;
pub mod rectangle;
pub mod text;
pub mod line;
pub mod arrow;
pub mod annulus;
pub mod annular_segment;
pub mod box_border;
pub mod texture_rect;
use super::primitives::StandardPrimitive;

pub use self::line::*;
pub use self::arrow::*;
pub use self::circle::*;
pub use self::annulus::Annulus;
pub use self::annular_segment::AnnularSegment;
pub use self::box_border::*;

pub trait Renderable<Prim> {
    fn get_primitives(&mut self) -> Vec<Prim>;
}

pub type StandardRenderable = dyn Renderable<StandardPrimitive>;