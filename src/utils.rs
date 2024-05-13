use crate::components::{Position, Rotation, Size, Vision};
use crate::constants::{BOUNDS, RADIUS};
use bevy::math::Vec2;
use rand::random;
use std::f32::consts::{PI, TAU};
use std::ops::{Add, AddAssign, Deref, Mul};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Direction {
    Left,
    Right,
    #[default]
    Straight,
}

impl Direction {
    pub fn next() -> Direction {
        let r = random::<f32>() * 6.0;
        if r < 4.0 {
            Direction::Straight
        } else if r < 5.0 {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}

/// normalizes an angle in radians into the range [-PI, PI)
pub fn normalize_radians(r: f32) -> f32 {
    let r = (r % TAU + TAU) % TAU;
    if r >= PI {
        r - TAU
    } else {
        r
    }
}

pub fn random_in_range(min: f32, max: f32) -> f32 {
    random::<f32>() * (max - min) + min
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity(Vec2::new(x, y))
    }
}

impl Mul<f32> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f32) -> Self::Output {
        Velocity(self.0 * rhs)
    }
}

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add<Velocity> for Position {
    type Output = Position;

    fn add(self, rhs: Velocity) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.0.x += rhs.x;
        self.0.y += rhs.y;
    }
}

pub fn can_see_position(p1: Position, r1: Rotation, v1: Vision, s2: Size, p2: Position) -> bool {
    let angle = normalize_radians(p1.point_towards(p2).0 - r1.0);
    let distance = p1.distance(p2);
    (distance * s2.0 < v1.distance) && (angle < v1.angle) && (angle > -v1.angle)
}

// returns (left, right, top, bottom)
// returns 0 if facing away from that wall
pub fn distance_to_walls(p: Position, r: Rotation) -> (f32, f32, f32, f32) {
    let [minx, maxx, miny, maxy] = BOUNDS;
    let left = p.x - minx;
    let right = maxx - p.x;
    let top = maxy - p.y;
    let bottom = p.y - miny;
    // println!("{p:?} {left} {right} {top} {bottom}");
    (
        if r.0 > PI / 2.0 || r.0 < -PI / 2.0 {
            left
        } else {
            0.0
        },
        if r.0 < PI / 2.0 && r.0 > -PI / 2.0 {
            right
        } else {
            0.0
        },
        if r.0 > 0.0 { top } else { 0.0 },
        if r.0 < 0.0 { bottom } else { 0.0 },
    )
}

// https://www.bluebill.net/circle_ray_intersection.html
pub fn distance_to_circle_wall(p: Position, r: Rotation) -> f32 {
    let C = Vec2::ZERO;
    let P = p.0;
    let V = r.unit_vector();
    let U = C - P;
    let U1 = U.dot(V) * V;
    let U2 = U - U1;
    let d = U2.length();
    let m = (RADIUS.powi(2) - d.powi(2)).sqrt();
    let P1 = P + U1 + m * V;
    p.distance(Position(P1))
}
