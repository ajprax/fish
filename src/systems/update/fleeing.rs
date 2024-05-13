use bevy::prelude::*;

use crate::can_see_position;
use crate::components::{Fleeing, IsFish, IsShark, Position, Rotation, Size, Speed, Vision};
use crate::constants::{FLIGHT_MAX, FLIGHT_SPEED};

pub fn start_fleeing(
    mut fish: Query<(&Position, &mut Rotation, &mut Speed, &Vision, &mut Fleeing), IsFish>,
    sharks: Query<(&Size, &Position), IsShark>,
) {
    for (p, mut r, mut s, v, mut f) in &mut fish {
        if f.0 {
            continue;
        }

        for (ss, sp) in &sharks {
            if can_see_position(*p, *r, *v, *ss, *sp) {
                f.0 = true;
                s.0 *= FLIGHT_SPEED;
                *r = p.point_away(*sp);
            }
        }
    }
}

pub fn stop_fleeing(
    mut fish: Query<(&Position, &mut Speed, &mut Fleeing), IsFish>,
    sharks: Query<&Position, IsShark>,
) {
    for (p, mut s, mut f) in &mut fish {
        if !f.0 {
            continue;
        }
        if sharks.iter().all(|sp| p.distance(*sp) > FLIGHT_MAX) {
            f.0 = false;
            s.0 /= FLIGHT_SPEED;
        }
    }
}
