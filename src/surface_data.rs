#![allow(dead_code)]

use crate::colormap;
use glam::Vec3;

pub fn parametric_surface_data_uv(
    umin: f32,
    umax: f32,
    vmin: f32,
    vmax: f32,
    nu: usize,
    nv: usize,
    ul: f32,
    vl: f32,
) -> Vec<[f32; 2]> {
    let du = (umax - umin) / (nu as f32 - 1.0);
    let dv = (vmax - vmin) / (nv as f32 - 1.0);
    let mut pts: Vec<Vec<[f32; 2]>> = vec![vec![Default::default(); nv]; nu];
    for i in 0..nu {
        let u = umin + i as f32 * du;
        let mut pt1: Vec<[f32; 2]> = Vec::with_capacity(nv);
        for j in 0..nv {
            let v = vmin + j as f32 * dv;
            let pt = normalize_uv_parametric(u, v, umin, umax, vmin, vmax, ul, vl);
            pt1.push(pt);
        }
        pts[i] = pt1;
    }

    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity((4 * (nu - 1) * (nv - 1)) as usize);

    for i in 0..nu - 1 {
        for j in 0..nv - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i + 1][j];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i][j + 1];

            let mut uv = create_quad_uv(p0, p1, p2, p3);
            uvs.append(&mut uv);
        }
    }

    uvs
}

pub fn parametric_surface_data(
    f: &dyn Fn(f32, f32) -> [f32; 3],
    colormap_name: &str,
    umin: f32,
    umax: f32,
    vmin: f32,
    vmax: f32,
    nu: usize,
    nv: usize,
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    scale: f32,
    scaley: f32,
) -> (
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 2]>,
    Vec<[f32; 2]>,
) {
    let du = (umax - umin) / (nu as f32 - 1.0);
    let dv = (vmax - vmin) / (nv as f32 - 1.0);
    let mut ymin1: f32 = 0.0;
    let mut ymax1: f32 = 0.0;

    let mut pts: Vec<Vec<[f32; 3]>> = vec![vec![Default::default(); nv]; nu];
    for i in 0..nu {
        let u = umin + i as f32 * du;
        let mut pt1: Vec<[f32; 3]> = Vec::with_capacity(nv);
        for j in 0..nv {
            let v = vmin + j as f32 * dv;
            let pt = f(u, v);
            pt1.push(pt);
            ymin1 = if pt[1] < ymin1 { pt[1] } else { ymin1 };
            ymax1 = if pt[1] > ymax1 { pt[1] } else { ymax1 };
        }
        pts[i] = pt1;
    }

    let ymin = ymin1 - scaley * (ymax1 - ymin1);
    let ymax = ymax1 + scaley * (ymax1 - ymin1);

    for i in 0..nu {
        for j in 0..nv {
            pts[i][j] = normalize_point(pts[i][j], xmin, xmax, ymin, ymax, zmin, zmax, scale);
        }
    }

    let cmin = normalize_point([0.0, ymin1, 0.0], xmin, xmax, ymin, ymax, zmin, zmax, scale)[1];
    let cmax = normalize_point([0.0, ymax1, 0.0], xmin, xmax, ymin, ymax, zmin, zmax, scale)[1];

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4 * (nu - 1) * (nv - 1)) as usize);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4 * (nu - 1) * (nv - 1)) as usize);
    let mut colors: Vec<[f32; 3]> = Vec::with_capacity((4 * (nu - 1) * (nv - 1)) as usize);
    let uvs: Vec<[f32; 2]> = Vec::with_capacity((4 * (nu - 1) * (nv - 1)) as usize);
    let mut uv1: Vec<[f32; 2]> = Vec::with_capacity((4 * (nu - 1) * (nv - 1)) as usize);

    for i in 0..nu - 1 {
        for j in 0..nv - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i + 1][j];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i][j + 1];

            let (mut pos, mut norm, mut col, _uv) =
                create_quad(p0, p1, p2, p3, cmin, cmax, colormap_name, scale);

            // positions
            positions.append(&mut pos);

            // normals
            normals.append(&mut norm);

            // colors
            colors.append(&mut col);

            // uv1
            uv1.push([0.0, 0.0]);
            uv1.push([1.0, 0.0]);
            uv1.push([1.0, 1.0]);
            uv1.push([1.0, 1.0]);
            uv1.push([0.0, 1.0]);
            uv1.push([0.0, 0.0]);
        }
    }
    (positions, normals, colors, uvs, uv1)
}

