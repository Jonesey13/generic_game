use super::AnimationFunctionEnum;

pub struct AnimationWindow<Stage: Clone> {
    pub time_function: Box<dyn Fn(f64) -> f64>, // should map [0,1] to [0,1]
    pub length: f64,
    pub stage: Stage
}

impl<Stage: Clone> AnimationWindow<Stage> {
    pub fn new(time_function: AnimationFunctionEnum, length: f64, stage: Stage) -> Self {
        Self {
            time_function: time_function.build_function(),
            length,
            stage
        }
    }
}