use super::CollisionObjectDetails;

#[derive(Clone, Debug)]
pub struct CollisionDetails {
    location: usize,
    object_details: CollisionObjectDetails
}

impl CollisionDetails {
    pub fn new(location: usize, object_details: CollisionObjectDetails) -> Self {
        Self {
            location,
            object_details
        }
    }
}