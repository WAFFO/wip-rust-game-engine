use std::f32::consts::PI;
use specs::{Component,VecStorage};

use ::glm::{Vec3, FSize};

pub struct Camera {
    pub rotation: Vec3,
    // pub target: Vec3,
    pub pitch: FSize,
    pub yaw: FSize,
    pub pole_arm: FSize,
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
    pub fn forward(&self) -> Vec3 {
        self.rotation.normalize()
    }
    pub fn right(&self) -> Vec3 {
        self.rotation.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize()
    }
    pub fn add_pitch(&mut self, pitch: FSize) {
        self.pitch += pitch;

        if self.pitch > PI / 2.0 - 0.1 {
            self.pitch = PI / 2.0 - 0.1;
        } else if self.pitch < -PI / 2.0 + 0.1 {
            self.pitch = -PI / 2.0 + 0.1;
        }

        self.update();
    }
    pub fn add_yaw(&mut self, yaw: FSize) {
        self.yaw -= yaw;

        self.update();
    }
    pub fn add_pole_arm(&mut self, len: FSize) {
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
            rotation: Vec3::new(0.0, 0.0, 0.0),
            // target: Vec3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            pole_arm: 0.1,
        }
    }
}