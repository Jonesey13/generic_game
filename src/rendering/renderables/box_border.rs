use na::{Vector2, Vector3, Vector4};
use rendering::{Renderable, StandardPrimitive, Rectangle, AnnularSegment};

#[derive(Clone, Debug)]
pub struct BoxBorder {
    thickness: f64,
    pos: Vector3<f64>,
    rect_height: f64,
    rect_width: f64,
    color: Vector4<f64>,
    border_type: BorderType,
    fixed: bool
}

impl BoxBorder {
    pub fn new(
    thickness: f64,
    pos: Vector3<f64>,
    rect_height: f64,
    rect_width: f64,
    color: Vector4<f64>,
    fixed: bool) -> Self {
        Self {
            thickness,
            pos,  
            rect_height,  
            rect_width,
            color,
            border_type: BorderType::Straight,
            fixed,
        }
    }

    pub fn new_rounded(
        thickness: f64,
        corner_radius: f64,
        pos: Vector3<f64>,
        rect_height: f64,
        rect_width: f64,
        color: Vector4<f64>,
        fixed: bool) -> Self {
        Self {
            thickness,
            pos,  
            rect_height,  
            rect_width,
            color,
            border_type: BorderType::Round(corner_radius),
            fixed
        }
    }

    fn get_straight_primitives(&self) -> Vec<StandardPrimitive> {
        let left_pos = self.pos - self.rect_width / 2.0 * Vector3::x();
        let right_pos = self.pos + self.rect_width / 2.0 * Vector3::x();
        let lower_pos = self.pos - self.rect_height / 2.0 * Vector3::y();
        let upper_pos = self.pos + self.rect_height / 2.0 * Vector3::y();
        let full_width = self.rect_width + self.thickness;
        let full_height = self.rect_height + self.thickness;

        let left_wall = Rectangle::new_regular(self.thickness, full_height, left_pos, self.color, self.fixed);
        let right_wall = Rectangle::new_regular(self.thickness, full_height, right_pos, self.color, self.fixed);
        let lower_wall = Rectangle::new_regular(full_width, self.thickness, lower_pos, self.color, self.fixed);
        let upper_wall = Rectangle::new_regular(full_width, self.thickness, upper_pos, self.color, self.fixed);        
        
        vec![StandardPrimitive::Rect(left_wall), 
        StandardPrimitive::Rect(right_wall), 
        StandardPrimitive::Rect(lower_wall), 
        StandardPrimitive::Rect(upper_wall)]
    }

    fn get_rounded_primitives(&self, border_radius: f64) -> Vec<StandardPrimitive> {
        let x_shift = self.rect_width / 2.0 * Vector3::x();
        let y_shift = self.rect_height / 2.0 * Vector3::y();
        let x_radius_shift = border_radius * Vector3::x();
        let y_radius_shift = border_radius * Vector3::y();

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

        let left_wall = Rectangle::new_regular(self.thickness, side_height, left_pos, self.color, self.fixed);
        let right_wall = Rectangle::new_regular(self.thickness, side_height, right_pos, self.color, self.fixed);
        let lower_wall = Rectangle::new_regular(side_width, self.thickness, lower_pos, self.color, self.fixed);
        let upper_wall = Rectangle::new_regular(side_width, self.thickness, upper_pos, self.color, self.fixed); 

        let corner_radial_dim = Vector2::new(border_radius - self.thickness / 2.0, border_radius + self.thickness / 2.0);
        let upper_left_circ = AnnularSegment::new(corner_radial_dim, Vector2::new(0.75, 1.0), upper_left_pos, self.color, self.fixed);
        let upper_right_circ = AnnularSegment::new(corner_radial_dim, Vector2::new(0.0, 0.25), upper_right_pos, self.color, self.fixed);
        let lower_right_circ = AnnularSegment::new(corner_radial_dim, Vector2::new(0.25, 0.5), lower_right_pos, self.color, self.fixed);
        let lower_left_circ = AnnularSegment::new(corner_radial_dim, Vector2::new(0.5, 0.75), lower_left_pos, self.color, self.fixed);
        
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

impl Renderable for BoxBorder {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> {
        match self.border_type {
            BorderType::Straight => self.get_straight_primitives(),
            BorderType::Round(radius) => self.get_rounded_primitives(radius)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BorderType {
    Straight,
    Round(f64) // Radius
}