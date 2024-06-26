use std::f32::consts::PI;

pub const WALL_AVOIDANCE: f32 = PI / 60.0;
pub const SEPARATION: f32 = PI / 90.0;
pub const ALIGNMENT: f32 = PI / 180.0;
pub const COHESION: f32 = PI / 180.0;
pub const TIME_RATE: f32 = 120.0;
pub const NFISH: usize = 400;
pub const NSHARKS: usize = 0;
pub const FISH_SPEED: f32 = 1.25;
pub const SHARK_SPEED: f32 = 0.75;
pub const FLIGHT_MAX: f32 = 200.0;
pub const FLIGHT_SPEED: f32 = 4.0;
pub const VISIBLE_DISTANCE: f32 = 75.0;
pub const VISIBLE_ANGLE: f32 = PI * 3.0 / 4.0;
pub const FISH_NOISE: f32 = PI / 45.0;
pub const SHARK_NOISE: f32 = PI / 90.0;
pub const FISH_SIZE_RANGE: (f32, f32) = (0.5, 2.0);
pub const SHARK_SIZE_RANGE: (f32, f32) = (1.5, 6.0);
pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const BOUNDS: [f32; 4] = [-WIDTH / 2.0, WIDTH / 2.0, -HEIGHT / 2.0, HEIGHT / 2.0];
pub const RADIUS: f32 = HEIGHT / 2.0;
pub const USE_CIRLCE: bool = true;
pub const PERF: bool = false;
