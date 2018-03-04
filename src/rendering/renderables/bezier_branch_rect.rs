use rendering::{Renderable, Primitive, BezierBranchRect};

impl Renderable for BezierBranchRect {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::BezierBranchRect(self.clone())] }
}