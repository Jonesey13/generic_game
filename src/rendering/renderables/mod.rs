pub mod bezier_rect;
pub mod bezier_subrect;
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
use super::primitives::Primitive;

pub use self::line::Line;
pub use self::arrow::Arrow;
pub use self::circle::Circle;
pub use self::annulus::Annulus;
pub use self::annular_segment::AnnularSegment;
pub use self::box_border::BoxBorder;

pub trait Renderable {
    fn get_primitives(&mut self) -> Vec<Primitive>;
}