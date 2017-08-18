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
        }
    }
}

impl InputHandler for MultiInput {
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
            }
            else {
                self.escape_key_switch.clear_switch();
            }
        }
        for index in 0..raw_states.device_stats.number_of_keyboards {
            if let Some(&state) = raw_states.key_states.get(&Key(index, KeyId::BackTick)) {
                self.backtick_key_switch.update_state(state);
            }
            else {
                self.backtick_key_switch.clear_switch();
            }
        }
    }

    fn pass_on_input<'a>(&self, game_input: Option<&'a mut GameInput>) {
        if let Some(input) = game_input {
            if let Some(kbd) = input.get_kbd_inp() {
                for index in 0..self.raw_states.device_stats.number_of_keyboards {
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Escape)) {
                        kbd.escape = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Left)) {
                        kbd.left = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Right)) {
                        kbd.right = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Up)) {
                        kbd.up = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Down)) {
                        kbd.down = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::A)) {
                        kbd.a = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::B)) {
                        kbd.b = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::C)) {
                        kbd.c = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::D)) {
                        kbd.d = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::E)) {
                        kbd.e = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::F)) {
                        kbd.f = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::G)) {
                        kbd.g = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::H)) {
                        kbd.h = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::I)) {
                        kbd.i = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::J)) {
                        kbd.j = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::K)) {
                        kbd.k = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::L)) {
                        kbd.l = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::M)) {
                        kbd.m = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::N)) {
                        kbd.n = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::O)) {
                        kbd.o = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::P)) {
                        kbd.p = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Q)) {
                        kbd.q = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::R)) {
                        kbd.r = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::S)) {
                        kbd.s = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::T)) {
                        kbd.t = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::U)) {
                        kbd.u = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::V)) {
                        kbd.v = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::W)) {
                        kbd.w = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::X)) {
                        kbd.x = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Y)) {
                        kbd.y = val;
                    }
                    if let Some(&val) = self.raw_states.key_states.get(&Key(index, KeyId::Z)) {
                        kbd.z = val;
                    }
                }
            }


            if let Some(mouse) = input.get_mouse_inp() {
                mouse.movement = (0, 0);
                for index in 0..self.raw_states.device_stats.number_of_mice {
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Left)) {
                        mouse.left = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Right)) {
                        mouse.left = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Middle)) {
                        mouse.middle = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Button4)) {
                        mouse.button4 = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_button_states.get(&MouseAndButton(index, MouseButton::Button5)) {
                        mouse.button5 = val;
                    }
                    if let Some(&val) = self.raw_states.mouse_move_states.get(&index) {
                        mouse.movement = val;
                    }
                }
            }

            if let Some(joystick) = input.get_joystick_inp() {
                for index in 0..self.raw_states.device_stats.number_of_joysticks {
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 0)) {
                        joystick.button_1 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 1)) {
                        joystick.button_2 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 2)) {
                        joystick.button_3 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 3)) {
                        joystick.button_4 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 4)) {
                        joystick.button_5 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 5)) {
                        joystick.button_6 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 6)) {
                        joystick.button_7 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 7)) {
                        joystick.button_8 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 8)) {
                        joystick.button_9 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_button_states.get(&JoystickButton(index, 9)) {
                        joystick.button_10 = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::X)) {
                        joystick.x_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::Y)) {
                        joystick.y_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::Z)) {
                        joystick.z_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::RX)) {
                        joystick.rx_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::RY)) {
                        joystick.ry_axis = val;
                    }
                    if let Some(&val) = self.raw_states.joystick_axis_states.get(&JoystickAxis(index, Axis::RZ)) {
                        joystick.rz_axis = val;
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
                        joystick.hat_switch = converted_val;
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

    fn backtick_key_pressed(&self) -> bool {
        self.backtick_key_switch.pressed()
    }
}
