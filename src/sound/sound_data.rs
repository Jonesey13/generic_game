pub struct SoundData<S: Eq + Hash> {
    pub id: S,
    pub file: &'static[u8],
    pub volume: f32
}

impl<S: Eq + Hash> SoundData<S> {
    pub fn new(id: S, file: &'static[u8], volume: f32) -> Self {
        Self {
            id,
            file, 
            volume
        }
    }
}