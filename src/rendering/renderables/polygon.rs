use rendering::{Renderable, StandardPrimitive, Polygon};

impl Renderable for Polygon {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Poly(self.clone())] }
}