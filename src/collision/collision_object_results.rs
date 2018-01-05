use super::{CollisionObjectDetails, CollisionResults};

#[derive(Clone, Debug)]
pub struct CollisionObjectResults {
    pub details: CollisionObjectDetails,
    pub time: f64
}

impl CollisionObjectResults {
    pub fn collided(details: CollisionObjectDetails, time: f64) -> CollisionObjectResults {
        CollisionObjectResults {
            details: details,
            time: time,
        }
    }

    pub fn to_line_results(mut self) -> CollisionObjectResults {
        self.details = CollisionObjectDetails::Line(self.details.to_line_info());
        self
    }
}

impl<T: Clone> From<CollisionResults<T>> for CollisionObjectResults {
    fn from(coll_results: CollisionResults<T>) -> Self {
        CollisionObjectResults {
            details: coll_results.details.object_details,
            time: coll_results.details.time
        }
    }
}