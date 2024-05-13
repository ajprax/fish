use crate::components::Fish;
use bevy::prelude::{Component, With, Without};

#[derive(Component)]
pub struct Shark;

pub type IsShark = (With<Shark>, Without<Fish>);
