use glam::{Mat4, Vec3};
use std::f32::consts::PI;

pub struct Camera {
    pub position: Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new<Pt: Into<Vec3>>(position: Pt, yaw: f32, pitch: f32) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.to_radians(),
            pitch: pitch.to_radians(),
        }
    }

    pub fn view_mat(&self) -> Mat4 {
        Mat4::look_to_rh(
            self.position,
            Vec3::new(
                self.pitch.cos() * self.yaw.cos(),
                self.pitch.sin(),
                self.pitch.cos() * self.yaw.sin(),
            )
            .normalize(),
            (0.0, 1.0, 0.0).into(),
        )
    }
}

pub struct CameraController {
    rotatex: f32,
    rotatey: f32,
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            rotatex: 0.0,
            rotatey: 0.0,
            speed,
        }
    }

    pub fn mouse_move(&mut self, mousex: f64, mousey: f64) {
        self.rotatex = mousex as f32;
        self.rotatey = mousey as f32;
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        camera.yaw += self.rotatex * self.speed;
        camera.pitch += self.rotatey * self.speed;

        self.rotatex = 0.0;
        self.rotatey = 0.0;

        if camera.pitch < -(89.0 * PI / 180.0) {
            camera.pitch = -(89.0 * PI / 180.0);
        } else if camera.pitch > (89.0 * PI / 180.0) {
            camera.pitch = 89.0 * PI / 180.0;
        }
    }
}
