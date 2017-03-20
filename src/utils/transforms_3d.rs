/// For use in shaders (projective space transforms)
use na::{Matrix4, Vector3, Vector4, dot, normalize};
use num::One;

/// Translation by the three_vec
pub fn translation_mat(three_vec: Vector3<f64>) -> Matrix4<f64>{
    return Matrix4::new(
        1.0, 0.0, 0.0, three_vec.x,
        0.0, 1.0, 0.0, three_vec.y,
        0.0, 0.0, 1.0, three_vec.z,
        0.0, 0.0, 0.0, 1.0
    );
}

/// Rotation by axis through angle theta
/// Follows the Rodrigues formula
pub fn rotation_mat(axis: Vector3<f64>, theta: f64) -> Matrix4<f64> {
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
    eye_position: Vector3<f64>,
    view_axis: Vector3<f64>,
    up_vec: Vector3<f64>,
) -> Matrix4<f64> {
    let correct_eye_position = translation_mat(eye_position * -1.0);

    let axis_reg = normalize(&view_axis);
    let ez = Vector3::new(0.0, 0.0, 1.0);
    let axis_to_ez_cross = axis_reg.cross(&ez);
    let axis_to_ez_angle = dot(&axis_reg, &ez);
    let correct_axis_rotation = rotation_mat(axis_to_ez_cross, axis_to_ez_angle);

    let up_vec_reg = normalize(&up_vec);
    let up_vec_rot_reg_4vec = correct_axis_rotation * Vector4::new(up_vec_reg.x, up_vec_reg.y, up_vec_reg.z, 1.0);
    let up_vec_rot_reg = Vector3::new(up_vec_rot_reg_4vec.x, up_vec_rot_reg_4vec.y, up_vec_rot_reg_4vec.z);
    let ey = Vector3::new(0.0, 1.0, 0.0);
    let up_vec_to_ey_cross = up_vec_rot_reg.cross(&ey);
    let up_vec_to_ey_angle = dot(&up_vec_rot_reg, &ey);
    let correct_up_vec_rotation = rotation_mat(up_vec_to_ey_cross, up_vec_to_ey_angle);

    let half_fov = fov / 2.0;
    let eye_distance = 1.0 / half_fov.tan();
    let correct_eye_distance = translation_mat(ez * eye_distance * -1.0);

    let persp_mat = perspective_mat(fov, aspect_ratio);

    return persp_mat
        * correct_eye_distance
        * correct_up_vec_rotation
        * correct_axis_rotation
        * correct_eye_position;
}
