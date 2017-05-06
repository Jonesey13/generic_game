/// For use in shaders (projective space transforms)
use na::{Matrix4, Matrix3, Vector2, Vector3, dot, normalize, Rotation2};
use num::One;
use std::f64::consts::PI;

/// Translation by the three_vec
pub fn translation_mat(two_vec: Vector2<f64>) -> Matrix3<f64>{
    return Matrix3::new(
        1.0, 0.0, two_vec.x,
        0.0, 1.0, two_vec.y,
        0.0, 0.0, 1.0
    );
}

/// Rotation by axis through angle theta
/// +ve theta <=> anticlockwise rotation
pub fn rotation_mat(theta: f64) -> Matrix3<f64> {
    Matrix3::new(
        theta.cos(), -theta.sin(), 0.0,
        theta.sin(), theta.cos(), 0.0,
        0.0, 0.0, 1.0,
    )
}

pub fn scaling_mat(scale: Vector2<f64>) -> Matrix3<f64> {
    Matrix3::new(
        scale.x, 0.0, 0.0,
        0.0, scale.y, 0.0,
        0.0, 0.0, 1.0
    )
}

pub fn build_worldview_mat(
    position: Vector2<f64>,
    view_height: f64,
    aspect_ratio: f64,
    up_vector: Vector2<f64>,
    use_aspect: bool
) -> Matrix4<f64> {
    let trans_mat = translation_mat(position * -1.0);
    let scaling = match use_aspect {
        true => Vector2::new(1.0 / aspect_ratio, 1.0) * (1.0 / view_height),
        false => Vector2::new(1.0, 1.0) * (1.0 / view_height)
    };
    let scale_mat = scaling_mat(scaling);

    let rotation_angle = (up_vector.y).atan2(up_vector.x) - PI / 2.0;
    let rot_mat = rotation_mat(rotation_angle * - 1.0);

    let three_mat = scale_mat * rot_mat * trans_mat;

    Matrix4::new(
        three_mat.m11, three_mat.m12, 0.0, three_mat.m13,
        three_mat.m21, three_mat.m22, 0.0, three_mat.m23,
        0.0, 0.0, 1.0, 0.0,
        three_mat.m31, three_mat.m32, 0.0, three_mat.m33,
    )
}
