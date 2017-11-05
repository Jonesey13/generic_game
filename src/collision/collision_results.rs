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

    pub fn no_collision() -> CollisionResults<T> {
        CollisionResults {
            collided: false,
            details: None,
            time: None,
            data: None,
        }
    }
}

impl<T: Clone> From<CollisionObjectResults<T>> for CollisionResults<T> {
    fn from(obj_result: CollisionObjectResults<T>) -> Self {
            CollisionResults::new_with_location(0, obj_result)
    }
}