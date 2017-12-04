use super::{CollisionObjectDetails, CollisionResults};

#[derive(Clone, Debug)]
pub struct CollisionObjectResults<T: Clone> {
    pub collided: bool,
    pub details: Option<CollisionObjectDetails>,
    pub time: Option<f64>,
    pub data: Option<T>
}

impl<T: Clone> CollisionObjectResults<T> {
    pub fn no_collision() -> CollisionObjectResults<T> {
        CollisionObjectResults {
            collided: false,
            details: None,
            time: None,
            data: None,
        }
    }

    pub fn collided(details: CollisionObjectDetails, time: f64) -> CollisionObjectResults<T> {
        CollisionObjectResults {
            collided: true,
            details: Some(details),
            time: Some(time),
            data: None,
        }
    }

    pub fn to_line_results(mut self) -> CollisionObjectResults<T> {
        if self.collided {
            self.details = self.details.and_then(|d| {Some(CollisionObjectDetails::Line(d.to_line_info()))});
        }
        self
    }
}

impl<T: Clone> From<CollisionResults<T>> for CollisionObjectResults<T> {
    fn from(coll_results: CollisionResults<T>) -> Self {
        CollisionObjectResults {
            collided: coll_results.collided,
            details: match coll_results.details {
                None => None,
                Some(details) => Some(details.object_details)
            },
            time: coll_results.time,
            data: coll_results.data
        }
    }
}