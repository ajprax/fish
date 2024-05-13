use crate::components::Rotation;
use crate::constants::RADIUS;
use crate::utils::random_in_range;
use bevy::math::Vec2;
use bevy::prelude::Component;
use rand::random;
use std::f32::consts::{PI, TAU};
use std::ops::{Deref, DerefMut};

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position(Vec2::new(x, y))
    }

    pub fn random_in_square() -> Position {
        Position::new(
            random_in_range(-720.0, 720.0),
            random_in_range(-360.0, 360.0),
        )
    }

    pub fn random_in_circle() -> Position {
        let r = RADIUS * random::<f32>().sqrt();
        let theta = random::<f32>() * TAU;
        let x = r * theta.cos();
        let y = r * theta.sin();
        Position::new(x, y)
    }

    pub fn distance(self, other: Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn point_towards(self, other: Position) -> Rotation {
        let mut away = self.point_away(other);
        away.0 += PI;
        away
    }

    pub fn point_away(self, other: Position) -> Rotation {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let mut theta = match (dx, dy) {
            (0.0, 0.0) => 0.0,
            (0.0, dy) if dy > 0.0 => PI / 2.0,
            (0.0, dy) if dy < 0.0 => 3.0 * PI / 2.0,
            (dx, dy) => (dy / dx).atan(),
        };
        if dx.is_sign_positive() {
            theta += PI;
        }
        Rotation::new(theta)
    }

    // like point_towards, but rotates no more than max radians
    pub fn steer_towards(self, other: Position, rotation: Rotation, max: f32) -> Rotation {
        let rel = self.point_towards(other) - rotation;
        if rel.0.abs() > max {
            Rotation::new(max * rel.0.signum())
        } else {
            rel
        }
    }

    pub fn steer_away(self, other: Position, rotation: Rotation, max: f32) -> Rotation {
        let rel = self.point_away(other) - rotation;
        if rel.0.abs() > max {
            Rotation::new(max * rel.0.signum())
        } else {
            rel
        }
    }
}

impl Deref for Position {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
