use crate::Vec3;

const KINDA_SMALL_NUMBER: f32 = 0.00001f32;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn maybe_expand(&mut self, point: Vec3) {
        if !self.point_inside(point) {
            self.radius = (point - self.center).distance();
        }
    }

    pub fn point_inside(&self, point: Vec3) -> bool {
        let relative_point = point - self.center;

        (self.radius + KINDA_SMALL_NUMBER) * (self.radius + KINDA_SMALL_NUMBER)
            >= relative_point.distance_squared()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let dist_sq = (self.center - other.center).distance_squared();
        let radius_sum = self.radius + other.radius;
        dist_sq <= radius_sum * radius_sum
    }
}
