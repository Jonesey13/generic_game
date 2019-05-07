/// For use in shaders (projective space transforms)
use std::f64::consts::PI;
use crate::geometry::*;

/// Translation by the three_vec
pub fn translation_mat(two_vec: Point) -> Matrix3 {
    return Matrix3::new(
        1.0, 0.0, two_vec.x,
        0.0, 1.0, two_vec.y,
        0.0, 0.0, 1.0
    );
}

/// Rotation by axis through angle theta
/// +ve theta <=> anticlockwise rotation
pub fn rotation_mat(theta: f64) -> Matrix3 {
    Matrix3::new(
        theta.cos(), -theta.sin(), 0.0,
        theta.sin(), theta.cos(), 0.0,
        0.0, 0.0, 1.0,
    )
}

pub fn scaling_mat(scale: Point) -> Matrix3 {
    Matrix3::new(
        scale.x, 0.0, 0.0,
        0.0, scale.y, 0.0,
        0.0, 0.0, 1.0
    )
}

pub fn build_worldview_mat(
    position: Point,
    view_height: f64,
    view_length: f64,
    aspect_ratio: f64,
    up_vector: Point,
    use_aspect: bool
) -> Matrix4 {
    let trans_mat = translation_mat(-position);
    let scaling = match use_aspect {
        true => Point::new(1.0 / (aspect_ratio * view_length), 1.0 / view_height),
        false => Point::new(1.0 / view_length, 1.0 / view_height)
    };
    let scale_mat = scaling_mat(scaling);

    let rotation_angle = (up_vector.y).atan2(up_vector.x) - PI / 2.0;
    let rot_mat = rotation_mat(rotation_angle);

    let three_mat = scale_mat * rot_mat * trans_mat;

    Matrix4::new(
        three_mat.xx, three_mat.xy, 0.0, three_mat.xz,
        three_mat.yx, three_mat.yy, 0.0, three_mat.yz,
        0.0, 0.0, 1.0, 0.0,
        three_mat.zx, three_mat.zy, 0.0, three_mat.zz,
    )
}
