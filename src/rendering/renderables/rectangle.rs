use rendering::{Renderable, Primitive, Rectangle};

impl Renderable for Rectangle {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::Rect(self.clone())] }
}