pub fn simple_surface_data(
    f: &dyn Fn(f32, f32) -> [f32; 3],
    colormap_name: &str,
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    nx: usize,
    nz: usize,
    scale: f32,
    scaley: f32,
) -> (
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 2]>,
    Vec<[f32; 2]>,
) {
    let dx = (xmax - xmin) / (nx as f32 - 1.0);
    let dz = (zmax - zmin) / (nz as f32 - 1.0);
    let mut ymin1: f32 = 0.0;
    let mut ymax1: f32 = 0.0;

    let mut pts: Vec<Vec<[f32; 3]>> = vec![vec![Default::default(); nz]; nx];
    for i in 0..nx {
        let x = xmin + i as f32 * dx;
        let mut pt1: Vec<[f32; 3]> = Vec::with_capacity(nz);
        for j in 0..nz {
            let z = zmin + j as f32 * dz;
            let pt = f(x, z);
            pt1.push(pt);
            ymin1 = if pt[1] < ymin1 { pt[1] } else { ymin1 };
            ymax1 = if pt[1] > ymax1 { pt[1] } else { ymax1 };
        }
        pts[i] = pt1;
    }

    let ymin = ymin1 - scaley * (ymax1 - ymin1);
    let ymax = ymax1 + scaley * (ymax1 - ymin1);

    for i in 0..nx {
        for j in 0..nz {
            pts[i][j] = normalize_point(pts[i][j], xmin, xmax, ymin, ymax, zmin, zmax, scale);
        }
    }

    let cmin = normalize_point([0.0, ymin1, 0.0], xmin, xmax, ymin, ymax, zmin, zmax, scale)[1];
    let cmax = normalize_point([0.0, ymax1, 0.0], xmin, xmax, ymin, ymax, zmin, zmax, scale)[1];

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    let mut colors: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    let mut uv1: Vec<[f32; 2]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);

    for i in 0..nx - 1 {
        for j in 0..nz - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i][j + 1];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i + 1][j];

            let (mut pos, mut norm, mut col, mut uv) =
                create_quad(p0, p1, p2, p3, cmin, cmax, colormap_name, scale);

            // positions
            positions.append(&mut pos);

            // normals
            normals.append(&mut norm);

            // colors
            colors.append(&mut col);

            // uvs
            uvs.append(&mut uv);

            // uv1
            uv1.push([0.0, 0.0]);
            uv1.push([0.0, 1.0]);
            uv1.push([1.0, 1.0]);
            uv1.push([1.0, 1.0]);
            uv1.push([1.0, 0.0]);
            uv1.push([0.0, 0.0]);
        }
    }
    (positions, normals, colors, uvs, uv1)
}

pub fn simple_mesh_data(
    f: &dyn Fn(f32, f32) -> [f32; 3],
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    nx: usize,
    nz: usize,
    scale: f32,
    scaley: f32,
) -> Vec<[f32; 3]> {
    let dx = (xmax - xmin) / (nx as f32 - 1.0);
    let dz = (zmax - zmin) / (nz as f32 - 1.0);
    let mut ymin1: f32 = 0.0;
    let mut ymax1: f32 = 0.0;

    let mut pts: Vec<Vec<[f32; 3]>> = vec![vec![Default::default(); nz]; nx];
    for i in 0..nx {
        let x = xmin + i as f32 * dx;
        let mut pt1: Vec<[f32; 3]> = Vec::with_capacity(nz);
        for j in 0..nz {
            let z = zmin + j as f32 * dz;
            let pt = f(x, z);
            pt1.push(pt);
            ymin1 = if pt[1] < ymin1 { pt[1] } else { ymin1 };
            ymax1 = if pt[1] > ymax1 { pt[1] } else { ymax1 };
        }
        pts[i] = pt1;
    }

    let ymin = ymin1 - scaley * (ymax1 - ymin1);
    let ymax = ymax1 + scaley * (ymax1 - ymin1);
    for i in 0..nx {
        for j in 0..nz {
            pts[i][j] = normalize_point(pts[i][j], xmin, xmax, ymin, ymax, zmin, zmax, scale);
        }
    }

    let mut mesh: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);

    for i in 0..nx - 1 {
        for j in 0..nz - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i][j + 1];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i + 1][j];

            // mesh data
            mesh.push(p0);
            mesh.push(p1);
            mesh.push(p0);
            mesh.push(p3);

            if i == nx - 2 || j == nz - 2 {
                mesh.push(p1);
                mesh.push(p2);
                mesh.push(p2);
                mesh.push(p3)
            }
        }
    }
    mesh
}

