use bevy::prelude::*;
use robotics_lib::world::tile::Tile;
use crate::asset_loader::SpriteSheet;
use crate::grid::Grid;
use crate::states::{UiStates, UiSystemSet};
use crate::world::res::WorldTiles;

#[derive(Bundle)]
pub struct TileBundle{
    marker: TileMarker,
    sprite: SpriteSheetBundle,
    position: GridPosition,
}

#[derive(Component)]
pub struct GridPosition(pub UVec2);
#[derive(Component)]
struct TileMarker;

impl TileBundle{
    pub fn new(sprite:SpriteSheetBundle,position: GridPosition)->Self{
        Self{
            marker: TileMarker,
            sprite,
            position
        }
    }
}
//TODO setup tiles
//TODO update tiles sprite when it changes
pub fn setup_tiles(mut commands: Commands, map: Res<WorldTiles>,atlas: Res<SpriteSheet>,grid: Res<Grid>){

    for (col,col_vec) in map.vec.iter().enumerate(){
        for (row,(_,e)) in col_vec.iter().enumerate(){
            commands.get_entity(*e).unwrap().insert(
                TileBundle::new(
                    SpriteSheetBundle{
                        sprite: TextureAtlasSprite::new(atlas.get_tiletype_sprite_index(None)) ,
                        texture_atlas: atlas.atlas.clone(),
                        transform: Transform{
                            translation: grid.compute_position(col as u32,row as u32).extend(0.),
                            rotation: Default::default(),
                            scale: Default::default(),
                        } ,
                        ..default()
                    }
                    ,
                    GridPosition(UVec2::new(col as u32,row as u32))
                )
            );
        }
    }
}
pub struct TilesPlugin;
impl Plugin for TilesPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiStates::Setup),setup_tiles.in_set(UiSystemSet::TilesSetup));
    }
}