#[derive(Clone)]
pub struct MouseInput {
    pub devices: Vec<MouseInputKeys>
}

impl Default for MouseInput {
    fn default() -> Self {
        Self {
            devices: vec![Default::default(); 8]
        }
    }
}

impl MouseInput {
    pub fn button_pressed(&self, device_index: usize) -> bool {
        let current = self.devices[device_index];

        current.left ||
        current.middle ||
        current.right ||
        current.button4 ||
        current.button5 
    }

    pub fn get_left_button(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.left})
    }
    pub fn get_middle_button(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.middle})
    }
    pub fn get_right_button(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.right})
    }
    pub fn get_button4(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button4})
    }
    pub fn get_button5(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button5})
    }
}

#[derive(Copy, Clone, Default)]
pub struct MouseInputKeys {
    pub movement: (i32, i32),
    pub left: bool,
    pub right: bool,
    pub middle: bool,
    pub button4: bool,
    pub button5: bool,
}
