/// For use in shaders (projective space transforms)
use na::{Matrix4, Vector4};
use num::One;
use ::geometry::*;

/// Translation by the three_vec
pub fn translation_mat(three_vec: Point3) -> Matrix4<f64>{
    return Matrix4::new(
        1.0, 0.0, 0.0, three_vec.x,
        0.0, 1.0, 0.0, three_vec.y,
        0.0, 0.0, 1.0, three_vec.z,
        0.0, 0.0, 0.0, 1.0
    );
}

/// Rotation by axis through angle theta
/// Follows the Rodrigues formula
pub fn rotation_mat(axis: Point3, theta: f64) -> Matrix4<f64> {
    let identity_mat = Matrix4::<f64>::one();
    let cross_prod_mat = Matrix4::new(
        0.0, -axis.z, axis.y, 0.0,
        axis.z, 0.0, -axis.x, 0.0,
        -axis.y, axis.x, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    return identity_mat + cross_prod_mat * theta.sin() + cross_prod_mat * cross_prod_mat * (1.0 - theta.cos());
}

/// Assumes eye at (0.0, 0.0, cot(fov))
pub fn perspective_mat(fov: f64, aspect_ratio: f64) -> Matrix4<f64> {
    let half_fov = fov / 2.0;
    let eye_distance = 1.0 / half_fov.tan();
    
    return Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0 / aspect_ratio, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0 / eye_distance, 1.0
    );
}

pub fn build_worldview_mat(
    fov: f64,
    aspect_ratio: f64,
    eye_position: Point3,
    view_axis: Point3,
    up_vec: Point3,
) -> Matrix4<f64> {
    let correct_eye_position = translation_mat(-eye_position);

    let axis_reg = view_axis.normalized();
    let ez = Point3::new(0.0, 0.0, 1.0);
    let axis_to_ez_cross = axis_reg.cross(&ez);
    let axis_to_ez_angle = axis_reg.dot(&ez);
    let correct_axis_rotation = rotation_mat(axis_to_ez_cross, axis_to_ez_angle);

    let up_vec_reg = up_vec.normalized();
    let up_vec_rot_reg_4vec = correct_axis_rotation * Vector4::new(up_vec_reg.x, up_vec_reg.y, up_vec_reg.z, 1.0);
    let up_vec_rot_reg = Point3::new(up_vec_rot_reg_4vec.x, up_vec_rot_reg_4vec.y, up_vec_rot_reg_4vec.z);
    let ey = Point3::new(0.0, 1.0, 0.0);
    let up_vec_to_ey_cross = up_vec_rot_reg.cross(&ey);
    let up_vec_to_ey_angle = up_vec_rot_reg.dot(&ey);
    let correct_up_vec_rotation = rotation_mat(up_vec_to_ey_cross, up_vec_to_ey_angle);

    let half_fov = fov / 2.0;
    let eye_distance = 1.0 / half_fov.tan();
    let correct_eye_distance = translation_mat(-eye_distance * ez);

    let persp_mat = perspective_mat(fov, aspect_ratio);

    return persp_mat
        * correct_eye_distance
        * correct_up_vec_rotation
        * correct_axis_rotation
        * correct_eye_position;
}
