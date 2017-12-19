#[derive(Debug, Clone)]
pub struct BoolSwitch {
    press_state: PressState, // Current state of button
    switch_flag: Option<PressState> // Indicates if a press has just happened
}

impl BoolSwitch {
    pub fn new() -> Self {
        BoolSwitch {
            press_state: PressState::Released,
            switch_flag: None
        }
    }

    pub fn update_state(&mut self, poll_state: bool) {
        match (poll_state, self.press_state) {
            (true, PressState::Released) => {self.press_state = PressState::Pressed; self.switch_flag = Some(PressState::Pressed);},
            (false, PressState::Pressed) => {self.press_state = PressState::Released; self.switch_flag = Some(PressState::Released);},
            (true, _) => {self.press_state = PressState::Pressed; self.switch_flag = None;},
            (false, _) => {self.press_state = PressState::Released; self.switch_flag = None;}
        }
    }

    pub fn switched(&self) -> Option<PressState> {
        self.switch_flag
    }

    pub fn pressed(&self) -> bool {
        if let Some(state) = self.switched() {
            return state == PressState::Pressed;
        }
        false
    }

    // Use as a press/release state has been read
    pub fn clear_switch(&mut self) {
        self.switch_flag = None
    }
    
    pub fn released(&self) -> bool {
        if let Some(state) = self.switched() {
            return state == PressState::Released;
        }
        false
    }
}

impl Default for BoolSwitch {
    fn default() -> BoolSwitch {
        BoolSwitch {
            press_state: PressState::Released,
            switch_flag: None
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PressState {
    Pressed,
    Released
}