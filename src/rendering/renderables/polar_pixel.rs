use rendering::{Renderable, StandardPrimitive, PolarPixel};

impl Renderable<StandardPrimitive> for PolarPixel {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::PolarPix(self.clone())] }
}