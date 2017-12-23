#[derive(Clone, Copy, Debug)]
pub struct DisplaySettings
{
    pub res: (u32, u32),
    pub fullscreen: bool,
    pub multisample_level: u16,
    pub text_glyph_detail: f32
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            res: (1280, 720),
            fullscreen: false,
            multisample_level: 2,
            text_glyph_detail: 128.0
        }
    }
}