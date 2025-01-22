use bevy::prelude::{Component, Entity};

#[derive(Component, Default)]
pub struct BulletCount {
    pub(crate) ids: Vec<Entity>,
}