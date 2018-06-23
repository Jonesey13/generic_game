#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }

    pub fn get_array(&self) -> [f64; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn get_array_f32(&self) -> [f32; 4] {
        [self.r as f32, self.g as f32, self.b as f32, self.a as f32]
    }

    pub fn zero() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    }
}

impl From<[f64; 4]> for Color {
    fn from(arr: [f64; 4]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3]
        }
    }
}