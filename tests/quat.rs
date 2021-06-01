use approx::abs_diff_eq;
use std::f32::consts::PI;

use mikpe_math::{Quat, Vec3};

#[test]
fn test_quat_identity() {
    let quat = Quat::new();
    let vec = Vec3::new(1.0, 1.0, 1.0);
    let rotated_vec = quat.rotate_vec3(vec);
    assert_eq!(vec[0], rotated_vec[0]);
    assert_eq!(vec[1], rotated_vec[1]);
    assert_eq!(vec[2], rotated_vec[2]);
}

#[test]
fn test_quat_rotate() {
    let x_axis = Vec3::new(1.0, 0.0, 0.0);
    let quat = Quat::new_from_axis_angle(&x_axis, PI);
    let vec = Vec3::new(1.0, 1.0, 1.0);
    let rotated_vec = quat.rotate_vec3(vec);
    let _ = abs_diff_eq!(rotated_vec[0], 1.0, epsilon = 0.0001);
    let _ = abs_diff_eq!(rotated_vec[1], -1.0, epsilon = 0.0001);
    let _ = abs_diff_eq!(rotated_vec[2], -1.0, epsilon = 0.0001);
}
