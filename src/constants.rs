use bevy::prelude::*;

pub const PLAYER_SPRITE_PATH: &str = "pixelshmup/Ships/ship_0004.png";
pub const BULLET_SPRITE_PATH: &str = "pixelshmup/Tiles/tile_0003.png";
pub const EXPLOSION_SPRITE_01_PATH: &str = "pixelshmup/Tiles/tile_0004.png";
pub const EXPLOSION_SPRITE_02_PATH: &str = "pixelshmup/Tiles/tile_0005.png";
pub const EXPLOSION_SPRITE_03_PATH: &str = "pixelshmup/Tiles/tile_0007.png";
pub const EXPLOSION_SPRITE_04_PATH: &str = "pixelshmup/Tiles/tile_0008.png";
pub const BULLET_COUNT_SPRITE_PATH: &str = "pixelshmup/Tiles/tile_0002.png";
pub const PLAYER_VELOCITY: f32 = 100.;
pub const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(0., -200., 0.);
pub const SPRITE_SCALE: f32 = 2.;
pub const MAGAZINE_CAPACITY: u32 = 5;
pub const RELOAD_TIME: f32 = 5.;
pub const NUMBER_SPRITES: (&str,&str,&str,&str,&str,&str,&str,&str,&str,&str,) = ("pixelshmup/Tiles/tile_0019.png",
"pixelshmup/Tiles/tile_0020.png",
"pixelshmup/Tiles/tile_0021.png",
"pixelshmup/Tiles/tile_0022.png",
"pixelshmup/Tiles/tile_0023.png",
"pixelshmup/Tiles/tile_0031.png",
"pixelshmup/Tiles/tile_0032.png",
"pixelshmup/Tiles/tile_0033.png",
"pixelshmup/Tiles/tile_0034.png",
"pixelshmup/Tiles/tile_0035.png",);