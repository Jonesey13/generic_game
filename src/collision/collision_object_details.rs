use geometry;
use geometry::{circle, con_poly, line, HasAngle, DualSoln, Poly, poly};
use geometry::circle::Circle;
use geometry::con_poly::ConPoly;
use geometry::line::Line;
use na::{normalize, Vector2, dot, abs};

pub enum CollisionObjectState {
    None,
    Circ(Circle, Circle),
    ConPoly(ConPoly, ConPoly),
    Line(Line, Line),
    Point(Vector2<f64>, Vector2<f64>)
}

#[derive(Clone, Debug)]
pub enum CollisionObjectDetails {
    None,
    Point(Vector2<f64>),
    Line(LineInfo), 
    Circ(Vector2<f64>), // Collision direction, outward from object
    ConPoly(ConPolyInfo),
}

#[derive(Clone, Debug)]
pub enum LineInfo {
    Point(f64, LineSide), // Position on line => (0,1)
    LineEnd(Vector2<f64>),
    LineBeg(Vector2<f64>),
    WholeLine(LineSide) // Collision along segment of the line
}

// Considered from beginning of line to end
#[derive(Clone, Debug)]
pub enum LineSide {
    Left,
    Right
}

#[derive(Clone, Debug)]
pub enum ConPolyInfo {
    LineInfo(usize, f64), // Line number and position on line => (0,1)
    CornerInfo(usize, Vector2<f64>), // Corner number and striking direction (outward)
    SideInfo(usize), // Collision along a side
}

impl CollisionObjectDetails {
    pub fn to_line_info(self) -> LineInfo {
        let con_poly_info = match self {
            CollisionObjectDetails::ConPoly(con_poly_info) => con_poly_info,
            CollisionObjectDetails::Line(line_info) => return line_info,
            _ => panic!("Cannot convert CollisionObjectDetails to LineInfo!")
        };
        match con_poly_info {
            ConPolyInfo::LineInfo(0, x) => LineInfo::Point(x, LineSide::Right),
            ConPolyInfo::LineInfo(1, x) => LineInfo::Point(x, LineSide::Left),            
            ConPolyInfo::CornerInfo(0, dir) => LineInfo::LineBeg(dir),
            ConPolyInfo::CornerInfo(1, dir) => LineInfo::LineEnd(dir),
            ConPolyInfo::SideInfo(0) => LineInfo::WholeLine(LineSide::Right),
            ConPolyInfo::SideInfo(1) => LineInfo::WholeLine(LineSide::Left),            
            _ => panic!("Invalid ConPolyInfo to convert to LineInfo!")
        }
    }
}