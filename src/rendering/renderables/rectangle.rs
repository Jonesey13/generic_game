use rendering::{Renderable, StandardPrimitive, RectanglePrimitive};

impl Renderable<StandardPrimitive> for RectanglePrimitive {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Rect(self.clone())] }
}