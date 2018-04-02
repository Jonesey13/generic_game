pub mod circle;
pub mod polar_pixel;
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

pub use self::line::{Line, LineShape};
pub use self::arrow::Arrow;
pub use self::circle::Circle;
pub use self::annulus::Annulus;
pub use self::annular_segment::AnnularSegment;
pub use self::box_border::BoxBorder;

pub trait Renderable<Prim> {
    fn get_primitives(&mut self) -> Vec<Prim>;
}

pub type StandardRenderable = Renderable<StandardPrimitive>;