use rendering::{Renderable, StandardPrimitive, Rectangle};

impl Renderable for Rectangle {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Rect(self.clone())] }
}