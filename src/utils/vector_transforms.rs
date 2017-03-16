/// For use in shaders (projective space transforms)
use na::{Mat4, Vec3};
use num::One;

/// Translation by the three_vec
pub fn translation_mat(three_vec: Vec3<f64>) -> Mat4<f64>{
    return Mat4::new(
        1.0, 0.0, 0.0, three_vec.x,
        0.0, 1.0, 0.0, three_vec.y,
        0.0, 0.0, 1.0, three_vec.z,
        0.0, 0.0, 0.0, 1.0
    );
}

/// Rotation by axis through angle theta
/// Follows the Rodrigues formula
pub fn rotation_mat(axis: Vec3<f64>, theta: f64) -> Mat4<f64> {
    let identity_mat = Mat4::<f64>::one();
    let cross_prod_mat = Mat4::new(
        0.0, -axis.z, axis.y, 0.0,
        axis.z, 0.0, -axis.x, 0.0,
        -axis.y, axis.x, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    return identity_mat + cross_prod_mat * theta.sin() + cross_prod_mat * cross_prod_mat * (1.0 - theta.cos());
}

/// Assumes eye at (0.0, 0.0, cot(fov))
pub fn perspective_mat(fov: f64, aspect_ratio: f64) -> Mat4<f64> {


}
