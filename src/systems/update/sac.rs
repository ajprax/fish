use crate::components::{Fleeing, IsFish, Position, Rotation, Size, Vision};
use crate::constants::{ALIGNMENT, COHESION, SEPARATION};
use crate::utils::can_see_position;
use bevy::math::Vec2;
use bevy::prelude::{Entity, Query};
use std::collections::HashMap;

// separation, alignment, and cohesion are system-like but are all called from the same system in
// order to share some prep work (e.g. which fish can see which others
pub fn sac(mut fish: Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), IsFish>) {
    let mut visibility: HashMap<Entity, Vec<Entity>> = HashMap::new();
    let mut combinations = fish.iter_combinations_mut();
    while let Some([(e1, _, p1, r1, v1, f1), (e2, s2, p2, r2, v2, f2)]) = combinations.fetch_next()
    {
        let mut check_visibility =
            |(e1, p1, r1, v1): (Entity, Position, Rotation, Vision),
             (e2, s2, p2): (Entity, Size, Position)| {
                if can_see_position(p1, r1, v1, s2, p2) {
                    visibility.entry(e1).or_default().push(e2);
                }
            };
        if !f1.0 {
            check_visibility((e1, *p1, *r1, *v1), (e2, *s2, *p2));
        }
        if !f2.0 {
            check_visibility((e2, *p2, *r2, *v2), (e1, *s2, *p1));
        }
    }

    separation(&mut fish, &visibility);
    alignment(&mut fish, &visibility);
    cohesion(&mut fish, &visibility);
}

/// point away from visible friends
fn separation(
    fish: &mut Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), IsFish>,
    visibility: &HashMap<Entity, Vec<Entity>>,
) {
    for (e, visible) in visibility {
        let r = {
            let (_, _, p1, r1, _, f1) = fish.get(*e).unwrap();
            if f1.0 {
                continue;
            }
            let mut r = Rotation::default();
            for e2 in visible {
                let (_, _, p2, _, _, _) = fish.get(*e2).unwrap();
                let inc = p1.steer_away(*p2, *r1, SEPARATION);
                r += inc;
            }
            r
        };
        let (_, _, _, mut r1, _, _) = fish.get_mut(*e).unwrap();
        *r1 += r;
    }
}

/// point in the same direction as visible friends
fn alignment(
    fish: &mut Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), IsFish>,
    visibility: &HashMap<Entity, Vec<Entity>>,
) {
    for (e, visible) in visibility {
        let r = {
            let (_, _, _, r1, _, f1) = fish.get(*e).unwrap();
            let mut r = Rotation::default();
            if f1.0 {
                continue;
            }
            for e2 in visible {
                let (_, _, _, r2, _, _) = fish.get(*e2).unwrap();
                r += Rotation::new({
                    let rel = *r2 - *r1;
                    if rel.0.abs() > ALIGNMENT {
                        ALIGNMENT * rel.0.signum()
                    } else {
                        rel.0
                    }
                });
            }
            r
        };
        let (_, _, _, mut r1, _, _) = fish.get_mut(*e).unwrap();
        *r1 += r;
    }
}

/// point towards the center of visible friends
fn cohesion(
    fish: &mut Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), IsFish>,
    visibility: &HashMap<Entity, Vec<Entity>>,
) {
    for (e, visible) in visibility {
        let r = {
            let (_, _, p1, r1, _, f1) = fish.get(*e).unwrap();
            if f1.0 {
                continue;
            }
            let mut center = Vec2::default();
            let mut count = 0.0;

            for e2 in visible {
                let (_, _, p2, _, _, _) = fish.get(*e2).unwrap();
                center += p2.0;
                count += 1.0;
            }

            center /= count;
            p1.steer_towards(Position(center), *r1, COHESION)
        };
        let (_, _, _, mut r1, _, _) = fish.get_mut(*e).unwrap();
        *r1 += r;
    }
}
