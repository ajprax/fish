use crate::components::{Fleeing, IsFish, IsShark, Rotation};
use crate::constants::{FISH_NOISE, SHARK_NOISE};
use crate::utils::Direction;
use bevy::prelude::Query;

pub fn fish_wander(mut fish: Query<(&mut Rotation, &Fleeing), IsFish>) {
    for (mut r, f) in &mut fish {
        if !f.0 {
            match Direction::next() {
                Direction::Left => r.0 += FISH_NOISE,
                Direction::Right => r.0 -= FISH_NOISE,
                Direction::Straight => {}
            }
        }
    }
}

pub fn sharks_wander(mut sharks: Query<&mut Rotation, IsShark>) {
    for mut r in &mut sharks {
        match Direction::next() {
            Direction::Left => r.0 += SHARK_NOISE,
            Direction::Right => r.0 -= SHARK_NOISE,
            Direction::Straight => {}
        }
    }
}
