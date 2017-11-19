use na::{Vector3, Vector4};
use rendering::{Renderable, Primitive, Rectangle};

pub struct BoxBorder {
    thickness: f64,
    pos: Vector3<f64>,
    rect_height: f64,
    rect_width: f64,
    colour: Vector4<f64>
}

impl BoxBorder {
    pub fn new(
    thickness: f64,
    pos: Vector3<f64>,
    rect_height: f64,
    rect_width: f64,
    colour: Vector4<f64>) -> Self {
        Self {
            thickness,
            pos,  
            rect_height,  
            rect_width,
            colour,
        }
    }
}

impl Renderable for BoxBorder {
    fn get_primitives(&mut self) -> Vec<Primitive> {
        let left_pos = self.pos - self.rect_width / 2.0 * Vector3::x();
        let right_pos = self.pos + self.rect_width / 2.0 * Vector3::x();
        let lower_pos = self.pos - self.rect_height / 2.0 * Vector3::y();
        let upper_pos = self.pos + self.rect_height / 2.0 * Vector3::y();
        let full_width = self.rect_width + self.thickness;
        let full_height = self.rect_height + self.thickness;

        let left_wall = Rectangle::new_regular(self.thickness, full_height, left_pos, self.colour);
        let right_wall = Rectangle::new_regular(self.thickness, full_height, right_pos, self.colour);
        let lower_wall = Rectangle::new_regular(full_width, self.thickness, lower_pos, self.colour);
        let upper_wall = Rectangle::new_regular(full_width, self.thickness, upper_pos, self.colour);        
        
        vec![Primitive::Rect(left_wall), 
        Primitive::Rect(right_wall), 
        Primitive::Rect(lower_wall), 
        Primitive::Rect(upper_wall)]
    }
}