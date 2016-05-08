use time;
use utils::debug::*;

pub struct Clock {
    name: String,
    last_time: f64,
    cycle_history: Vec<f64>,
}

impl Clock {
    pub fn new(name: &str) -> Clock {
        Clock {
            name: name.to_string(),
            last_time: 0.0,
            cycle_history: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.last_time = time::precise_time_s();
    }

    pub fn end(&mut self) {
        let cycle_time = time::precise_time_s() - self.last_time;
        self.cycle_history.push(cycle_time);

        if self.cycle_history.iter().fold(0.0, |acc, x| acc + x) >= 1.0 {
            let total_cycles = self.cycle_history.len();
            let longest_cycle = self.cycle_history.iter().fold(0.0, |acc: f64, &x|{ acc.max(x) });

            debug_clock(&format!("Clock {}: Clocks per second: {}, Longest Clock: {}", self.name, total_cycles, longest_cycle));
            self.cycle_history.clear();
        }
    }
}
