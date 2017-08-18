use rendering::{Renderable, Primitive, PlainText};

impl Renderable for PlainText {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::Text(self.clone())] }
}