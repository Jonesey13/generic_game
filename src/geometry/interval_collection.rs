use super::{Interval, IntervalEnd, IntervalCollisionObject};
use collision::Collidable;
use na::{Vector2, Rotation2};

#[derive(Debug, Clone)]
pub struct IntervalCollection {
    pub start: IntervalEnd,
    pub end: IntervalEnd,
    intervals: Vec<Interval>
}

impl IntervalCollection {
    const ERRORMARGIN: f64 = 0.0001;

    pub fn new(start: IntervalEnd, end: IntervalEnd, intervals: Vec<Interval>) -> Self {
        Self {
            start,
            end,
            intervals
        }
    }

    fn relative_error_margin(&self) -> f64 {
        Self::ERRORMARGIN * (self.end.value() - self.start.value())
    }

    fn get_starts(&self) -> Vec<IntervalEnd> {
        self.intervals.iter().map(|end| { end.get_start().clone() }).collect()
    }

    fn get_ends(&self) -> Vec<IntervalEnd> {
        self.intervals.iter().map(|end| { end.get_end().clone() }).collect()
    }

    pub fn get_start_end_interval(&self) -> Interval {
        Interval::new(self.start, self.end)
    }

    pub fn invert(&self) -> IntervalCollection {
        let mut inv_intervals: Vec<Interval> = Vec::new();

        let start_included = self.start.equals(&self.intervals[0].get_start(), self.relative_error_margin());
        if !start_included {
            let first_interval = Interval::new(self.start, self.intervals[0].get_start());
            inv_intervals.push(first_interval);
        }

        for (end_prev, start_next) in self.get_ends().iter().zip(self.get_starts().iter().skip(1)) {
            inv_intervals.push(Interval::new(end_prev.clone(), start_next.clone()));
        } 

        let end_included = self.end.equals(&self.intervals[0].get_end(), self.relative_error_margin());
        if !end_included {
            let last_interval = Interval::new(self.end, self.intervals[0].get_end());
            inv_intervals.push(last_interval);
        }

        IntervalCollection {
            start: self.start,
            end: self.end,
            intervals: inv_intervals
        }
    }

    pub fn contains(&self, point: f64) -> bool {
        self.intervals.iter().fold(false, |acc, int| {int.contains(point) || acc})
    }

    pub fn get_collision_objects(&self) -> Vec<IntervalCollisionObject> {
        self.intervals.iter().map(|interval| {interval.get_collision_object()} ).collect()
    }

    pub fn wrap_to_interval(&self, int: Interval) -> IntervalCollection {
        let wrapped_intervals = self.intervals.iter().flat_map(|inner_int| {int.wrap_interval_to(*inner_int)}).collect();
        Self::new(int.start, int.end, wrapped_intervals)
    }
}

pub struct IntervalCollectionWith2DPosition {
    collection: IntervalCollection,
    pos: Vector2<f64>,
    rot: Rotation2<f64>
}

impl IntervalCollectionWith2DPosition {
    pub fn new(collection: IntervalCollection, pos: Vector2<f64>, rot: Rotation2<f64>) -> Self {
        Self {
            collection,
            pos,
            rot
        }
    }
}
 