use crate::vec3::Vec3;
use core::ops::Index;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Index<usize> for Quat {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("INDEXING OUT_OF_BOUNDS in Quat"),
        }
    }
}
impl Quat {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Quat {
        Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn new_from_axis_angle(axis: &Vec3, angle: f32) -> Quat {
        let factor = f32::sin(angle / 2.0);

        let x = axis[0] * factor;
        let y = axis[1] * factor;
        let z = axis[2] * factor;
        let w = f32::cos(angle / 2.0);

        let mut quat = Quat { x, y, z, w };
        quat.normalize();

        quat
    }

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalize(&mut self) {
        let len_sq = self.length_squared();
        self.x = self.x / len_sq;
        self.y = self.y / len_sq;
        self.z = self.z / len_sq;
        self.w = self.w / len_sq;
    }

    pub fn rotate_vec3(&self, v: Vec3) -> Vec3 {
        let u = Vec3::new(self.x, self.y, self.z);
        let s = self.w;

        2.0 * u.dot(v) * u + (s * s - u.dot(u)) * v + 2.0 * s * u.cross(v)
    }
}
