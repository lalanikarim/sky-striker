use bevy::prelude::{Commands, Entity, Query, Res, Single, Sprite, Text2d, Transform};
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use crate::components::{BulletCount, Player, ScoreBoard};
use crate::constants::BULLET_COUNT_SPRITE_PATH;

pub fn display_bullet_count(mut commands: Commands, player_query: Single<&Player>, mut bullet_count_query: Query<(Entity, &mut BulletCount)>, asset_server: Res<AssetServer>) {
    let (entity, mut bullet_count) = bullet_count_query.single_mut();
    let player = player_query.into_inner();
    let bullets = player.bullets;
    if bullets != bullet_count.ids.len() as u32 {
        for c in bullet_count.ids.len() as u32 .. bullets {
            let id = commands.spawn((
                Sprite::from_image(asset_server.load(BULLET_COUNT_SPRITE_PATH)),
                Transform::from_xyz(c as f32 * 10.,0.,0.)
            )).id();
            commands.entity(entity).add_child(id);
            bullet_count.ids.push(id);
        }
        if bullets < bullet_count.ids.len() as u32 {
            let slice = &bullet_count.ids.clone()[bullets as usize ..bullet_count.ids.len()];
            commands.entity(entity).remove_children(slice);
            for _ in slice {
                if let Some(id) = bullet_count.ids.pop() {
                    commands.entity(id).despawn();
                }
            }
        }
    }
}

pub fn display_score(mut score_query: Single<(&ScoreBoard, &mut Text2d)>,
) {
    let (score, mut text) = score_query.into_inner();
    text.0 = score.0.to_string();
}