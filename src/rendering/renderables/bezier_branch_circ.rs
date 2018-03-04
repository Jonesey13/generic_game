use rendering::{Renderable, Primitive, BezierBranchCirc};

impl Renderable for BezierBranchCirc {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::BezierBranchCirc(self.clone())] }
}