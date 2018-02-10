pub struct AnimationTime<Stage: Clone> {
    pub stage: Stage,
    pub stage_time: f64,
    pub finished: bool
}

impl<Stage: Clone> AnimationTime<Stage> {
    pub fn new(stage: Stage, stage_time: f64, finished: bool) -> Self {
        Self {
            stage,
            stage_time,
            finished
        }
    }
}