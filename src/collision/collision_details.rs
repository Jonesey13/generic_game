use super::CollisionObjectDetails;

#[derive(Clone, Debug)]
pub struct CollisionDetails {
    pub location: usize,
    pub object_details: CollisionObjectDetails,
    pub time: f64
}

impl CollisionDetails {
    pub fn new(location: usize, object_details: CollisionObjectDetails, time: f64) -> Self {
        Self {
            location,
            object_details,
            time
        }
    }
}