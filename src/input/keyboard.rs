#[derive(Clone)]
pub struct KeyboardInput {
    pub devices: Vec<KeyboardInputKeys>
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self {
            devices: vec![Default::default(); 8]
        }
    }
}

impl KeyboardInput {
    pub fn get_escape(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.escape})
    } 
    pub fn get_ret(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.ret})
    }
    pub fn get_backspace(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.backspace})
    }
    pub fn get_left(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.left})
    }
    pub fn get_right(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.right})
    }
    pub fn get_up(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.up})
    }
    pub fn get_down(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.down})
    }
    pub fn get_space(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.space})
    }
    pub fn get_a(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.a})
    }
    pub fn get_b(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.b})
    }
    pub fn get_c(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.c})
    }
    pub fn get_d(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.d})
    }
    pub fn get_e(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.e})
    }
    pub fn get_f(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.f})
    }
    pub fn get_g(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.g})
    }
    pub fn get_h(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.h})
    }
    pub fn get_i(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.i})
    }
    pub fn get_j(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.j})
    }
    pub fn get_k(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.k})
    }
    pub fn get_l(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.l})
    }
    pub fn get_m(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.m})
    }
    pub fn get_n(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.n})
    }
    pub fn get_o(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.o})
    }
    pub fn get_p(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.p})
    }
    pub fn get_q(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.q})
    }
    pub fn get_r(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.r})
    }
    pub fn get_s(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.s})
    }
    pub fn get_t(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.t})
    }
    pub fn get_u(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.u})
    }
    pub fn get_v(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.v})
    }
    pub fn get_w(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.w})
    }
    pub fn get_x(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.x})
    }
    pub fn get_y(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.y})
    }
    pub fn get_z(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.z})
    }
    pub fn get_zero(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.zero})
    }
    pub fn get_one(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.one})
    }
    pub fn get_two(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.two})
    }
    pub fn get_three(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.three})
    }
    pub fn get_four(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.four})
    }
    pub fn get_five(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.five})
    }
    pub fn get_six(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.six})
    }
    pub fn get_seven(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.seven})
    }
    pub fn get_eight(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.eight})
    }
    pub fn get_nine(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.nine})
    }
    pub fn get_shift(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.shift})
    }
    pub fn get_leftctrl(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.leftctrl})
    }
    pub fn get_rightctrl(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.rightctrl})
    }
    pub fn get_leftalt(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.leftalt})
    }
    pub fn get_rightalt(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.rightalt})
    }
    pub fn get_capslock(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.capslock})
    }
    pub fn get_pause(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.pause})
    }
    pub fn get_pageup(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.pageup})
    }
    pub fn get_pagedown(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.pagedown})
    }
    pub fn get_printscreen(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.printscreen})
    }
    pub fn get_insert(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.insert})
    }
    pub fn get_end(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.end})
    }
    pub fn get_home(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.home})
    }
    pub fn get_delete(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.delete})
    }
    pub fn get_add(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.add})
    }
    pub fn get_subtract(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.subtract})
    }
    pub fn get_multiply(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.multiply})
    }
    pub fn get_separator(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.separator})
    }
    pub fn get_decimal(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.decimal})
    }
    pub fn get_divide(&self) -> bool {
        self.devices.iter().fold(false, |acc, device| {acc || device.divide})
    }
}

#[derive(Copy, Clone, Default)]
pub struct KeyboardInputKeys {
    pub escape: bool,
    pub ret: bool,
    pub backspace: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub space: bool,
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
    pub g: bool,
    pub h: bool,
    pub i: bool,
    pub j: bool,
    pub k: bool,
    pub l: bool,
    pub m: bool,
    pub n: bool,
    pub o: bool,
    pub p: bool,
    pub q: bool,
    pub r: bool,
    pub s: bool,
    pub t: bool,
    pub u: bool,
    pub v: bool,
    pub w: bool,
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub zero: bool,
    pub one: bool,
    pub two: bool,
    pub three: bool,
    pub four: bool,
    pub five: bool,
    pub six: bool,
    pub seven: bool,
    pub eight: bool,
    pub nine: bool,
    pub shift: bool,
    pub leftctrl: bool,
    pub rightctrl: bool,
    pub leftalt: bool,
    pub rightalt: bool,
    pub capslock: bool,
    pub pause: bool,
    pub pageup: bool,
    pub pagedown: bool,
    pub printscreen: bool,
    pub insert: bool,
    pub end: bool,
    pub home: bool,
    pub delete: bool,
    pub add: bool,
    pub subtract: bool,
    pub multiply: bool,
    pub separator: bool,
    pub decimal: bool,
    pub divide: bool,
    pub backslash: bool,
    pub forwardslash: bool,
    pub plus: bool,
    pub minus: bool,
    pub fullstop: bool,
    pub comma: bool,
    pub tab: bool,
    pub numlock: bool,
    pub leftsquarebracket: bool,
    pub rightsquarebracket: bool,
    pub semicolon: bool,
    pub apostrophe: bool,
    pub hash: bool,
}
