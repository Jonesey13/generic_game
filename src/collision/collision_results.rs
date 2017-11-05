use super::{CollisionDetails, CollisionObjectResults};

#[derive(Clone)]
pub struct CollisionResults<T: Clone> {
    pub collided: bool,
    pub details: Option<CollisionDetails>,
    pub time: Option<f64>,
    pub data: Option<T>
}

impl<T: Clone> CollisionResults<T> {
    pub fn new_with_location(location: usize, object_results: CollisionObjectResults<T>) -> Self {
        CollisionResults {
            collided: object_results.collided,
            details: match object_results.details {
                None => None,
                Some(obj_details) => Some(CollisionDetails::new(location, obj_details))
            },
            time: object_results.time,
            data: object_results.data
        }
    }
}