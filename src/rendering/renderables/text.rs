use crate::rendering::{Renderable, StandardPrimitive, PlainText};

impl Renderable<StandardPrimitive> for PlainText {
    fn get_primitives(&mut self) -> Vec<StandardPrimitive> { vec![StandardPrimitive::Text(self.clone())] }
}