use std::ops::Mul;
use super::{Point, Line};
use na::{Vector2, Rotation2};
use std::ops::Rem;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub start: IntervalEnd,
    pub end: IntervalEnd
}

impl Interval {
    pub fn new(start: IntervalEnd, end: IntervalEnd) -> Interval {
        Interval {
            start,
            end
        }
    }

    pub fn new_open(start: f64, end: f64) -> Interval {
        Interval {
            start: IntervalEnd::Open(start),
            end: IntervalEnd::Open(end)
        }
    }

    pub fn new_closed(start: f64, end: f64) -> Interval {
        Interval {
            start: IntervalEnd::Closed(start),
            end: IntervalEnd::Closed(end)
        }
    }

    pub fn new_closed_open(start: f64, end: f64) -> Interval {
        Interval {
            start: IntervalEnd::Closed(start),
            end: IntervalEnd::Open(end)
        }
    }

    pub fn new_open_closed(start: f64, end: f64) -> Interval {
        Interval {
            start: IntervalEnd::Open(start),
            end: IntervalEnd::Closed(end)
        }
    }

    fn set_start_value(&mut self, value: f64) {
        self.start = match self.start {
            IntervalEnd::Open(_) => IntervalEnd::Open(value),
            IntervalEnd::Closed(_) => IntervalEnd::Closed(value)
        }
    }

    fn set_end_value(&mut self, value: f64) {
        self.end = match self.end {
            IntervalEnd::Open(_) => IntervalEnd::Open(value),
            IntervalEnd::Closed(_) => IntervalEnd::Closed(value)
        }
    }

    pub fn get_start(&self) -> IntervalEnd {
       self.start
    }

    pub fn get_end(&self) -> IntervalEnd {
       self.end
    }

    pub fn get_start_value(&self) -> f64 {
       self.start.value()
    }

    pub fn get_end_value(&self) -> f64 {
       self.end.value()
    }

    pub fn wrap_value_to(&self, point: f64) -> f64 {
        let start = self.get_start_value();
        let end = self.get_end_value();
        let diff = start - end;
        let shifted_value = point - start;
        (shifted_value.rem(diff)).abs() + start
    }

    // TODO: Fix for end types
    pub fn wrap_interval_to(&self, int: Interval) -> Vec<Interval> {
        let int_start = int.get_start_value();
        let int_end = int.get_end_value();
        let start = self.get_start_value();
        let end = self.get_end_value();
        let diff = start - end;
        if int_end - int_start >= diff {
            return vec![self.clone()];
        }   
        let wrapped_start = self.wrap_value_to(int_start);
        let wrapped_end = self.wrap_value_to(int_end);

        if wrapped_start <= wrapped_end {
            vec![Interval::new_closed(wrapped_start, wrapped_end)]
        }
        else {
            vec![
                Interval::new_closed(start, wrapped_end), Interval::new_closed(wrapped_start, end)
            ]
        }
    }

    pub fn contains(&self, value: f64) -> bool {
        match (self.start, self.end) {
            (IntervalEnd::Open(start), IntervalEnd::Open(end)) => start < value && end > value,
            (IntervalEnd::Open(start), IntervalEnd::Closed(end)) => start < value && end >= value,
            (IntervalEnd::Closed(start), IntervalEnd::Open(end)) => start <= value && end > value,
            (IntervalEnd::Closed(start), IntervalEnd::Closed(end)) => start <= value && end >= value,
        }
    }

    pub fn point_to_regularised(&self, point: f64) -> f64 {
        (point - self.start.value()) / (self.end.value() - self.start.value())
    }

    pub fn get_point(&self, t: f64) -> f64 {
        self.start.value() * (1.0 - t) + self.end.value() * t
    }

    /// Fits a point in this interval to another interval
    pub fn fit_point_to(&self, point: f64, another: Interval) -> f64 {
        another.get_point(self.point_to_regularised(point))
    }

    pub fn get_collision_object(&self) -> IntervalCollisionObject {
        if let (IntervalEnd::Closed(val1), IntervalEnd::Closed(val2)) = (self.start, self.end) {
            if val1 == val2 {
                return IntervalCollisionObject::Point(self.start.value());
            }
        }
        return IntervalCollisionObject::Line(self.start.value(), self.end.value());
    }
}

impl Mul<f64> for Interval {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut output = self.clone();
        output.set_start_value(self.start.value() * rhs);
        output.set_end_value(self.end.value() * rhs);
        output
    }
}

#[derive(Copy, Clone, Debug)]
pub enum IntervalEnd {
    Open(f64),
    Closed(f64)
}

impl IntervalEnd {
    pub fn value(&self) -> f64 {
        match *self {
            IntervalEnd::Open(val) => val,
            IntervalEnd::Closed(val) => val
        }
    }

    pub fn equals(&self, other: &Self, tolerance: f64) -> bool {
        match (*self, *other) {
            (IntervalEnd::Open(val1), IntervalEnd::Open(val2)) => (val1 - val2).abs() < tolerance,
            (IntervalEnd::Closed(val1), IntervalEnd::Closed(val2)) => (val1 - val2).abs() < tolerance,
            _ => false
        }
    }
}

pub enum IntervalCollisionObject {
    Line(f64, f64), //(start, end)
    Point(f64)
}

pub enum IntervalCollisionObject2D {
    Point(Point),
    Line(Line)
}

impl IntervalCollisionObject {
    pub fn to_twod_collision_object(&self, pos: Vector2<f64>, rot: Rotation2<f64>) -> IntervalCollisionObject2D{
        match *self {
            IntervalCollisionObject::Point(val) 
            => IntervalCollisionObject2D::Point(Point::new(rot * Vector2::new(val, 0.0) + pos)),
            
            IntervalCollisionObject::Line(val1, val2) 
            => IntervalCollisionObject2D::Line(
                Line::new(rot * Vector2::new(val1, 0.0) + pos, rot * Vector2::new(val2, 0.0) + pos)
            )
        }
    }
}

pub struct IntervalWith2DPosition {
    interval: Interval,
    pos: Vector2<f64>,
    rot: Rotation2<f64>
}

impl IntervalWith2DPosition {
    pub fn new(interval: Interval, pos: Vector2<f64>, rot: Rotation2<f64>) -> Self {
        Self {
            interval,
            pos,
            rot
        }
    }
}