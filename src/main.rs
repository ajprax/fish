use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};
use rand::random;
use std::collections::HashMap;
use std::f32::consts::{PI, TAU};
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign};

const WALL_AVOIDANCE: f32 = PI / 60.0;
const SEPARATION: f32 = PI / 90.0;
const ALIGNMENT: f32 = PI / 180.0;
const COHESION: f32 = PI / 180.0;
const TIME_RATE: f32 = 120.0; // TODO: the time rate affects the relative scale of speed vs angles. it may be correct to multiply or divide some of those values as well
const NFISH: usize = 400;
const NSHARKS: usize = 1;
const FISH_SPEED: f32 = 1.25;
const SHARK_SPEED: f32 = 0.75;
const FLIGHT_MAX: f32 = 200.0; // TODO: consider making this a duration instead
const FLIGHT_SPEED: f32 = 4.0;
const VISIBLE_DISTANCE: f32 = 75.0;
const VISIBLE_ANGLE: f32 = PI * 3.0 / 4.0;
const FISH_NOISE: f32 = PI / 45.0;
const SHARK_NOISE: f32 = PI / 90.0;
const FISH_SIZE_RANGE: (f32, f32) = (0.5, 2.0);
const SHARK_SIZE_RANGE: (f32, f32) = (1.5, 6.0);
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const BOUNDS: [f32; 4] = [-WIDTH / 2.0, WIDTH / 2.0, -HEIGHT / 2.0, HEIGHT / 2.0];
const RADIUS: f32 = HEIGHT / 2.0;
const USE_CIRLCE: bool = true;

#[derive(Component)]
struct Fish;

#[derive(Component)]
struct Shark;

#[derive(Component, Clone, Copy, Debug)]
struct Size(f32);

impl Default for Size {
    fn default() -> Self {
        Size(1.0)
    }
}

#[derive(Component, Clone, Copy, Debug)]
struct Vision {
    distance: f32,
    angle: f32,
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

#[derive(Component, Clone, Copy, Debug, Default)]
struct Position(Vec2);

impl Position {
    fn new(x: f32, y: f32) -> Position {
        Position(Vec2::new(x, y))
    }

    fn random_in_square() -> Position {
        Position::new(
            random_in_range(-720.0, 720.0),
            random_in_range(-360.0, 360.0),
        )
    }

    fn random_in_circle() -> Position {
        let r = RADIUS * random::<f32>().sqrt();
        let theta = random::<f32>() * TAU;
        let x = r * theta.cos();
        let y = r * theta.sin();
        Position::new(x, y)
    }

