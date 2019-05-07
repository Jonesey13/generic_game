use crate::animation::{AnimationWindow, AnimationType, AnimationTime};

pub struct AnimationSpec<Stage: Clone> {
    windows: Vec<AnimationWindow<Stage>>,
    anim_type: AnimationType,
    time: f64
}

impl<Stage: Clone> AnimationSpec<Stage>{
    pub fn new(windows: Vec<AnimationWindow<Stage>>, anim_type: AnimationType) -> Self {
        Self {
            windows,
            anim_type,
            time: 0.0
        }
    }

    pub fn update(&mut self, t_step: f64) {
        self.time += t_step;

        if self.anim_type == AnimationType::Repeat {
            self.time = self.time % self.get_total_running_time();
        }
    }

    fn get_total_running_time(&self) -> f64 {
        self.windows.iter().map(|window| {window.length}).sum()
    }

    pub fn get_current_time(&self) -> AnimationTime<Stage> {
        let window_times: Vec<f64> = self.windows.iter().map(|window| {window.length}).collect(); 
        let cumulative_times: Vec<f64> = (1..window_times.len() + 1)
            .map(|index| {window_times.iter().take(index).sum()})
            .collect(); 
        let cumulative_times_prev: Vec<f64> = (0..window_times.len())
            .map(|index| {window_times.iter().take(index).sum()})
            .collect(); 

        let current_window_opt = cumulative_times.into_iter().position(|time| {self.time <= time});
        let current_window_index = match current_window_opt {
            Some(index) => index,
            None => {
                let current_window = &self.windows[self.windows.len() - 1];
                return AnimationTime::new(current_window.stage.clone(), 1.0, true);
            }
        };

        let current_window = &self.windows[current_window_index];
        let current_window_time_unprocessed = (self.time - cumulative_times_prev[current_window_index]) / current_window.length;
        let current_window_time = (current_window.time_function)(current_window_time_unprocessed);

        AnimationTime::new(current_window.stage.clone(), current_window_time, false)
    }
}