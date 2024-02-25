use std::string::ToString;

use crate::states::UiSystemSet;
use bevy::prelude::*;
use robotics_lib::world::tile::{Content, TileType};

const FILE_NAME: &str = "TILEMAP.png";
const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const COLUMNS: usize = 6;
const ROWS: usize = 5;

pub struct SpriteMapPlugin;
impl Plugin for SpriteMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_assets).in_set(UiSystemSet::UiStartup));
    }
}
#[derive(Resource)]
pub struct SpriteSheet {
    pub atlas: Handle<TextureAtlas>,
}

impl SpriteSheet {
    pub fn get_tiletype_sprite_index(&self, t: Option<TileType>) -> usize {
        match t {
            None => 11,
            Some(tt) => match tt {
                TileType::DeepWater => 0,
                TileType::ShallowWater => 1,
                TileType::Sand => 2,
                TileType::Grass => 3,
                TileType::Street => 4,
                TileType::Hill => 5,
                TileType::Mountain => 6,
                TileType::Snow => 7,
                TileType::Lava => 8,
                TileType::Teleport(_) => 9,
                TileType::Wall => 10,
            },
        }
    }

    pub fn get_content_sprite_index(&self, c: Content) -> usize {
        match c {
            Content::Rock(_) => 12,
            Content::Tree(_) => 13,
            Content::Garbage(_) => 14,
            Content::Fire => 15,
            Content::Coin(_) => 16,
            Content::Bin(_) => 17,
            Content::Crate(_) => 18,
            Content::Bank(_) => 19,
            Content::Water(_) => 20,
            Content::Market(_) => 21,
            Content::Fish(_) => 22,
            Content::Building => 23,
            Content::Bush(_) => 24,
            Content::JollyBlock(_) => 25,
            Content::Scarecrow => 26,
            Content::None => 27,
        }
    }
    pub fn get_robot_sprite_index(&self) -> usize {
        28
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let atlas_handle = atlases.add(TextureAtlas::from_grid(
        asset_server.load(FILE_NAME),
        TILE_SIZE,
        COLUMNS,
        ROWS,
        None,
        None,
    ));
    commands.insert_resource(SpriteSheet {
        atlas: atlas_handle,
    });
}
