use glam::{Mat4, Vec3};
use std::f32::consts::PI;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array_2d(&[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0], // Z scaling
    [0.0, 0.0, 0.5, 1.0], // Z translation
]);

pub fn create_projection(aspect: f32, is_perspective: bool) -> Mat4 {
    let project_mat: Mat4;
    if is_perspective {
        project_mat = OPENGL_TO_WGPU_MATRIX * Mat4::perspective_rh(2.0 * PI / 5.0, aspect, 0.1, 100.0);
    } else {
        project_mat = OPENGL_TO_WGPU_MATRIX * Mat4::orthographic_rh(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0);
    }
    project_mat
}

pub fn create_view_projection_ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
    camera_position: Vec3,
    look_direction: Vec3,
    up_direction: Vec3) -> (Mat4, Mat4, Mat4) {

    // construct view matrix
    let view_mat = Mat4::look_at_rh(camera_position, look_direction, up_direction);

    // construct projection matrix
    let project_mat = OPENGL_TO_WGPU_MATRIX * Mat4::orthographic_rh(left, right, bottom, top, near, far);

    // contruct view-projection matrix
    let view_project_mat = project_mat * view_mat;

    // return various matrices
    (view_mat, project_mat, view_project_mat)
}

pub fn create_view_projection(
    camera_position: Vec3,
    look_direction: Vec3,
    up_direction: Vec3,
    aspect: f32,
    is_perspective: bool,
) -> (Mat4, Mat4, Mat4) {
    // construct view matrix
    let view_mat = Mat4::look_at_rh(camera_position, look_direction, up_direction);

    // construct projection matrix
    let project_mat: Mat4;
    if is_perspective {
        project_mat = OPENGL_TO_WGPU_MATRIX * Mat4::perspective_rh(2.0 * PI / 5.0, aspect, 0.1, 100.0);
    } else {
        project_mat = OPENGL_TO_WGPU_MATRIX * Mat4::orthographic_rh(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0);
    }

    // contruct view-projection matrix
    let view_project_mat = project_mat * view_mat;

    // return various matrices
    (view_mat, project_mat, view_project_mat)
}

pub fn create_transforms(
    translation: [f32; 3],
    rotation: [f32; 3],
    scaling: [f32; 3],
) -> Mat4 {
    // create transformation matrices
    let trans_mat =
        Mat4::from_translation(Vec3::new(translation[0], translation[1], translation[2]));
    let rotate_mat_x = Mat4::from_axis_angle(Vec3::X, rotation[0]);
    let rotate_mat_y = Mat4::from_axis_angle(Vec3::Y, rotation[1]);
    let rotate_mat_z = Mat4::from_axis_angle(Vec3::Z, rotation[2]);
    let scale_vec = Vec3::new(scaling[0], scaling[1], scaling[2]);
    let scale_mat = Mat4::from_scale(scale_vec);

    // combine all transformation matrices together to form a final transform matrix: model matrix
    let model_mat = trans_mat * rotate_mat_z * rotate_mat_y * rotate_mat_x * scale_mat;

    // return final model matrix
    model_mat
}
