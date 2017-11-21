pub mod bezier_rect;
pub mod bezier_subrect;
pub mod circle_part;
pub mod polar_pixel;
pub mod polygon;
pub mod text;
pub mod rectangle;
pub use self::bezier_rect::{BezierRect, BezierQuadControl};
pub use self::bezier_subrect::{BezierSubrect, BezierLogic};
pub use self::circle_part::CirclePart;
pub use self::polar_pixel::PolarPixel;
pub use self::polygon::Polygon;
pub use self::text::{PlainText, TextAlign};
pub use self::rectangle::Rectangle;
use super::renderables::Renderable;

#[derive(Clone)]
pub enum Primitive {
    Rect(Rectangle),
    Circ(CirclePart),
    Poly(Polygon),
    PolarPix(PolarPixel),
    Text(PlainText),
    BezierRect(BezierRect),
    BezierSubrect(BezierSubrect),
}