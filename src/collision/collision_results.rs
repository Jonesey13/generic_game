use super::{CollisionDetails, CollisionObjectResults};

#[derive(Clone, Debug)]
pub struct CollisionResults<T: Clone> {
    pub details: CollisionDetails,
    pub data: T
}

impl<T: Clone> CollisionResults<T> {
    pub fn new(details: CollisionDetails, data: T) -> Self {
        CollisionResults {
            details: details,
            data: data
        }
    }
}
