use std::f32::consts::PI;
use cgmath::{Vector3, InnerSpace};
use specs::{Component,VecStorage};

use engine::FS;

pub struct Camera {
    pub rotation: Vector3<FS>,
    // pub target: Vector3<FS>,
    pub pitch: FS,
    pub yaw: FS,
    pub pole_arm: FS,
}

//   side view    |    top view
//      c         | t<----z cos(yaw)
//     /|y sin(p) |   \  |
//    / |         |    \ |
//   /  |         |     \|x sin(yaw)
// t<---|xz cos(p)|      c

impl Camera {
    pub fn update(&mut self) {
        self.rotation[0] = self.pitch.cos() * self.yaw.sin() * self.pole_arm;
        self.rotation[1] = self.pitch.sin() * self.pole_arm;
        self.rotation[2] = self.pitch.cos() * self.yaw.cos() * self.pole_arm;
    }
    pub fn forward(&self) -> Vector3<FS> {
        self.rotation.normalize()
    }
    pub fn right(&self) -> Vector3<FS> {
        self.rotation.cross(Vector3::new(0.0, 1.0, 0.0)).normalize()
    }
    pub fn add_pitch(&mut self, pitch: FS) {
        self.pitch += pitch;

        if self.pitch > PI / 2.0 - 0.1 {
            self.pitch = PI / 2.0 - 0.1;
        } else if self.pitch < -PI / 2.0 + 0.1 {
            self.pitch = -PI / 2.0 + 0.1;
        }

        self.update();
    }
    pub fn add_yaw(&mut self, yaw: FS) {
        self.yaw -= yaw;

        self.update();
    }
    pub fn add_pole_arm(&mut self, len: FS) {
        self.pole_arm += len;

        if self.pole_arm < 0.1 {
            self.pole_arm = 0.1;
        }

        self.update();
    }
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            rotation: Vector3::new(0.0, 0.0, 0.0),
            // target: Vector3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            pole_arm: 0.1,
        }
    }
}