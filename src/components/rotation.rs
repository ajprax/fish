use crate::components::Speed;
use crate::utils::{normalize_radians, Velocity};
use bevy::math::Vec2;
use bevy::prelude::Component;
use std::ops::{Add, AddAssign, Deref, Sub, SubAssign};

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Rotation(pub f32);

impl Rotation {
    pub fn new(radians: f32) -> Rotation {
        Rotation(normalize_radians(radians))
    }

    pub fn unit_vector(self) -> Vec2 {
        self.to_velocity(Speed(1.0)).0
    }

    pub fn to_velocity(self, speed: Speed) -> Velocity {
        let x = *speed * self.0.cos();
        let y = *speed * self.0.sin();
        Velocity::new(x, y)
    }
}

impl Add for Rotation {
    type Output = Rotation;

    fn add(self, rhs: Self) -> Self::Output {
        Rotation::new(self.0 + rhs.0)
    }
}

impl AddAssign for Rotation {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = normalize_radians(self.0 + rhs.0);
    }
}

impl Deref for Rotation {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sub for Rotation {
    type Output = Rotation;

    fn sub(self, rhs: Self) -> Self::Output {
        Rotation::new(self.0 - rhs.0)
    }
}

impl SubAssign for Rotation {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
