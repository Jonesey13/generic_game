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

    pub fn new_rgba_comp(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
            a: a as f64 / 255.0,
        }
    }

    pub fn new_rgba(rgba: u32) -> Self {
        let mut rem = rgba;
        let a = (rem % 256) as u8;
        rem = rem / 256;
        let b = (rem % 256) as u8;
        rem = rem / 256;
        let g = (rem % 256) as u8;
        rem = rem / 256;
        let r = (rem % 256) as u8;

        Self::new_rgba_comp(r, g, b, a)
    }

    pub fn new_rgb(rgb: u32) -> Self {
        let mut rem = rgb;
        let b = (rem % 256) as u8;
        rem = rem / 256;
        let g = (rem % 256) as u8;
        rem = rem / 256;
        let r = (rem % 256) as u8;

        Self::new_rgba_comp(r, g, b, 0xFF)
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

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0
        }
    }

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r, 
            g: self.g + other.g, 
            b: self.b + other.b, 
            a: self.a + other.a, 
        }
    }

    fn multiply(self, t: f64) -> Self {
        Self {
            r: t * self.r,
            g: t * self.g,
            b: t * self.b,
            a: t * self.a,
        }
    }

    pub fn interpolate(self, other: Self, t: f64) -> Self {
        self.multiply(1.0 - t).add(other.multiply(t))
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