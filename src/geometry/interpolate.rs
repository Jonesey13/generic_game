use std::ops::{Add, Mul};

pub fn interpolate<T> (start: T, end: T, time: f64) -> T
    where T: Add<T, Output=T> + Mul<f64, Output=T>
{
    start * (1.0 - time)  + end * time
}