    fn distance(self, other: Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn point_towards(self, other: Position) -> Rotation {
        let mut away = self.point_away(other);
        away.0 += PI;
        away
    }

    fn point_away(self, other: Position) -> Rotation {
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
    fn steer_towards(self, other: Position, rotation: Rotation, max: f32) -> Rotation {
        let rel = rotation.relative_rotation(self.point_towards(other));
        if rel.0.abs() > max {
            Rotation::new(max * rel.0.signum())
        } else {
            rel
        }
    }

    fn steer_away(self, other: Position, rotation: Rotation, max: f32) -> Rotation {
        let rel = rotation.relative_rotation(self.point_away(other));
        if rel.0.abs() > max {
            let maxed = Rotation::new(max * rel.0.signum());
            maxed
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

#[derive(Clone, Copy, Debug, Default)]
struct Velocity(Vec2);

impl Velocity {
    fn new(x: f32, y: f32) -> Velocity {
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

#[derive(Component, Clone, Copy, Debug, Default)]
struct Rotation(f32);

impl Rotation {
    fn new(radians: f32) -> Rotation {
        Rotation(normalize_radians(radians))
    }

    fn relative_rotation(self, other: Rotation) -> Rotation {
        Rotation::new(other.0 - self.0)
    }

    // fn mirror_over_x(self) -> Rotation {
    //     Rotation::new(-self.0)
    // }
    //
    // fn mirror_over_y(self) -> Rotation {
    //     if self.0.is_sign_positive() {
    //         Rotation::new(self.0 + (PI / 2.0 - self.0) * 2.0)
    //     } else {
    //         Rotation::new(self.0 - (-PI / 2.0 - self.0) * 2.0)
    //     }
    // }

    fn unit_vector(self) -> Vec2 {
        self.to_velocity(Speed(1.0)).0
    }

    fn to_velocity(self, speed: Speed) -> Velocity {
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

#[derive(Component, Clone, Copy, Debug, Default)]
struct Speed(f32);

impl Deref for Speed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
struct Fleeing(bool);

impl Deref for Fleeing {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Direction {
    Left,
    Right,
    #[default]
    Straight,
}

impl Direction {
    fn next() -> Direction {
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
fn normalize_radians(r: f32) -> f32 {
    let r = (r % TAU + TAU) % TAU;
    if r > PI {
        r - TAU
    } else {
        r
    }
}

fn random_in_range(min: f32, max: f32) -> f32 {
    random::<f32>() * (max - min) + min
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    for _ in 0..NFISH {
        let size = Size(random_in_range(FISH_SIZE_RANGE.0, FISH_SIZE_RANGE.1));
        let position = if USE_CIRLCE {
            Position::random_in_circle()
        } else {
            Position::random_in_square()
        };
        let rotation = Rotation::new(random_in_range(-PI, PI));
        let speed = Speed(FISH_SPEED * size.0);
        let vision = Vision::default() * size;
        let fleeing = Fleeing::default();
        let mesh = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                Vec2::new(10.0, 0.0) * size.0,
                Vec2::new(-3.0, 3.0) * size.0,
                Vec2::new(-3.0, -3.0) * size.0,
            ))),
            // material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
            material: materials.add(Color::hsl(
                random_in_range(180.0, 250.0),
                random_in_range(0.3, 0.7),
                random_in_range(0.3, 0.7),
            )),
            transform: Transform::from_xyz(position.x, position.y, 0.5),
            ..default()
        };
        commands.spawn((Fish, size, position, rotation, speed, vision, fleeing, mesh));
    }

    for _ in 0..NSHARKS {
        let size = Size(random_in_range(SHARK_SIZE_RANGE.0, SHARK_SIZE_RANGE.1));
        let position = Position::default();
        let rotation = Rotation::default();
        let speed = Speed(SHARK_SPEED * size.0);
        let mut vision = Vision::default() * size;
        vision.angle = PI / 3.0;
        let mesh = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                Vec2::new(10.0, 0.0) * size.0,
                Vec2::new(-3.0, 3.0) * size.0,
                Vec2::new(-3.0, -3.0) * size.0,
            ))),
            material: materials.add(Color::rgb(0.75, 0.75, 0.75)),
            transform: Transform::from_xyz(position.x, position.y, 0.5),
            ..default()
        };
        commands.spawn((Shark, size, position, rotation, speed, vision, mesh));
    }
}

fn start_fleeing(
    mut fish: Query<
        (&Position, &mut Rotation, &mut Speed, &Vision, &mut Fleeing),
        (With<Fish>, Without<Shark>),
    >,
    sharks: Query<(&Size, &Position), (With<Shark>, Without<Fish>)>,
) {
    for (p, mut r, mut s, v, mut f) in &mut fish {
        if f.0 {
            continue;
        }

        for (ss, sp) in &sharks {
            if can_see_position(*p, *r, *v, *ss, *sp) {
                f.0 = true;
                s.0 *= FLIGHT_SPEED;
                // TODO: be subtler, just turn away
                *r = p.point_away(*sp);
            }
        }
    }
}

fn stop_fleeing(
    mut fish: Query<(&Position, &mut Speed, &mut Fleeing), (With<Fish>, Without<Shark>)>,
    sharks: Query<&Position, (With<Shark>, Without<Fish>)>,
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

fn can_see_position(p1: Position, r1: Rotation, v1: Vision, s2: Size, p2: Position) -> bool {
    let angle = normalize_radians(p1.point_towards(p2).0 - r1.0);
    let distance = p1.distance(p2);
    (distance * s2.0 < v1.distance) && (angle < v1.angle) && (angle > -v1.angle)
}

// returns (left, right, top, bottom
// returns 0 if facing away from that wall
fn proximity_to_walls(p: Position, r: Rotation) -> (f32, f32, f32, f32) {
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

// separation, alignment, and cohesion are system-like but are all called from the same system in
// order to share some prep work (e.g. which fish can see which others
// TODO: see if we can make visibility a resource with a system to update it
//       ultimately this resource could manage a kdtree for more efficient lookups also
fn sac(mut fish: Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), With<Fish>>) {
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

/// point awaay from visible friends
fn separation(
    fish: &mut Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), With<Fish>>,
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
    fish: &mut Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), With<Fish>>,
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
    fish: &mut Query<(Entity, &Size, &Position, &mut Rotation, &Vision, &Fleeing), With<Fish>>,
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

fn noise(mut fish: Query<(&mut Rotation, &Fleeing), (With<Fish>, Without<Shark>)>) {
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
fn sharks(mut sharks: Query<&mut Rotation, With<Shark>>) {
    for mut r in &mut sharks {
        match Direction::next() {
            Direction::Left => r.0 += SHARK_NOISE,
            Direction::Right => r.0 -= SHARK_NOISE,
            Direction::Straight => {}
        }
    }
}

// https://www.bluebill.net/circle_ray_intersection.html
fn distance_to_circle_wall(p: Position, r: Rotation) -> f32 {
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

fn avoid_circle_walls(mut swimmers: Query<(&Position, &mut Rotation, &Vision)>) {
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

fn avoid_square_walls(mut swimmers: Query<(&Position, &mut Rotation, &Vision)>) {
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

fn movement(time: Res<Time>, mut moveable: Query<(&mut Position, &Rotation, &Speed)>) {
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

fn translate(mut positioned: Query<(&Position, &mut Transform)>) {
    for (p, mut t) in &mut positioned {
        t.translation.x = p.x;
        t.translation.y = p.y;
    }
}

fn rotate(mut rotated: Query<(&Rotation, &mut Transform)>) {
    for (r, mut t) in &mut rotated {
        t.rotation = Quat::from_rotation_z(r.0);
    }
}

struct Perf;

impl Perf {
    fn startup(mut commands: Commands) {
        commands.spawn(PerfUiCompleteBundle::default());
    }
}

impl Plugin for Perf {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(120.0))
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, Perf::startup);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(Perf)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                start_fleeing,
                stop_fleeing,
                sac,
                noise,
                sharks,
                avoid_circle_walls,
                // avoid_square_walls,
                movement,
                translate,
                rotate,
            )
                .chain(),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
