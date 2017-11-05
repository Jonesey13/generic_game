use super::collision_details::CollisionDetails;

#[derive(Clone)]
pub struct CollisionResults<T: Clone> {
    pub collided: bool,
    pub details: Option<CollisionDetails>,
    pub time: Option<f64>,
    pub data: Option<T>
}

impl<T: Clone> CollisionResults<T> {
    pub fn no_collision() -> CollisionResults<T> {
        CollisionResults {
            collided: false,
            details: None,
            time: None,
            data: None,
        }
    }

    pub fn collided(details: CollisionDetails, time: f64) -> CollisionResults<T> {
        CollisionResults {
            collided: true,
            details: Some(details),
            time: Some(time),
            data: None,
        }
    }

    pub fn to_line_results(mut self) -> CollisionResults<T> {
        if self.collided {
            self.details = self.details.and_then(|d| {Some(CollisionDetails::Line(d.to_line_info()))});
        }
        self
    }
}