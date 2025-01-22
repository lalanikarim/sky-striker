mod constants;
mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(
            Update,
            (
                steer_aircraft,
                reload,
                move_aircraft,
                shoot_bullets,
                move_bullets,
                animate_bullets,
                display_score,
                display_bullet_count,
            ),
        )
        .run();
}