use std::hash::Hash;
use rodio::{Decoder, Sink};
use std::io::BufReader;
use std::io::Cursor;
use crate::sound::*;

pub struct SoundBank<S: Eq + Hash> {
    data: SoundData<S>,
}

impl<S: Eq + Hash> SoundBank<S> {
    pub fn new(data: SoundData<S>) -> Self {
        Self {
            data,
        }
    }

    pub fn play(&mut self, sink: &mut Sink) {
        let decoder = Decoder::new(BufReader::new(Cursor::new(self.data.file))).unwrap();

        sink.append(decoder);

        sink.set_volume(self.data.volume);
        
        sink.play();
    }
}