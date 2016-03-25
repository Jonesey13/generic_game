use multiinput::manager::{RawInputManager, DeviceType, DeviceStats};
use multiinput::event::{RawEvent, KeyId, State, MouseButton};
use std::collections::HashMap;
use games::GameInput;
use super::InputHandler;

pub struct MultiInput {
    pub raw_states: RawStates,
    raw_manager: RawInputManager,
    escape_key_flag: bool
}

#[derive(Default)]
pub struct RawStates {
    pub key_states: HashMap<Key, bool>,
    pub mouse_button_states: HashMap<MouseAndButton, bool>,
    pub mouse_move_states: HashMap<usize, (i32, i32)>,
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

impl MultiInput {
    pub fn new() -> Self {
        let mut raw_manager = RawInputManager::new().unwrap();
        raw_manager.register_devices(DeviceType::Keyboards);
        raw_manager.register_devices(DeviceType::Mice);
        let raw_states = RawStates{
            device_stats: raw_manager.get_device_stats(),
            ..Default::default()
            };
        MultiInput {
            raw_states: raw_states,
            raw_manager: raw_manager,
            escape_key_flag: false,
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
                _ => (),
            }
        }

        for index in 0..raw_states.device_stats.number_of_keyboards {
            if let Some(&state) = raw_states.key_states.get(&Key(index, KeyId::Escape)) {
                self.escape_key_flag = state;
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
        }
    }

    fn flush_input(&mut self) {
        self.raw_states.flush();
    }

    fn escape_key_pressed(&self) -> bool {
        self.escape_key_flag
    }
}
