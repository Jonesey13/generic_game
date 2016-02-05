pub mod circle;
pub mod con_poly;
use std::cell::RefCell;
use std::iter::{Repeat, repeat};

pub trait Collidable {
    type Data: Clone;
    fn get_collision_object(&self) -> CollObj { CollObj::None }
    fn get_collision_results(&self) -> CollResults<Self::Data>;
    fn set_collision_results(&mut self, CollResults<Self::Data>);
    fn has_collided(&self) -> bool { self.get_collision_results().collided }
}

pub enum CollObj {
    None,
    Circ(circle::Circle, Option<circle::Circle>),
    ConPoly(con_poly::ConPoly, Option<con_poly::ConPoly>)
}

#[derive(Clone)]
pub struct CollResults<T: Clone> {
    collided: bool,
    data: Option<T>
}

impl<T: Clone> CollResults<T> {
    pub fn new() -> CollResults<T> {
        CollResults {
            collided: false,
            data: None
        }
    }
}

pub struct Collider;

impl Collider {
    pub fn process_all<T> (&mut self, mut objects: Vec<&mut Collidable<Data=T>>) {
        loop {
            if let Some((first, rest)) = objects.split_last() {
                for second in rest {
                    Collider::process_pair(first, second);
                }
            }
            else {
                break;
            }
            objects.pop();
        }
    }

    fn process_pair<T> (first: &&mut Collidable<Data=T>, second: &&mut Collidable<Data=T>) {}
}
