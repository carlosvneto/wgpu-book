#![allow(dead_code)]

use crate::math_func;
use glam::Vec3;
use std::f32::consts::PI;

pub fn torus_data(
    r_torus: f32,
    r_tube: f32,
    n_torus: usize,
    n_tube: usize,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let mut positions: Vec<[f32; 3]> =
        Vec::with_capacity((4 * (n_torus - 1) * (n_tube - 1)) as usize);
    let mut normals: Vec<[f32; 3]> =
        Vec::with_capacity((4 * (n_torus - 1) * (n_tube - 1)) as usize);
    let uvs: Vec<[f32; 3]> = Vec::with_capacity((4 * (n_torus - 1) * (n_tube - 1)) as usize);

    for i in 0..n_torus - 1 {
        for j in 0..n_tube - 1 {
            let u = i as f32 * 360.0 / (n_torus as f32 - 1.0);
            let v = j as f32 * 360.0 / (n_tube as f32 - 1.0);
            let u1 = (i as f32 + 1.0) * 360.0 / (n_torus as f32 - 1.0);
            let v1 = (j as f32 + 1.0) * 360.0 / (n_tube as f32 - 1.0);
            let p0 = math_func::torus_position(r_torus, r_tube, u.to_radians(), v.to_radians());
            let p1 = math_func::torus_position(r_torus, r_tube, u1.to_radians(), v.to_radians());
            let p2 = math_func::torus_position(r_torus, r_tube, u1.to_radians(), v1.to_radians());
            let p3 = math_func::torus_position(r_torus, r_tube, u.to_radians(), v1.to_radians());

            // positions
            positions.push(p0);
            positions.push(p1);
            positions.push(p2);
            positions.push(p2);
            positions.push(p3);
            positions.push(p0);

            // normals
            let ca = Vec3::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);
            let db = Vec3::new(p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]);
            let cp = (ca.cross(db)).normalize();

            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
        }
    }
    (positions, normals, uvs)
}

pub fn cone_data(
    rtop: f32,
    rbottom: f32,
    height: f32,
    n: usize,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let h = height / 2.0;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(12 * (n - 1));
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(12 * (n - 1));
    let uvs: Vec<[f32; 3]> = Vec::with_capacity(12 * (n - 1));

    for i in 0..n - 1 {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta1 = (i as f32 + 1.0) * 360.0 / (n as f32 - 1.0);
        let p0 = math_func::cylinder_position(rtop, h, theta.to_radians());
        let p1 = math_func::cylinder_position(rbottom, -h, theta.to_radians());
        let p2 = math_func::cylinder_position(0.0, -h, theta.to_radians());
        let p3 = math_func::cylinder_position(0.0, h, theta.to_radians());
        let p4 = math_func::cylinder_position(rtop, h, theta1.to_radians());
        let p5 = math_func::cylinder_position(rbottom, -h, theta1.to_radians());

        // positions

        // top face
        positions.push(p0);
        positions.push(p4);
        positions.push(p3);

        // bottom face
        positions.push(p1);
        positions.push(p2);
        positions.push(p5);

        // outer face
        positions.push(p0);
        positions.push(p1);
        positions.push(p5);
        positions.push(p5);
        positions.push(p4);
        positions.push(p0);

        // normals
        let ca = Vec3::new(p5[0] - p0[0], p5[1] - p0[1], p5[2] - p0[2]);
        let db = Vec3::new(p4[0] - p1[0], p4[1] - p1[1], p4[2] - p1[2]);
        let cp = (ca.cross(db)).normalize();

        // top face
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);

        // bottom face
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);

        // outer face
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
    }
    (positions, normals, uvs)
}

