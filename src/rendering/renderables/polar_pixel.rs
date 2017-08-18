use rendering::{Renderable, Primitive, PolarPixel};

impl Renderable for PolarPixel {
    fn get_primitives(&mut self) -> Vec<Primitive> { vec![Primitive::PolarPix(self.clone())] }
}