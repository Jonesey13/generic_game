// Satisfies f(0) = f'(0) = f'(1) = 0 and f(1) = 1
pub fn interpolation_cubic(t: f64) -> f64 {
    -2.0 * t * t * t + 3.0 * t * t 
}

pub fn simple_quadratic(t: f64) -> f64 {
    t*t
}

pub fn reversed_quadratic(t: f64) -> f64 {
    (1.0 - t) * (1.0 - t)
}

pub fn identity(t: f64) -> f64 {
    t
}