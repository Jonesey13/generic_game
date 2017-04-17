#[derive(Copy, Clone, Debug)]
pub struct PolarViewDetails {
    pub rotation_angle: f64,
    pub radial_shift: f64,
    pub tunnel_mode: bool,
    pub length_circle: f64,
    pub length_total: f64
}

impl Default for PolarViewDetails {
    fn default() -> Self {
        PolarViewDetails {
            rotation_angle: 0.0,
            radial_shift: 0.0,
            tunnel_mode: true,
            length_circle: 1.0,
            length_total: 1.0
        }
    }
}
