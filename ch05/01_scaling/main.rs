use glam::{Mat4 , Vec3, Vec4};

fn main() {
    //create original vector
    let my_vec = Vec4::new(1.0, 2.0, 3.0, 1.0);

    // create scale matrix
    let my_scale_vec = Vec3::new(0.5, 0.5, 1.5);
    let my_mat = Mat4::from_scale(my_scale_vec);

    // get the scaled vector
    let scaled_vec = my_mat * my_vec;

    println!("\nOriginal vector: \n{:?}", my_vec);
    println!("Scaling matrix: \n{:?}", my_mat);
    println!(
        "Vector after scaling: scaled_vec = my_mat * my_vec = \n{:?}",
        scaled_vec
    );

    // two successive scaling transforms:
    // get total scaling matrix after another scaling transformation:
    let scaling_vec = Vec3::new(1.0, 0.5, 0.3);
    let scaling_mat = my_mat * Mat4::from_scale(scaling_vec);

    // get final scaled vector
    let final_vec = scaling_mat * my_vec;

    println!("\nScaling matrix after two scalings: \n{:?}", scaling_mat);
    println!(
        "Vector after two scalings: scaled_vec = scaling_mat * my_vec = \n{:?}\n",
        final_vec
    );
}
