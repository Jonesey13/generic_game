use rendering::{Renderable, StandardPrimitive, BezierRect};

impl Renderable for BezierRect {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::BezierRect(self.clone())] }
}