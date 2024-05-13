use crate::components::{Position, Rotation, Speed};
use crate::constants::{BOUNDS, HEIGHT, TIME_RATE, WIDTH};
use bevy::math::Quat;
use bevy::prelude::{Query, Res, Time, Transform};

pub fn movement(time: Res<Time>, mut moveable: Query<(&mut Position, &Rotation, &Speed)>) {
    // TODO avoid edges instead of wrapping, maybe make the tank a circle?
    for (mut p, r, s) in &mut moveable {
        *p += r.to_velocity(*s) * time.delta().as_secs_f32() * TIME_RATE;
        // println!("{p:?}");
        let [minx, maxx, miny, maxy] = BOUNDS;
        if p.x > maxx {
            p.x -= WIDTH;
        } else if p.x < minx {
            p.x += WIDTH;
        }
        if p.y > maxy {
            p.y -= HEIGHT;
        } else if p.y < miny {
            p.y += HEIGHT;
        }
    }
}

pub fn translate(mut positioned: Query<(&Position, &mut Transform)>) {
    for (p, mut t) in &mut positioned {
        t.translation.x = p.x;
        t.translation.y = p.y;
    }
}

pub fn rotate(mut rotated: Query<(&Rotation, &mut Transform)>) {
    for (r, mut t) in &mut rotated {
        t.rotation = Quat::from_rotation_z(r.0);
    }
}
