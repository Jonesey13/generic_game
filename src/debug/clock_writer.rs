use debug::*;

#[derive(Default)]
pub struct ClockWriter {
    pub clocks: HashMap<String, ClockData>,   
}

impl ClockWriter {
    pub fn log_clocks(&mut self) {
        

        
    }
}

#[derive(Default)]
struct ClockData {
    ticks: usize,
    longest_tick: f64,
    parent_key: Option<String>,
    child_keys: Option<Vec<String>>,
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
    }

    pub fn log(&self, prefix: String, name: String) {
        debug(&format!("{} {}: Ticks Per Second: {}; Longest Tick {};", prefix, name, self.ticks, self.longest_tick));
    }

    pub fn get_child_keys(&self) -> &Option<Vec<String>> {
        &self.child_keys
    }
}
