use crate::geometry::*;

#[derive(Clone)]
pub struct BezierPatch {
    pub control: BezierQuad,
    pub vert_dir: Point,
    pub width: f64,
    pub pos: Point,
}

impl BezierPatch {
    pub fn get_subpatch(&self, sub_region: CartesianRectangle) -> BezierPatch {
        let x_bounds = sub_region.x_bounds();
        let sub_width = x_bounds.y - x_bounds.x;
        let x_mid = (x_bounds.y + x_bounds.x) / 2.0;
        let sub_pos = self.pos + x_mid * self.vert_dir;
        let sub_vert_dir = self.vert_dir;
        let y_bounds = sub_region.y_bounds();
        let sub_control = self.control.get_sub_bezier(y_bounds.x, y_bounds.y);

        BezierPatch {
            control: sub_control,
            vert_dir: sub_vert_dir,
            width: sub_width,
            pos: sub_pos
        }
    }
}