pub fn cylinder_data_uv(
    rin: f32,
    rout: f32,
    height: f32,
    n: usize,
    ul: f32,
    vl: f32,
) -> Vec<[f32; 2]> {
    let h = height / 2.0;
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(24 * (n - 1));

    for i in 0..n - 1 {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta1 = (i as f32 + 1.0) * 360.0 / (n as f32 - 1.0);
        let p0 = math_func::cylinder_position(rout, h, theta.to_radians());
        let p1 = math_func::cylinder_position(rout, -h, theta.to_radians());
        let p2 = math_func::cylinder_position(rin, -h, theta.to_radians());
        let p3 = math_func::cylinder_position(rin, h, theta.to_radians());
        let p4 = math_func::cylinder_position(rout, h, theta1.to_radians());
        let p5 = math_func::cylinder_position(rout, -h, theta1.to_radians());
        let p6 = math_func::cylinder_position(rin, -h, theta1.to_radians());
        let p7 = math_func::cylinder_position(rin, h, theta1.to_radians());

        let u0 = ul * (0.5 + (p0[0] / rout).atan2(p0[2] / rout) / PI / 2.0);
        let u1 = ul * (0.5 + (p1[0] / rout).atan2(p1[2] / rout) / PI / 2.0);
        let u2 = ul * (0.5 + (p2[0] / rin).atan2(p2[2] / rin) / PI / 2.0);
        let u3 = ul * (0.5 + (p3[0] / rin).atan2(p3[2] / rin) / PI / 2.0);
        let u4 = ul * (0.5 + (p4[0] / rout).atan2(p4[2] / rout) / PI / 2.0);
        let u5 = ul * (0.5 + (p5[0] / rout).atan2(p5[2] / rout) / PI / 2.0);
        let u6 = ul * (0.5 + (p6[0] / rin).atan2(p6[2] / rin) / PI / 2.0);
        let u7 = ul * (0.5 + (p7[0] / rin).atan2(p7[2] / rin) / PI / 2.0);

        let vt = vl * (rout - rin) / height;
        let u2i = u2 * rin / rout;
        let u3i = u3 * rin / rout;
        let u6i = u6 * rin / rout;
        let u7i = u7 * rin / rout;

        // top face
        uvs.push([u0, vt]);
        uvs.push([u4, vt]);
        uvs.push([u7, 0.0]);
        uvs.push([u7, 0.0]);
        uvs.push([u3, 0.0]);
        uvs.push([u0, vt]);

        // bottom face
        uvs.push([u1, vt]);
        uvs.push([u2, 0.0]);
        uvs.push([u6, 0.0]);
        uvs.push([u6, 0.0]);
        uvs.push([u5, vt]);
        uvs.push([u1, vt]);

        //outer face
        uvs.push([u0, vl]);
        uvs.push([u1, 0.0]);
        uvs.push([u5, 0.0]);
        uvs.push([u5, 0.0]);
        uvs.push([u4, vl]);
        uvs.push([u0, vl]);

        // inner face
        uvs.push([u2i, 0.0]);
        uvs.push([u3i, vl]);
        uvs.push([u7i, vl]);
        uvs.push([u7i, vl]);
        uvs.push([u6i, 0.0]);
        uvs.push([u2i, 0.0]);
    }

    uvs
}

pub fn cylinder_data(
    rin: f32,
    rout: f32,
    height: f32,
    n: usize,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let h = height / 2.0;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(24 * (n - 1));
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(24 * (n - 1));
    let uvs: Vec<[f32; 2]> = Vec::with_capacity(24 * (n - 1));

    for i in 0..n - 1 {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta1 = (i as f32 + 1.0) * 360.0 / (n as f32 - 1.0);
        let p0 = math_func::cylinder_position(rout, h, theta.to_radians());
        let p1 = math_func::cylinder_position(rout, -h, theta.to_radians());
        let p2 = math_func::cylinder_position(rin, -h, theta.to_radians());
        let p3 = math_func::cylinder_position(rin, h, theta.to_radians());
        let p4 = math_func::cylinder_position(rout, h, theta1.to_radians());
        let p5 = math_func::cylinder_position(rout, -h, theta1.to_radians());
        let p6 = math_func::cylinder_position(rin, -h, theta1.to_radians());
        let p7 = math_func::cylinder_position(rin, h, theta1.to_radians());

        // positions

        // top face
        positions.push(p0);
        positions.push(p4);
        positions.push(p7);
        positions.push(p7);
        positions.push(p3);
        positions.push(p0);

        // bottom face
        positions.push(p1);
        positions.push(p2);
        positions.push(p6);
        positions.push(p6);
        positions.push(p5);
        positions.push(p1);

        // outer face
        positions.push(p0);
        positions.push(p1);
        positions.push(p5);
        positions.push(p5);
        positions.push(p4);
        positions.push(p0);

        // inner face
        positions.push(p2);
        positions.push(p3);
        positions.push(p7);
        positions.push(p7);
        positions.push(p6);
        positions.push(p2);

        // normals

        // top face
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);

        // bottom face
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);

        // outer face
        normals.push([p0[0] / rout, 0.0, p0[2] / rout]);
        normals.push([p1[0] / rout, 0.0, p1[2] / rout]);
        normals.push([p5[0] / rout, 0.0, p5[2] / rout]);
        normals.push([p5[0] / rout, 0.0, p5[2] / rout]);
        normals.push([p4[0] / rout, 0.0, p4[2] / rout]);
        normals.push([p0[0] / rout, 0.0, p0[2] / rout]);

        // inner face
        normals.push([p3[0] / rin, 0.0, p3[2] / rin]);
        normals.push([p7[0] / rin, 0.0, p7[2] / rin]);
        normals.push([p6[0] / rin, 0.0, p6[2] / rin]);
        normals.push([p6[0] / rin, 0.0, p6[2] / rin]);
        normals.push([p2[0] / rin, 0.0, p2[2] / rin]);
        normals.push([p3[0] / rin, 0.0, p3[2] / rin]);
    }
    (positions, normals, uvs)
}

