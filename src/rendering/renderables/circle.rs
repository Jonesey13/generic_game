use rendering::{Renderable, Primitive, Circle};

impl Renderable for Circle {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::Circ(self.clone())] }
}