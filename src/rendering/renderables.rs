use super::rectangle::Rectangle;
use super::circle::Circle;
use super::text::PlainText;

pub trait Renderable {
    fn get_type(&self) -> RenderType;
}

#[derive(Clone)]
pub enum RenderType {
    Rect(Rectangle),
    Circ(Circle),
    Txt(PlainText)
}
