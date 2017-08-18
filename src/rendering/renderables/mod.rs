pub mod bezier_rect;
pub mod bezier_subrect;
pub mod circle;
pub mod polar_pixel;
pub mod polygon;
pub mod rectangle;
pub mod text;
use super::primitives::Primitive;

pub trait Renderable {
    fn get_primitives(&mut self) -> Vec<Primitive>;
}