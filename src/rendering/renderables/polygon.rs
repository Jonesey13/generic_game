use rendering::{Renderable, StandardPrimitive, Polygon};

impl Renderable<StandardPrimitive> for Polygon {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Poly(self.clone())] }
}