use rendering::{Renderable, StandardPrimitive, PlainText};

impl Renderable for PlainText {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Text(self.clone())] }
}