fn normalize_uv_parametric(
    u: f32,
    v: f32,
    umin: f32,
    umax: f32,
    vmin: f32,
    vmax: f32,
    ul: f32,
    vl: f32,
) -> [f32; 2] {
    let uu = ul * (u - umin) / (umax - umin);
    let vv = vl * (v - vmin) / (vmax - vmin);
    [uu, vv]
}

fn normalize_uv(pt: [f32; 3], scale: f32) -> [f32; 2] {
    let px = (1.0 + pt[0] / scale) / 2.0;
    let pz = (1.0 + pt[2] / scale) / 2.0;
    [px, pz]
}

fn normalize_point(
    pt: [f32; 3],
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
    zmin: f32,
    zmax: f32,
    scale: f32,
) -> [f32; 3] {
    let px = scale * (-1.0 + 2.0 * (pt[0] - xmin) / (xmax - xmin));
    let py = scale * (-1.0 + 2.0 * (pt[1] - ymin) / (ymax - ymin));
    let pz = scale * (-1.0 + 2.0 * (pt[2] - zmin) / (zmax - zmin));
    [px, py, pz]
}

fn create_quad_uv(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> Vec<[f32; 2]> {
    let mut uv: Vec<[f32; 2]> = Vec::with_capacity(6);
    uv.push(p0);
    uv.push(p1);
    uv.push(p2);
    uv.push(p2);
    uv.push(p3);
    uv.push(p0);
    uv
}

fn create_quad(
    p0: [f32; 3],
    p1: [f32; 3],
    p2: [f32; 3],
    p3: [f32; 3],
    ymin: f32,
    ymax: f32,
    colormap_name: &str,
    scale: f32,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    // position
    let mut position: Vec<[f32; 3]> = Vec::with_capacity(6);
    position.push(p0);
    position.push(p1);
    position.push(p2);
    position.push(p2);
    position.push(p3);
    position.push(p0);

    // normal
    let ca = Vec3::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);
    let db = Vec3::new(p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]);
    let cp = (ca.cross(db)).normalize();
    let mut normal: Vec<[f32; 3]> = Vec::with_capacity(6);
    normal.push([cp[0], cp[1], cp[2]]);
    normal.push([cp[0], cp[1], cp[2]]);
    normal.push([cp[0], cp[1], cp[2]]);
    normal.push([cp[0], cp[1], cp[2]]);
    normal.push([cp[0], cp[1], cp[2]]);
    normal.push([cp[0], cp[1], cp[2]]);

    // color
    let c0 = colormap::color_lerp(colormap_name, ymin, ymax, p0[1]);
    let c1 = colormap::color_lerp(colormap_name, ymin, ymax, p1[1]);
    let c2 = colormap::color_lerp(colormap_name, ymin, ymax, p2[1]);
    let c3 = colormap::color_lerp(colormap_name, ymin, ymax, p3[1]);
    let mut color: Vec<[f32; 3]> = Vec::with_capacity(6);
    color.push(c0);
    color.push(c1);
    color.push(c2);
    color.push(c2);
    color.push(c3);
    color.push(c0);

    // uv
    let mut uv: Vec<[f32; 2]> = Vec::with_capacity(6);
    let uv0 = normalize_uv(p0, scale);
    let uv1 = normalize_uv(p1, scale);
    let uv2 = normalize_uv(p2, scale);
    let uv3 = normalize_uv(p3, scale);
    uv.push(uv0);
    uv.push(uv1);
    uv.push(uv2);
    uv.push(uv2);
    uv.push(uv3);
    uv.push(uv0);

    (position, normal, color, uv)
}
