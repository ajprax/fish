use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use iyes_perf_ui::PerfUiCompleteBundle;

use crate::components::*;
use crate::constants::*;
use crate::utils::*;

pub fn perf_startup(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}

pub fn fish_startup(
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
}

pub fn sharks_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
