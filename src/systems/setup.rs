use bevy::prelude::{Camera2d, Commands, Res, Sprite, Transform};
use bevy::asset::AssetServer;
use bevy::math::{Quat, Vec3};
use crate::components::{BulletCount, Player, ScoreBoard};
use crate::constants::{PLAYER_SPRITE_PATH, PLAYER_STARTING_POSITION, SPRITE_SCALE};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Spawn the player
    commands.spawn((
        Sprite::from_image(asset_server.load(PLAYER_SPRITE_PATH)),
        Transform {
            translation: PLAYER_STARTING_POSITION,
            rotation: Quat::default(),
            scale: Vec3::splat(SPRITE_SCALE),
        },
        Player::default(),
    ));
    commands.spawn((
        Transform::from_xyz(0.,300.,1.),
        ScoreBoard
    ));
    commands.spawn((
        Transform::from_xyz(-600.,300.,1.),
        BulletCount::default(),
    ));
}