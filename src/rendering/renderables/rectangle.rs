use rendering::{Renderable, StandardPrimitive, Rectangle};

impl Renderable<StandardPrimitive> for Rectangle {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Rect(self.clone())] }
}