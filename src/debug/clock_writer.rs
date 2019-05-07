use crate::debug::*;
use std::collections::HashMap;
use time;

#[derive(Default)]
pub struct ClockWriter {
    clocks: HashMap<String, ClockData>,
    global_clock: ClockData
}

impl ClockWriter {
    pub fn log_clocks(&mut self) {
        let root_clocks = self.clocks.iter().filter(|&(_,v)| { v.parent_key == None} );
        for (clock_name, _) in root_clocks {
            let clock_name: String = clock_name.clone();
            self.log_clock_recursive(clock_name, "".to_string());
        }
    }

    pub fn start(&mut self) {
        self.global_clock.start();
    }

    pub fn stop(&mut self) {
        self.global_clock.stop();
        
        let total_time = time::precise_time_s() - self.global_clock.start_time;
        
        if total_time >= 1.0 {
            debug_clock(&format!("Clock {}: Clocks per second: {}, Longest Clock: {}", "Overall".to_string(), self.global_clock.ticks, self.global_clock.longest_tick));
            self.global_clock.reset();
            self.log_clocks();
            debug_clock(&format!(""));
            
            for (_, ref mut clock) in self.clocks.iter_mut() {
                clock.reset();
            }
        }
    }

    pub fn start_clock(&mut self, clock_name: String) {
        let clock_exists = self.clocks.get(&clock_name).is_some();
        
        if clock_exists {
            self.clocks.get_mut(&clock_name).unwrap().start();
        }
        else if let Some(parent_key) = get_parent_key(clock_name.clone()) {
            let parent_exists = self.clocks.get(&parent_key).is_some();

            if parent_exists {
                self.clocks.get_mut(&parent_key.clone()).unwrap().child_keys.push(clock_name.clone());
                self.clocks.insert(clock_name, ClockData::new_parent(parent_key));
            }
            else {
                return;
            }
        }
        else {
            self.clocks.insert(clock_name, ClockData::default());
        }
    }

    pub fn stop_clock(&mut self, clock_name: String) {
        if let Some(clock) = self.clocks.get_mut(&clock_name) {
            clock.stop();
        }
    }

    fn log_clock_recursive(&self, key: String, prefix: String) {
        if let Some(current_clock) = self.clocks.get(&key) {
            current_clock.log(prefix.clone(), key);

            for child_key in current_clock.get_child_keys() {
                let child_key = child_key.clone();
                self.log_clock_recursive(child_key, "-->".to_string() + &prefix.clone());
            }
        }
    }
}

#[derive(Default)]
struct ClockData {
    ticks: usize,
    longest_tick: f64,
    last_time: f64,
    start_time: f64,
    parent_key: Option<String>,
    child_keys: Vec<String>,
    tick_history: Vec<f64>
}

impl ClockData {
    pub fn new_parent(parent_key: String) -> ClockData {
        ClockData {
            parent_key: Some(parent_key),
            ..Default::default()
        }
    }

    pub fn add_child_key(&mut self, child_key: String) {
        self.child_keys.push(child_key);
    }

    pub fn reset(&mut self) {
        self.ticks = 0;
        self.longest_tick = 0.0;
        self.last_time = time::precise_time_s();
        self.start_time = self.last_time;
        self.tick_history.clear();
    }

    pub fn start(&mut self) {
        let current_time = time::precise_time_s();
        
        if self.ticks == 0 {
            self.start_time = current_time;
        }
        
        self.last_time = current_time;
    }

    pub fn stop(&mut self) {
        let cycle_time = time::precise_time_s() - self.last_time;

        self.ticks += 1;
        self.tick_history.push(cycle_time);

        if cycle_time > self.longest_tick {
            self.longest_tick = cycle_time;
        }
    }

    pub fn log(&self, prefix: String, name: String) {
        let total_time: f64 = self.tick_history.iter().sum();
        debug(&format!("{}{}: Time Spent: {}; Ticks: {}; Longest Tick: {};",
                       prefix, get_last_name(name), total_time, self.ticks, self.longest_tick));
    }

    pub fn get_child_keys(&self) -> &Vec<String> {
        &self.child_keys
    }
}

fn get_parent_key(name: String) -> Option<String> {
    let mut name_components: Vec<String> = name.split("::").map(|s| {s.to_owned()}).collect();
    if name_components.len() == 1 {
        return None;
    }
    name_components.pop();
    let initial_name = name_components.remove(0);
    let parent_key = name_components.into_iter().fold(initial_name, |acc, name| {acc + "::" + &name});
    debug(&format!("name: {} Parent_key: {}", name, parent_key));
    Some(parent_key)
}

fn get_last_name(name: String) -> String {
    let mut name_components: Vec<String> = name.split("::").map(|s| {s.to_owned()}).collect();
    name_components.pop().unwrap()
}
