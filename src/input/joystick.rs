#[derive(Copy, Clone, Default)]
pub struct JoystickInput {
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

#[derive(Copy, Clone)]
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
