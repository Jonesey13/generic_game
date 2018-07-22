use multiinput::manager::{RawInputManager, DeviceType, XInputInclude,DeviceStats};
use multiinput::event::{RawEvent, KeyId, State, MouseButton, Axis};
use multiinput::devices::HatSwitch;
use std::collections::HashMap;
use games::GameInput;
use input;
use super::{InputHandler, bool_switch};

pub struct MultiInput {
    pub raw_states: RawStates,
    raw_manager: RawInputManager,
    escape_key_switch: bool_switch::BoolSwitch,
    backtick_key_switch: bool_switch::BoolSwitch,
    f8_key_switch: bool_switch::BoolSwitch,
    f9_key_switch: bool_switch::BoolSwitch
}

#[derive(Default)]
pub struct RawStates {
    pub key_states: HashMap<Key, bool>,
    pub mouse_button_states: HashMap<MouseAndButton, bool>,
    pub mouse_move_states: HashMap<usize, (i32, i32)>,
    pub joystick_button_states: HashMap<JoystickButton, bool>,
    pub joystick_axis_states: HashMap<JoystickAxis, f64>,
    pub joystick_hatswitch_states: HashMap<JoystickHatSwitch, HatSwitch>,
    device_stats: DeviceStats,
}

impl RawStates {
    pub fn flush(&mut self) {
        self.key_states.clear();
        self.mouse_button_states.clear();
        self.mouse_move_states.clear();
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Key(pub usize, pub KeyId);
#[derive(Eq, PartialEq, Hash)]
pub struct MouseAndButton(pub usize, pub MouseButton);
#[derive(Eq, PartialEq, Hash)]
pub struct JoystickButton(pub usize, pub usize);
#[derive(Eq, PartialEq, Hash)]
pub struct JoystickAxis(pub usize, pub Axis);
#[derive(Eq, PartialEq, Hash)]
pub struct JoystickHatSwitch(pub usize);

impl MultiInput {
    pub fn new() -> Self {
        let mut raw_manager = RawInputManager::new().unwrap();
        raw_manager.register_devices(DeviceType::Keyboards);
        raw_manager.register_devices(DeviceType::Mice);
        raw_manager.register_devices(DeviceType::Joysticks(XInputInclude::True));
        let raw_states = RawStates{
            device_stats: raw_manager.get_device_stats(),
            ..Default::default()
            };
        MultiInput {
            raw_states: raw_states,
            raw_manager: raw_manager,
            escape_key_switch: bool_switch::BoolSwitch::new(),
            backtick_key_switch: bool_switch::BoolSwitch::new(),
            f8_key_switch: bool_switch::BoolSwitch::new(),    
            f9_key_switch: bool_switch::BoolSwitch::new(),    
        }
    }
}

impl InputHandler for MultiInput {
    fn reset(&mut self) {
        self.raw_manager = RawInputManager::new().unwrap();
        self.raw_manager.register_devices(DeviceType::Keyboards);
        self.raw_manager.register_devices(DeviceType::Mice);
        self.raw_manager.register_devices(DeviceType::Joysticks(XInputInclude::True));
        self.raw_states = RawStates{
            device_stats: self.raw_manager.get_device_stats(),
            ..Default::default()
        };
    }

    fn receive_input(&mut self) {
        let raw_states = &mut self.raw_states;
        while let Some(event) = self.raw_manager.get_event() {
            match event {
                RawEvent::KeyboardEvent(num, key_id, state)
                    => {raw_states.key_states.insert(Key(num, key_id), state == State::Pressed);},
                RawEvent::MouseButtonEvent(num, mouse_button, state)
                    => {raw_states.mouse_button_states.insert(MouseAndButton(num, mouse_button), state == State::Pressed);},
                RawEvent::MouseMoveEvent(num, x, y)
                    => {raw_states.mouse_move_states.insert(num, (x, y));},
                RawEvent::JoystickButtonEvent(num, button_num, state)
                    => {raw_states.joystick_button_states.insert(JoystickButton(num, button_num), state == State::Pressed);},
                RawEvent::JoystickAxisEvent(num, axis, value)
                    => {raw_states.joystick_axis_states.insert(JoystickAxis(num, axis), value);},
                RawEvent::JoystickHatSwitchEvent(num, value)
                    => {raw_states.joystick_hatswitch_states.insert(JoystickHatSwitch(num), value);},
                _ => (),
            }
        }

        for index in 0..raw_states.device_stats.number_of_keyboards {
            if let Some(&state) = raw_states.key_states.get(&Key(index, KeyId::Escape)) {
                self.escape_key_switch.update_state(state);
            } else {
                self.escape_key_switch.clear_switch();
            }

            if let Some(&state) = raw_states.key_states.get(&Key(index, KeyId::BackTick)) {
                self.backtick_key_switch.update_state(state);
            } else {
                self.backtick_key_switch.clear_switch();
            }
            
            if let Some(&state) = raw_states.key_states.get(&Key(index, KeyId::F8)) {
                self.f8_key_switch.update_state(state);
            } else {
                self.f8_key_switch.clear_switch();
            }

            if let Some(&state) = raw_states.key_states.get(&Key(index, KeyId::F9)) {
                self.f9_key_switch.update_state(state);
            } else {
                self.f9_key_switch.clear_switch();
            }
        }
    }

    fn pass_on_input<'a>(&self, game_input: Option<&'a mut GameInput>) {
        if let Some(input) = game_input {
            if let Some(kbds) = input.get_kbd_inp() {
                for index in 0..self.raw_states.device_stats.number_of_keyboards {
                    if kbds.devices.iter().nth(index).is_none() {
                        kbds.devices.push(Default::default());
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Escape)) {
                        kbds.devices[index].escape = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Return)) {
                        kbds.devices[index].ret = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Backspace)) {
                        kbds.devices[index].backspace = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Space)) {
                        kbds.devices[index].space = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::LeftCtrl)) {
                        kbds.devices[index].leftctrl = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::RightCtrl)) {
                        kbds.devices[index].rightctrl = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::LeftAlt)) {
                        kbds.devices[index].leftalt = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::RightAlt)) {
                        kbds.devices[index].rightalt = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Left)) {
                        kbds.devices[index].left = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Right)) {
                        kbds.devices[index].right = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Up)) {
                        kbds.devices[index].up = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Down)) {
                        kbds.devices[index].down = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::A)) {
                        kbds.devices[index].a = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::B)) {
                        kbds.devices[index].b = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::C)) {
                        kbds.devices[index].c = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::D)) {
                        kbds.devices[index].d = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::E)) {
                        kbds.devices[index].e = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::F)) {
                        kbds.devices[index].f = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::G)) {
                        kbds.devices[index].g = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::H)) {
                        kbds.devices[index].h = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::I)) {
                        kbds.devices[index].i = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::J)) {
                        kbds.devices[index].j = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::K)) {
                        kbds.devices[index].k = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::L)) {
                        kbds.devices[index].l = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::M)) {
                        kbds.devices[index].m = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::N)) {
                        kbds.devices[index].n = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::O)) {
                        kbds.devices[index].o = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::P)) {
                        kbds.devices[index].p = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Q)) {
                        kbds.devices[index].q = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::R)) {
                        kbds.devices[index].r = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::S)) {
                        kbds.devices[index].s = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::T)) {
                        kbds.devices[index].t = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::U)) {
                        kbds.devices[index].u = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::V)) {
                        kbds.devices[index].v = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::W)) {
                        kbds.devices[index].w = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::X)) {
                        kbds.devices[index].x = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Y)) {
                        kbds.devices[index].y = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Z)) {
                        kbds.devices[index].z = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Pause)) {
                        kbds.devices[index].pause = val;
                    }
                }
            }

            if let Some(mice) = input.get_mouse_inp() {
                for mouse in &mut mice.devices {
                    mouse.movement = (0, 0);
                } 
                for index in 0..self.raw_states.device_stats.number_of_mice {
                    if mice.devices.iter().nth(index).is_none() {
                        mice.devices.push(Default::default());
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Left)) {
                        mice.devices[index].left = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Right)) {
                        mice.devices[index].right = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Middle)) {
                        mice.devices[index].middle = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Button4)) {
                        mice.devices[index].button4 = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Button5)) {
                        mice.devices[index].button5 = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_move_states.get(&index) {
                        mice.devices[index].movement = val;
                    }
                }
            }

            if let Some(joysticks) = input.get_joystick_inp() {
                for index in 0..self.raw_states.device_stats.number_of_joysticks {
                    if joysticks.devices.iter().nth(index).is_none() {
                        joysticks.devices.push(Default::default());
                    }

                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 0)) {
                        joysticks.devices[index].button_1 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 1)) {
                        joysticks.devices[index].button_2 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 2)) {
                        joysticks.devices[index].button_3 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 3)) {
                        joysticks.devices[index].button_4 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 4)) {
                        joysticks.devices[index].button_5 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 5)) {
                        joysticks.devices[index].button_6 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 6)) {
                        joysticks.devices[index].button_7 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 7)) {
                        joysticks.devices[index].button_8 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 8)) {
                        joysticks.devices[index].button_9 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 9)) {
                        joysticks.devices[index].button_10 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::X)) {
                        joysticks.devices[index].x_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::Y)) {
                        joysticks.devices[index].y_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::Z)) {
                        joysticks.devices[index].z_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::RX)) {
                        joysticks.devices[index].rx_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::RY)) {
                        joysticks.devices[index].ry_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::RZ)) {
                        joysticks.devices[index].rz_axis = val;
                    }
                    if let Some(&ref val) = self.raw_states.joystick_hatswitch_states.get(&JoystickHatSwitch(index)) {
                        let converted_val = match *val {
                            HatSwitch::Center => input::HatSwitch::Center,
                            HatSwitch::Up => input::HatSwitch::Up,
                            HatSwitch::UpRight => input::HatSwitch::UpRight,
                            HatSwitch::Right => input::HatSwitch::Right,
                            HatSwitch::DownRight => input::HatSwitch::DownRight,
                            HatSwitch::Down => input::HatSwitch::Down,
                            HatSwitch::DownLeft => input::HatSwitch::DownLeft,
                            HatSwitch::Left => input::HatSwitch::Left,
                            HatSwitch::UpLeft => input::HatSwitch::UpLeft,
                        };
                        joysticks.devices[index].hat_switch = converted_val;
                    }
                }
            }
        }
    }

    fn flush_input(&mut self) {
        self.raw_states.flush();
    }

    fn escape_key_pressed(&self) -> bool {
        self.escape_key_switch.pressed()
    }

    fn f8_key_pressed(&self) -> bool {
        self.f8_key_switch.pressed()
    }

    fn f9_key_pressed(&self) -> bool {
        self.f9_key_switch.pressed()
    }
    
    fn backtick_key_pressed(&self) -> bool {
        self.backtick_key_switch.pressed()
    }
}
