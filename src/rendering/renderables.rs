pub use super::rectangle::Rectangle;
pub use super::circle::Circle;
pub use super::polar_pixel::PolarPixel;
pub use super::text::PlainText;
pub use super::bezier_rect::BezierRect;
pub use super::bezier_subrect::BezierSubrect;
pub use super::polygon::Polygon;

pub trait Renderable {
    fn get_type(&self) -> RenderType;
}

#[derive(Clone)]
pub enum RenderType {
    Rect(Rectangle),
    Circ(Circle),
    Poly(Polygon),
    PolarPix(PolarPixel),
    Text(PlainText),
    BezierRect(BezierRect),
    BezierSubrect(BezierSubrect),
}
