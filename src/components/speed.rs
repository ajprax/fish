use bevy::prelude::Component;
use std::ops::Deref;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Speed(pub f32);

impl Deref for Speed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