pub fn sphere_data(r: f32, u: usize, v: usize) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);

    for i in 0..u - 1 {
        for j in 0..v - 1 {
            let theta = i as f32 * 180.0 / (u as f32 - 1.0);
            let phi = j as f32 * 360.0 / (v as f32 - 1.0);
            let theta1 = (i as f32 + 1.0) * 180.0 / (u as f32 - 1.0);
            let phi1 = (j as f32 + 1.0) * 360.0 / (v as f32 - 1.0);
            let p0 = math_func::sphere_position(r, theta.to_radians(), phi.to_radians());
            let p1 = math_func::sphere_position(r, theta1.to_radians(), phi.to_radians());
            let p2 = math_func::sphere_position(r, theta1.to_radians(), phi1.to_radians());
            let p3 = math_func::sphere_position(r, theta.to_radians(), phi1.to_radians());

            // positions
            positions.push(p0);
            positions.push(p1);
            positions.push(p3);
            positions.push(p1);
            positions.push(p2);
            positions.push(p3);

            // normals
            normals.push([p0[0] / r, p0[1] / r, p0[2] / r]);
            normals.push([p1[0] / r, p1[1] / r, p1[2] / r]);
            normals.push([p3[0] / r, p3[1] / r, p3[2] / r]);
            normals.push([p1[0] / r, p1[1] / r, p1[2] / r]);
            normals.push([p2[0] / r, p2[1] / r, p2[2] / r]);
            normals.push([p3[0] / r, p3[1] / r, p3[2] / r]);

            // uvs
            let u0 = 0.5 + ((p0[0] / r).atan2(p0[2] / r)) / PI / 2.0;
            let u1 = 0.5 + ((p1[0] / r).atan2(p1[2] / r)) / PI / 2.0;
            let u2 = 0.5 + ((p2[0] / r).atan2(p2[2] / r)) / PI / 2.0;
            let u3 = 0.5 + ((p3[0] / r).atan2(p3[2] / r)) / PI / 2.0;
            let v0 = 0.5 - (p0[1] / r).asin() / PI;
            let v1 = 0.5 - (p1[1] / r).asin() / PI;
            let v2 = 0.5 - (p2[1] / r).asin() / PI;
            let v3 = 0.5 - (p3[1] / r).asin() / PI;

            uvs.push([u0, v0]);
            uvs.push([u1, v1]);
            uvs.push([u3, v3]);
            uvs.push([u1, v1]);
            uvs.push([u2, v2]);
            uvs.push([u3, v3]);
        }
    }
    (positions, normals, uvs)
}

pub fn cube_data_index() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<u16>) {
    let positions = [
        [-1, -1, 1],  // vertex a
        [1, -1, 1],   // vertex b
        [1, 1, 1],    // vertex c
        [-1, 1, 1],   // vertex d
        [-1, -1, -1], // vertex e
        [1, -1, -1],  // vertex f
        [1, 1, -1],   // vertex g
        [-1, 1, -1],  // vertex h
    ];

    let colors = [
        [0, 0, 1], // vertex a
        [1, 0, 1], // vertex b
        [1, 1, 1], // vertex c
        [0, 1, 1], // vertex d
        [0, 0, 0], // vertex e
        [1, 0, 0], // vertex f
        [1, 1, 0], // vertex g
        [0, 1, 0], // vertex h
    ];

    let indices = [
        0, 1, 2, 2, 3, 0, // front
        1, 5, 6, 6, 2, 1, // right
        4, 7, 6, 6, 5, 4, // back
        0, 3, 7, 7, 4, 0, // left
        3, 2, 6, 6, 7, 3, // top
        0, 4, 5, 5, 1, 0, // bottom
    ];

    (positions.to_vec(), colors.to_vec(), indices.to_vec())
}

