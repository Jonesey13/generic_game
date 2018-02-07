use rendering::{Renderable, Primitive, TextureRect};

impl Renderable for TextureRect {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::TextureRect(self.clone())] }
}