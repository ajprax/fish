use crate::components::size::Size;
use crate::constants::{VISIBLE_ANGLE, VISIBLE_DISTANCE};
use bevy::prelude::Component;
use std::ops::Mul;

#[derive(Component, Clone, Copy, Debug)]
pub struct Vision {
    pub distance: f32,
    pub angle: f32,
}

impl Vision {
    fn new(distance: f32, angle: f32) -> Vision {
        Vision { distance, angle }
    }
}

impl Default for Vision {
    fn default() -> Self {
        Vision {
            distance: VISIBLE_DISTANCE,
            angle: VISIBLE_ANGLE,
        }
    }
}

impl Mul<Size> for Vision {
    type Output = Vision;

    fn mul(self, rhs: Size) -> Self::Output {
        Vision::new(self.distance * rhs.0, self.angle)
    }
}
