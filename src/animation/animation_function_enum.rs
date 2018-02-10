use ::animation::animation_functions::*;

#[derive(Copy, Clone, Debug)]
pub enum AnimationFunctionEnum {
    SlowInSlowOut,
    SlowIn,
    SlowOut,
    Linear
}

impl AnimationFunctionEnum {
    pub fn build_function(self) -> Box<Fn(f64) -> f64> {
        match self {
            AnimationFunctionEnum::SlowInSlowOut => Box::new(interpolation_cubic),
            AnimationFunctionEnum::SlowOut => Box::new(reversed_quadratic),
            AnimationFunctionEnum::SlowIn => Box::new(simple_quadratic),
            AnimationFunctionEnum::Linear => Box::new(identity)
        }
    }
}