#[derive(Copy, Clone, Default)]
pub struct MouseInput {
    pub movement: (i32, i32),
    pub left: bool,
    pub right: bool,
    pub middle: bool,
    pub button4: bool,
    pub button5: bool,
}
