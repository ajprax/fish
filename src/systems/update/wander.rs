use crate::components::{Fish, Fleeing, Rotation, Shark};
use crate::constants::{FISH_NOISE, SHARK_NOISE};
use crate::utils::Direction;
use bevy::prelude::{Query, With, Without};

pub fn fish_wander(mut fish: Query<(&mut Rotation, &Fleeing), (With<Fish>, Without<Shark>)>) {
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

// TODO: give the shark hunger.
//       make it steer towards fish in proportion to that hunger
//       remove (eat) fish that get too close
pub fn sharks_wander(mut sharks: Query<&mut Rotation, With<Shark>>) {
    for mut r in &mut sharks {
        match Direction::next() {
            Direction::Left => r.0 += SHARK_NOISE,
            Direction::Right => r.0 -= SHARK_NOISE,
            Direction::Straight => {}
        }
    }
}
