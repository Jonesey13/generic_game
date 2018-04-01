use rendering::{Renderable, StandardPrimitive, PolarPixel};

impl Renderable for PolarPixel {
    type Primitive = StandardPrimitive;
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::PolarPix(self.clone())] }
}