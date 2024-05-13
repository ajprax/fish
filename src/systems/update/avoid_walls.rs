use crate::components::{Position, Rotation, Vision};
use crate::constants::WALL_AVOIDANCE;
use crate::utils::{distance_to_circle_wall, proximity_to_walls};
use bevy::prelude::Query;
use std::f32::consts::PI;

pub fn avoid_circle_walls(mut swimmers: Query<(&Position, &mut Rotation, &Vision)>) {
    // TODO: calculate the distance to the wall in the direction of travel
    //       if it hits the wall, calculate the distances to the wall of a small turn left or right
    //       add the turn with the greater distance, scaled inversely by that distance

    for (p, mut r, v) in &mut swimmers {
        if distance_to_circle_wall(*p, *r) < v.distance {
            // println!("{p:?} {r:?} avoiding");
            let left = distance_to_circle_wall(*p, *r + Rotation::new(WALL_AVOIDANCE));
            let right = distance_to_circle_wall(*p, *r + Rotation::new(-WALL_AVOIDANCE));
            if left > right {
                // println!("turning left");
                *r = Rotation::new(r.0 + WALL_AVOIDANCE * (v.distance / left).max(2.0));
            } else {
                // println!("turning right");
                *r = Rotation::new(r.0 - WALL_AVOIDANCE * (v.distance / right).max(2.0));
            }
        }
    }
}

pub fn avoid_square_walls(mut swimmers: Query<(&Position, &mut Rotation, &Vision)>) {
    for (p, mut r, v) in &mut swimmers {
        let (left, right, top, bottom) = proximity_to_walls(*p, *r);
        // println!("{p:?} {r:?} {left} {right} {top} {bottom}");
        let mut left_turn = 0.0;
        let mut right_turn = 0.0;
        if left != 0.0 && left < v.distance {
            let power = WALL_AVOIDANCE * (v.distance / left).max(2.0);
            if r.0 > 0.0 {
                right_turn += power;
            } else {
                left_turn += power;
            }
        }
        if right != 0.0 && right < v.distance {
            let power = WALL_AVOIDANCE * (v.distance / right).max(2.0);
            if r.0 > 0.0 {
                left_turn += power;
            } else {
                right_turn += power;
            }
        }
        if top != 0.0 && top < v.distance {
            let power = WALL_AVOIDANCE * (v.distance / top).max(2.0);
            if r.0 > PI / 2.0 {
                left_turn += power;
            } else {
                right_turn += power;
            }
        }
        if bottom != 0.0 && bottom < v.distance {
            let power = WALL_AVOIDANCE * (v.distance / bottom).max(2.0);
            if r.0 < -PI / 2.0 {
                right_turn += power;
            } else {
                left_turn += power;
            }
        }
        if left_turn == right_turn {
            // bias for right turns (clockwise)
            *r = Rotation::new(r.0 - right_turn);
        } else if left_turn > right_turn {
            *r = Rotation::new(r.0 + left_turn);
        } else {
            *r = Rotation::new(r.0 - right_turn);
        }
    }
}
