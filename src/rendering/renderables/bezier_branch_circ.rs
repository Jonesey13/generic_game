use rendering::{Renderable, StandardPrimitive, BezierBranchCirc};

impl Renderable for BezierBranchCirc {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::BezierBranchCirc(self.clone())] }
}