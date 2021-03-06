use crate::rendering::*;
use crate::geometry::*;

#[derive(Clone, Debug)]
pub struct BoxBorder {
    pub thickness: f64,
    pub pos: Point3,
    pub rect_height: f64,
    pub rect_width: f64,
    pub color: Color,
    pub border_type: BoxBorderType,
    pub fixed: bool
}

impl BoxBorder {
    pub fn new(
    thickness: f64,
    pos: Point3,
    rect_height: f64,
    rect_width: f64,
    color: Color,
    fixed: bool) -> Self {
        Self {
            thickness,
            pos,  
            rect_height,  
            rect_width,
            color,
            border_type: BoxBorderType::Straight,
            fixed,
        }
    }

    pub fn new_rounded(
        thickness: f64,
        corner_radius: f64,
        pos: Point3,
        rect_height: f64,
        rect_width: f64,
        color: Color,
        fixed: bool) -> Self {
        Self {
            thickness,
            pos,  
            rect_height,  
            rect_width,
            color,
            border_type: BoxBorderType::Round(corner_radius),
            fixed
        }
    }

    fn get_straight_primitives(&self) -> Vec<StandardPrimitive> {
        let left_pos = self.pos - self.rect_width / 2.0 * Point3::x();
        let right_pos = self.pos + self.rect_width / 2.0 * Point3::x();
        let lower_pos = self.pos - self.rect_height / 2.0 * Point3::y();
        let upper_pos = self.pos + self.rect_height / 2.0 * Point3::y();
        let full_width = self.rect_width + self.thickness;
        let full_height = self.rect_height + self.thickness;

        let left_wall = RectanglePrimitive::new_regular(self.thickness, full_height, left_pos, self.color, self.fixed);
        let right_wall = RectanglePrimitive::new_regular(self.thickness, full_height, right_pos, self.color, self.fixed);
        let lower_wall = RectanglePrimitive::new_regular(full_width, self.thickness, lower_pos, self.color, self.fixed);
        let upper_wall = RectanglePrimitive::new_regular(full_width, self.thickness, upper_pos, self.color, self.fixed);        
        
        vec![StandardPrimitive::Rect(left_wall), 
        StandardPrimitive::Rect(right_wall), 
        StandardPrimitive::Rect(lower_wall), 
        StandardPrimitive::Rect(upper_wall)]
    }

    fn get_rounded_primitives(&self, border_radius: f64) -> Vec<StandardPrimitive> {
        let x_shift = self.rect_width / 2.0 * Point3::x();
        let y_shift = self.rect_height / 2.0 * Point3::y();
        let x_radius_shift = border_radius * Point3::x();
        let y_radius_shift = border_radius * Point3::y();

        let left_pos = self.pos - x_shift;
        let right_pos = self.pos + x_shift;
        let lower_pos = self.pos - y_shift;
        let upper_pos = self.pos + y_shift;
        let upper_left_pos = self.pos - x_shift + y_shift + x_radius_shift - y_radius_shift;
        let upper_right_pos = self.pos + x_shift + y_shift - x_radius_shift - y_radius_shift;
        let lower_left_pos = self.pos - x_shift - y_shift + x_radius_shift + y_radius_shift;
        let lower_right_pos = self.pos + x_shift - y_shift - x_radius_shift + y_radius_shift;
        let side_width = self.rect_width - 2.0 * border_radius;
        let side_height = self.rect_height - 2.0 * border_radius;

        let left_wall = RectanglePrimitive::new_regular(self.thickness, side_height, left_pos, self.color, self.fixed);
        let right_wall = RectanglePrimitive::new_regular(self.thickness, side_height, right_pos, self.color, self.fixed);
        let lower_wall = RectanglePrimitive::new_regular(side_width, self.thickness, lower_pos, self.color, self.fixed);
        let upper_wall = RectanglePrimitive::new_regular(side_width, self.thickness, upper_pos, self.color, self.fixed); 

        let corner_radial_dim = Point::new(border_radius - self.thickness / 2.0, border_radius + self.thickness / 2.0);
        let upper_left_circ = AnnularSegment::new(corner_radial_dim, Point::new(0.25, 0.5), upper_left_pos, self.color, self.fixed);
        let upper_right_circ = AnnularSegment::new(corner_radial_dim, Point::new(0.0, 0.25), upper_right_pos, self.color, self.fixed);
        let lower_right_circ = AnnularSegment::new(corner_radial_dim, Point::new(0.75, 1.0), lower_right_pos, self.color, self.fixed);
        let lower_left_circ = AnnularSegment::new(corner_radial_dim, Point::new(0.5, 0.75), lower_left_pos, self.color, self.fixed);
        
        vec![StandardPrimitive::Rect(left_wall), 
        StandardPrimitive::Rect(right_wall), 
        StandardPrimitive::Rect(lower_wall), 
        StandardPrimitive::Rect(upper_wall),
        StandardPrimitive::Circ(upper_left_circ.into()),
        StandardPrimitive::Circ(upper_right_circ.into()),
        StandardPrimitive::Circ(lower_right_circ.into()),
        StandardPrimitive::Circ(lower_left_circ.into())]
    }
}

impl Renderable<StandardPrimitive> for BoxBorder {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> {
        match self.border_type {
            BoxBorderType::Straight => self.get_straight_primitives(),
            BoxBorderType::Round(radius) => self.get_rounded_primitives(radius)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BoxBorderType {
    Straight,
    Round(f64) // Radius
}