use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub [f32; 3]);

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.0[0],
            1 => &self.0[1],
            2 => &self.0[2],
            _ => panic!("INDEXING OUT_OF_BOUNDS in Vec3"),
        }
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0[0],
            1 => &mut self.0[1],
            2 => &mut self.0[2],
            _ => panic!("INDEXING OUT_OF_BOUNDS in Vec3"),
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self([self[0] - other[0], self[1] - other[1], self[2] - other[2]])
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
    }
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3([x, y, z])
    }

    #[inline]
    pub fn add(&self, _rhs: &Vec3) -> Vec3 {
        Vec3([self[0] + _rhs[0], self[1] + _rhs[1], self[2] + _rhs[2]])
    }

    #[inline]
    pub fn mul(&self, _rhs: f32) -> Vec3 {
        Vec3([self[0] * _rhs, self[1] * _rhs, self[2] * _rhs])
    }

    #[inline]
    pub fn normalize(&self) -> Vec3 {
        let lensq = self[0] * self[0] + self[1] * self[1] + self[2] * self[2];
        if lensq == 0.0 {
            return Vec3([0.0, 0.0, 0.0]);
        }
        let lenroot = f32::sqrt(lensq);
        Vec3([self[0] / lenroot, self[1] / lenroot, self[2] / lenroot])
    }

    #[inline]
    pub fn dot(&self, b: Vec3) -> f32 {
        self[0] * b[0] + self[1] * b[1] + self[2] * b[2]
    }

    #[inline]
    pub fn cross(&self, b: Vec3) -> Self {
        Self([
            self[1] * b[2] - self[2] * b[1],
            self[2] * b[0] - self[0] * b[2],
            self[0] * b[1] - self[1] * b[0],
        ])
    }
}

#[test]
fn test_cross() {
    use crate::Vec3;
    let x_axis = Vec3::new(1.0, 0.0, 0.0);
    let y_axis = Vec3::new(0.0, 1.0, 0.0);
    let cross_product = x_axis.cross(y_axis);
    assert_eq!(cross_product[0], 0.0);
    assert_eq!(cross_product[1], 0.0);
    assert_eq!(cross_product[2], 1.0);
}
