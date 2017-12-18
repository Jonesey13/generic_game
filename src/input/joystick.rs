#[derive(Clone)]
pub struct JoystickInput {
    pub devices: Vec<JoystickInputKeys>
}

impl Default for JoystickInput {
    fn default() -> Self {
        Self {
            devices: vec![Default::default(); 8]
        }
    }
}

impl JoystickInput {
    pub fn get_button_1(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_1})
    } 
    pub fn get_button_2(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_2})
    }
    pub fn get_button_3(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_3})
    }
    pub fn get_button_4(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_4})
    } 
    pub fn get_button_5(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_5})
    } 
    pub fn get_button_6(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_6})
    } 
    pub fn get_button_7(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_7})
    } 
    pub fn get_button_8(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_8})
    } 
    pub fn get_button_9(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_9})
    }
    pub fn get_button_10(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.button_10})
    }
    pub fn get_x_axis(&self) -> f64 {
        self.devices.iter().fold(0.0, |acc, device| {if device.x_axis.abs() > acc.abs() {device.x_axis} else {0.0}})
    }
    pub fn get_y_axis(&self) -> f64 {
        self.devices.iter().fold(0.0, |acc, device| {if device.y_axis.abs() > acc.abs() {device.y_axis} else {0.0}})
    }
    pub fn get_z_axis(&self) -> f64 {
        self.devices.iter().fold(0.0, |acc, device| {if device.z_axis.abs() > acc.abs() {device.z_axis} else {0.0}})
    }
    pub fn get_rx_axis(&self) -> f64 {
        self.devices.iter().fold(0.0, |acc, device| {if device.rx_axis.abs() > acc.abs() {device.rx_axis} else {0.0}})
    }
    pub fn get_ry_axis(&self) -> f64 {
        self.devices.iter().fold(0.0, |acc, device| {if device.ry_axis.abs() > acc.abs() {device.ry_axis} else {0.0}})
    }
    pub fn get_rz_axis(&self) -> f64 {
        self.devices.iter().fold(0.0, |acc, device| {if device.rz_axis.abs() > acc.abs() {device.rz_axis} else {0.0}})
    }
    pub fn get_hat_switch(&self) -> HatSwitch {
        self.devices.iter().fold(HatSwitch::Center, |acc, device| {if device.hat_switch != HatSwitch::Center {device.hat_switch} else {acc}})
    }
}

#[derive(Copy, Clone, Default)]
pub struct JoystickInputKeys {
    pub button_1: bool,
    pub button_2: bool,
    pub button_3: bool,
    pub button_4: bool,
    pub button_5: bool,
    pub button_6: bool,
    pub button_7: bool,
    pub button_8: bool,
    pub button_9: bool,
    pub button_10: bool,
    pub x_axis: f64,
    pub y_axis: f64,
    pub z_axis: f64,
    pub rx_axis: f64,
    pub ry_axis: f64,
    pub rz_axis: f64,
    pub hat_switch: HatSwitch
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HatSwitch {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Center
}

impl Default for HatSwitch {
    fn default() -> Self {
        HatSwitch::Center
    }
}
