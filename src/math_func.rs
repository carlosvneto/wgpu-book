#![allow(dead_code)]
use std::f32::consts::PI;

pub fn torus(u: f32, v: f32) -> [f32; 3] {
    let x = (1.0 + 0.3 * v.cos()) * u.cos();
    let y = 0.3 * v.sin();
    let z = (1.0 + 0.3 * v.cos()) * u.sin();
    [x, y, z]
}

pub fn sphere(u: f32, v: f32) -> [f32; 3] {
    let x = v.cos() * u.cos();
    let y = v.sin();
    let z = v.cos() * u.sin();
    [x, y, z]
}

pub fn breather(u: f32, v: f32) -> [f32; 3] {
    const A: f32 = 0.4; // where 0 < A < 1

    let de = A
        * ((1.0 - A * A) * ((A * u).cosh()).powf(2.0)
            + A * A * ((((1.0 - A * A).sqrt() * v).sin()).powf(2.0)));

    let x = -u + (2.0 * (1.0 - A * A) * (A * u).cosh() * (A * u).sinh()) / de;

    let y = (2.0
        * (1.0 - A * A).sqrt()
        * (A * u).cosh()
        * (-((1.0 - A * A).sqrt() * v.cos() * ((1.0 - A * A).sqrt() * v).cos())
            - v.sin() * ((1.0 - A * A).sqrt() * v).sin()))
        / de;

    let z = (2.0
        * (1.0 - A * A).sqrt()
        * (A * u).cosh()
        * (-((1.0 - A * A).sqrt() * v.sin() * ((1.0 - A * A).sqrt() * v).cos())
            + v.cos() * ((1.0 - A * A).sqrt() * v).sin()))
        / de;

    [x, y, z]
}

pub fn sievert_enneper(u: f32, v: f32) -> [f32; 3] {
    const A: f32 = 1.0;

    let pu = -u / (1.0 + A).sqrt() + (u.tan() * (1.0 + A).sqrt()).atan();
    let auv = 2.0 / (1.0 + A - A * v.sin() * v.sin() * u.cos() * u.cos());
    let ruv = auv * v.sin() * ((1.0 + 1.0 / A) * (1.0 + A * u.sin() * u.sin())).sqrt();

    let x = (((v / 2.0).tan()).ln() + (1.0 + A) * auv * v.cos()) / A.sqrt();
    let y = ruv * pu.cos();
    let z = ruv * pu.sin();

    [x, y, z]
}

pub fn seashell(u: f32, v: f32) -> [f32; 3] {
    let x = 2.0 * (-1.0 + (u / (6.0 * PI)).exp()) * u.sin() * (((v / 2.0).cos()).powf(2.0));

    let y = 1.0 - (u / (3.0 * PI)).exp() - v.sin() + (u / (6.0 * PI)).exp() * v.sin();

    let z = 2.0 * (1.0 - (u / (6.0 * PI)).exp()) * u.cos() * ((v / 2.0).cos()).powf(2.0);

    [x, y, z]
}

pub fn wellenkugel(u: f32, v: f32) -> [f32; 3] {
    let x = u * (u.cos()).cos() * v.sin();
    let y = u * (u.cos()).sin();
    let z = u * (u.cos()).cos() * v.cos();
    [x, y, z]
}

pub fn klein_bottle(u: f32, v: f32) -> [f32; 3] {
    let x = 2.0 / 15.0 * (3.0 + 5.0 * u.cos() * u.sin()) * v.sin();

    let y = -1.0 / 15.0
        * u.sin()
        * (3.0 * v.cos()
            - 3.0 * (u.cos()).powf(2.0) * v.cos()
            - 48.0 * (u.cos()).powf(4.0) * v.cos()
            + 48.0 * (u.cos()).powf(6.0) * v.cos()
            - 60.0 * u.sin()
            + 5.0 * u.cos() * v.cos() * u.sin()
            - 5.0 * (u.cos()).powf(3.0) * v.cos() * u.sin()
            - 80.0 * (u.cos()).powf(5.0) * v.cos() * u.sin()
            + 80.0 * (u.cos()).powf(7.0) * v.cos() * u.sin());

    let z = -2.0 / 15.0
        * u.cos()
        * (3.0 * v.cos() - 30.0 * u.sin() + 90.0 * (u.cos()).powf(4.0) * u.sin()
            - 60.0 * (u.cos()).powf(6.0) * u.sin()
            + 5.0 * u.cos() * v.cos() * u.sin());

    [x, y, z]
}

pub fn peaks(x: f32, z: f32) -> [f32; 3] {
    let y = 3.0 * (1.0 - x) * (1.0 - x) * (-(x * x) - (z + 1.0) * (z + 1.0)).exp()
        - 10.0 * (x / 5.0 - x * x * x - z * z * z * z * z) * (-x * x - z * z).exp()
        - 1.0 / 3.0 * (-(x + 1.0) * (x + 1.0) - z * z).exp();
    [x, y, z]
}

pub fn sinc(x: f32, z: f32) -> [f32; 3] {
    let r = (x * x + z * z).sqrt();
    let y = if r == 0.0 { 1.0 } else { r.sin() / r };
    [x, y, z]
}

pub fn torus_position(r_torus: f32, r_tube: f32, u: f32, v: f32) -> [f32; 3] {
    let x = (r_torus + r_tube * v.cos()) * u.cos();
    let y = r_tube * v.sin();
    let z = -(r_torus + r_tube * v.cos()) * u.sin();
    [x, y, z]
}

pub fn cylinder_position(r: f32, y: f32, theta: f32) -> [f32; 3] {
    [r * theta.cos(), y, -r * theta.sin()]
}

pub fn sphere_position(r: f32, theta: f32, phi: f32) -> [f32; 3] {
    let snt = theta.sin();
    let cnt = theta.cos();
    let snp = phi.sin();
    let cnp = phi.cos();
    [r * snt * cnp, r * cnt, -r * snt * snp]
}
