use bevy::prelude::Component;
use std::ops::Deref;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Fleeing(pub bool);

impl Deref for Fleeing {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
