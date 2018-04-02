use rendering::{Renderable, StandardPrimitive, TextureRect};

impl Renderable<StandardPrimitive> for TextureRect {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::TextureRect(self.clone())] }
}