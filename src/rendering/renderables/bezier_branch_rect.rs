use rendering::{Renderable, StandardPrimitive, BezierBranchRect};

impl Renderable for BezierBranchRect {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::BezierBranchRect(self.clone())] }
}