use super::CollisionObjectDetails;

#[derive(Clone)]
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