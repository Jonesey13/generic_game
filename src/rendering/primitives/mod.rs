pub mod circle_part;
pub mod polygon;
pub mod text;
pub mod rectangle;
pub mod texture_rect;
pub use self::circle_part::CirclePart;
pub use self::polygon::Polygon;
pub use self::text::*;
pub use self::rectangle::*;
pub use self::texture_rect::TextureRect;
use super::renderables::Renderable;

#[derive(Clone)]
pub enum StandardPrimitive {
    Rect(RectanglePrimitive),
    TextureRect(TextureRect),
    Circ(CirclePart),
    Poly(Polygon),
    Text(PlainText),
}