pub mod bezier_rect;
pub mod bezier_branch_rect;
pub mod bezier_branch_circ;
pub mod circle_part;
pub mod polar_pixel;
pub mod polygon;
pub mod text;
pub mod rectangle;
pub mod texture_rect;
pub use self::bezier_rect::{BezierRect, BezierQuadControl};
pub use self::bezier_branch_rect::BezierBranchRect;
pub use self::bezier_branch_circ::BezierBranchCirc;
pub use self::circle_part::CirclePart;
pub use self::polar_pixel::PolarPixel;
pub use self::polygon::Polygon;
pub use self::text::{PlainText, TextAlign};
pub use self::rectangle::Rectangle;
pub use self::texture_rect::TextureRect;
use super::renderables::Renderable;

#[derive(Clone)]
pub enum StandardPrimitive {
    Rect(Rectangle),
    TextureRect(TextureRect),
    Circ(CirclePart),
    Poly(Polygon),
    PolarPix(PolarPixel),
    Text(PlainText),
    BezierRect(BezierRect),
    BezierBranchRect(BezierBranchRect),
    BezierBranchCirc(BezierBranchCirc),    
}