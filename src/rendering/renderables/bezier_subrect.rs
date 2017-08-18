use rendering::{Renderable, Primitive, BezierSubrect};

impl Renderable for BezierSubrect {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::BezierSubrect(self.clone())] }
}