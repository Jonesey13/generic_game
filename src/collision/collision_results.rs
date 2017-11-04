use super::collision_details::CollDetails;

#[derive(Clone)]
pub struct CollResults<T: Clone> {
    pub collided: bool,
    pub details: Option<CollDetails>,
    pub time: Option<f64>,
    pub data: Option<T>
}

impl<T: Clone> CollResults<T> {
    pub fn no_collision() -> CollResults<T> {
        CollResults {
            collided: false,
            details: None,
            time: None,
            data: None,
        }
    }

    pub fn collided(details: CollDetails, time: f64) -> CollResults<T> {
        CollResults {
            collided: true,
            details: Some(details),
            time: Some(time),
            data: None,
        }
    }

    pub fn to_line_results(mut self) -> CollResults<T> {
        if self.collided {
            self.details = self.details.and_then(|d| {Some(CollDetails::Line(d.to_line_info()))});
        }
        self
    }
}