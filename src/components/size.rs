use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug)]
pub struct Size(pub f32);

impl Default for Size {
    fn default() -> Self {
        Size(1.0)
    }
}
