use crate::components::Shark;
use bevy::prelude::{Component, With, Without};

#[derive(Component)]
pub struct Fish;

pub type IsFish = (With<Fish>, Without<Shark>);
