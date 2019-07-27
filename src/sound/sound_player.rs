use std::hash::Hash;
use rodio::{Sink, Device};
use std::collections::HashMap;
use crate::sound::*;

pub struct SoundPlayer<S: Eq + Hash> {
    sinks: Vec<Sink>,
    device: Device,
    sound_effect_decoders: HashMap<S, SoundBank<S>>,
    _music_decoder: Option<SoundBank<S>>,
}

impl<'a, S: Eq + Hash + Copy> SoundPlayer<S> {
    pub fn new(sound_list: Vec<SoundData<S>>) -> Self {
        let device = rodio::default_output_device().unwrap();

        Self {
            sinks: vec![Sink::new(&device)],
            device,
            sound_effect_decoders: Self::build_sound_decoders(sound_list),
            _music_decoder: None,
        }
    }

    fn build_sound_decoders(sound_list: Vec<SoundData<S>>) -> HashMap<S, SoundBank<S>> {
        let mut output: HashMap<S, SoundBank<S>> = HashMap::new();

        for sound in sound_list {
            output.insert(sound.id, SoundBank::new(sound));
        }

        output
    }

    pub fn process_sounds(&mut self, sounds: Vec<S>) {
        for sound in sounds {
            self.process_sound(sound)
        }
    }

    fn process_sound(&mut self, sound: S) {
        let next_available_sink = Self::get_next_available_sink(&self.device, &mut self.sinks);
        self.sound_effect_decoders.get_mut(&sound).unwrap().play(next_available_sink);
    }

    fn get_next_available_sink(device: &Device, sinks: &'a mut Vec<Sink>) -> &'a mut Sink {
        let free_sink_pos = sinks.iter_mut().position(|sink| {sink.len() == 0});

        if free_sink_pos.is_none() {
            sinks.push(Sink::new(device));

            return sinks.last_mut().unwrap();
        } else {
            return sinks.iter_mut().nth(free_sink_pos.unwrap()).unwrap();
        }
    }
}