pub fn cube_data() -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[i8; 2]>, Vec<[i8; 3]>) {
    let positions = [
        // front (0, 0, 1)
        [-1, -1, 1],
        [1, -1, 1],
        [-1, 1, 1],
        [-1, 1, 1],
        [1, -1, 1],
        [1, 1, 1],
        // right (1, 0, 0)
        [1, -1, 1],
        [1, -1, -1],
        [1, 1, 1],
        [1, 1, 1],
        [1, -1, -1],
        [1, 1, -1],
        // back (0, 0, -1)
        [1, -1, -1],
        [-1, -1, -1],
        [1, 1, -1],
        [1, 1, -1],
        [-1, -1, -1],
        [-1, 1, -1],
        // left (-1, 0, 0)
        [-1, -1, -1],
        [-1, -1, 1],
        [-1, 1, -1],
        [-1, 1, -1],
        [-1, -1, 1],
        [-1, 1, 1],
        // top (0, 1, 0)
        [-1, 1, 1],
        [1, 1, 1],
        [-1, 1, -1],
        [-1, 1, -1],
        [1, 1, 1],
        [1, 1, -1],
        // bottom (0, -1, 0)
        [-1, -1, -1],
        [1, -1, -1],
        [-1, -1, 1],
        [-1, -1, 1],
        [1, -1, -1],
        [1, -1, 1],
    ];

    let colors = [
        // front - blue
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        // right - red
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        // back - yellow
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        [1, 1, 0],
        // left - aqua
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        [0, 1, 1],
        // top - green
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        // bottom - fuchsia
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
    ];

    let uvs = [
        // front
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        // right
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        // back
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        // left
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        // top
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
        // bottom
        [0, 0],
        [1, 0],
        [0, 1],
        [0, 1],
        [1, 0],
        [1, 1],
    ];

    let normals = [
        // front
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
        // right
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        // back
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        [0, 0, -1],
        // left
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        [-1, 0, 0],
        // top
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        // bottom
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
        [0, -1, 0],
    ];

    // return data
    (
        positions.to_vec(),
        colors.to_vec(),
        uvs.to_vec(),
        normals.to_vec(),
    )
}

pub fn cube_data_uv1() -> Vec<[f32; 2]> {
    [
        //front
        [0.0, 1.0 / 2.0],
        [1.0 / 3.0, 1.0 / 2.0],
        [0.0, 1.0],
        [0.0, 1.0],
        [1.0 / 3.0, 1.0 / 2.0],
        [1.0 / 3.0, 1.0],
        //right
        [1.0 / 3.0, 1.0 / 2.0],
        [2.0 / 3.0, 1.0 / 2.0],
        [1.0 / 3.0, 1.0],
        [1.0 / 3.0, 1.0],
        [2.0 / 3.0, 1.0 / 2.0],
        [2.0 / 3.0, 1.0],
        //back
        [2.0 / 3.0, 1.0 / 2.0],
        [1.0, 1.0 / 2.0],
        [2.0 / 3.0, 1.0],
        [2.0 / 3.0, 1.0],
        [1.0, 1.0 / 2.0],
        [1.0, 1.0],
        //left
        [0.0, 0.0],
        [0.0, 1.0 / 2.0],
        [1.0 / 3.0, 0.0],
        [1.0 / 3.0, 0.0],
        [0.0, 1.0 / 2.0],
        [1.0 / 3.0, 1.0 / 2.0],
        //top
        [1.0 / 3.0, 0.0],
        [2.0 / 3.0, 0.0],
        [1.0 / 3.0, 1.0 / 2.0],
        [1.0 / 3.0, 1.0 / 2.0],
        [2.0 / 3.0, 0.0],
        [2.0 / 3.0, 1.0 / 2.0],
        //bottom
        [2.0 / 3.0, 1.0 / 2.0],
        [1.0, 1.0 / 2.0],
        [2.0 / 3.0, 0.0],
        [2.0 / 3.0, 0.0],
        [1.0, 1.0 / 2.0],
        [1.0, 0.0],
    ]
    .to_vec()
}
