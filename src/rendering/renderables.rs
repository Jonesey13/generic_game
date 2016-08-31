use super::rectangle::Rectangle;
use super::circle::Circle;

pub trait Renderable {
    fn get_type(&self) -> RenderType;
}

#[derive(Clone)]
pub enum RenderType {
    Rect(Rectangle),
    Circ(Circle),
}
