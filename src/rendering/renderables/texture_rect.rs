use rendering::{Renderable, StandardPrimitive, TextureRect};

impl Renderable for TextureRect {
    type Primitive = StandardPrimitive;
    
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::TextureRect(self.clone())] }
}