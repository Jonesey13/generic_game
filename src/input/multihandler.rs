use multiinput::manager::{RawInputManager, DeviceType, DeviceStats};
use multiinput::event::{RawEvent, KeyId, State, MouseButton};
use std::collections::HashMap;
use super::InputHandler;

pub struct MultiInput {
    pub raw_states: RawStates,
    raw_manager: RawInputManager
}

#[derive(Default)]
pub struct RawStates {
    pub key_states: HashMap<Key, bool>,
    pub mouse_button_states: HashMap<MouseAndButton, bool>,
    pub mouse_move_states: HashMap<usize, (i32, i32)>,
    device_stats: DeviceStats,
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
    }

    fn escape_key_pressed(&self) -> bool {
        for index in 0..self.raw_states.device_stats.number_of_keyboards {
            if let Some(&true) = self.raw_states.key_states.get(&Key(index, KeyId::Escape)) {
                return true;
            }
        }
        return false;

    }
}
