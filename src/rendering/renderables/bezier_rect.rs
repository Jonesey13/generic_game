use rendering::{Renderable, Primitive, BezierRect};

impl Renderable for BezierRect {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::BezierRect(self.clone())] }
}