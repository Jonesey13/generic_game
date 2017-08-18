use rendering::{Renderable, Primitive, Polygon};

impl Renderable for Polygon {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::Poly(self.clone())] }
}