use glam::{Mat4, Vec3, Vec4};
use std::f32::consts::PI;

fn main() {
    // create original vector
    let my_vec = Vec4::new(1.0, 2.0, 3.0, 1.0);

    // create a rotation matrix around the z axis by 20 degrees:
    let rot_mat_z = Mat4::from_axis_angle(Vec3::Z, 20.0 * PI / 180.0);

    // get total rotation matrix after another rotation around the z axis by 25 degrees
    let rot_mat = rot_mat_z * Mat4::from_axis_angle(Vec3::Z, 25.0 * PI / 180.0);

    // get final rotated vector
    let rot_vec = rot_mat * my_vec;

    println!("\nOriginal vector: my_vec = \n{:?}", my_vec);
    println!(
        "Total rotation matrix after two rotations: rot_mat = \n{:?}",
        rot_mat
    );
    println!(
        "Vector after two rotations: rot_vec = rot_mat * my_vec = \n{:?}\n",
        rot_vec
    );
}
