use crate::asset_loader::SpriteSheet;
use crate::grid::Grid;
use crate::lifecycle::CurrentTick;
use crate::states::{LifeCycleSets, UiStates, UiSystemSet};
use crate::world::res::WorldTiles;
use bevy::prelude::*;
use robotics_lib::world::tile::{Content, Tile};
use ui_and_robot_communication::{LibEvent, Message};

#[derive(Bundle)]
pub struct TileBundle {
    marker: TileMarker,
    sprite: SpriteSheetBundle,
    position: GridPosition,
}

#[derive(Debug, Component)]
pub struct GridPosition(pub UVec2);
#[derive(Component)]
pub struct TileMarker;

impl TileBundle {
    pub fn new(sprite: SpriteSheetBundle, position: GridPosition) -> Self {
        Self {
            marker: TileMarker,
            sprite,
            position,
        }
    }
}
//TODO setup tiles
//TODO update tiles sprite when it changes
pub fn setup_tiles(
    mut commands: Commands,
    map: Res<WorldTiles>,
    sprite_sheet: Res<SpriteSheet>,
    grid: Res<Grid>,
) {
    info!("Im going to setup THE TILES!!!");
    for (col, col_vec) in map.vec.iter().enumerate() {
        for (row, (_, e)) in col_vec.iter().enumerate() {
            info!("setting up tile: {:?}", e);
            commands.get_entity(*e).unwrap().insert(TileBundle::new(
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(sprite_sheet.get_tiletype_sprite_index(None)),
                    texture_atlas: sprite_sheet.atlas.clone(),
                    transform: Transform {
                        translation: grid.compute_position(col as u32, row as u32).extend(0.),
                        ..default()
                    },
                    ..default()
                },
                GridPosition(UVec2::new(col as u32, row as u32)),
            ));
        }
    }
}
pub struct TilesPlugin;
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(UiStates::Setup),
            (setup_tiles, apply_deferred, setup_content)
                .chain()
                .in_set(UiSystemSet::TilesSetup),
        )
        .add_systems(Update, update_tiles.in_set(LifeCycleSets::Tiles));
    }
}
#[derive(Component)]
pub struct ContentMarker;
#[derive(Bundle)]
struct ContentBundle {
    marker: ContentMarker,
    sprite: SpriteSheetBundle,
}
impl ContentBundle {
    pub fn new(sprite: SpriteSheetBundle) -> Self {
        Self {
            marker: ContentMarker,
            sprite,
        }
    }
}
fn setup_content(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<TileMarker>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (e, t) in &query {
        info!("setting up content for {:?}", e);
        let content = commands
            .spawn(ContentBundle::new(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(
                    sprite_sheet.get_content_sprite_index(Content::None),
                ),
                texture_atlas: sprite_sheet.atlas.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    ..default()
                },
                ..default()
            }))
            .id();

        commands.entity(e).push_children(&[content]);
    }
}

fn update_tiles(
    mut world_tiles: ResMut<WorldTiles>,
    mut curr_tick: ResMut<CurrentTick>,
    mut param_set: ParamSet<(
        Query<(&mut TextureAtlasSprite, &Children), (With<TileMarker>)>,
        Query<&mut TextureAtlasSprite, (With<ContentMarker>)>,
    )>,
    sprite_sheet: Res<SpriteSheet>,
) {
    match curr_tick.peek() {
        None => {
            return;
        }
        Some(message) => {
            match message {
                Message::LibEvent(LibEvent::DiscoveredTiles(x)) => {
                    /*
                    for (tile,(col,row)) in x {
                        let (col,row) = (*col,*row);
                        world_tiles.vec[col][row].0 =Some( tile.clone());
                        let entity = world_tiles.vec[col][row].1;

                        let (mut sprite,children) = tiles_query.get_mut(entity).unwrap();

                        sprite.index = sprite_sheet.get_tiletype_sprite_index(Some(tile.tile_type.clone()));
                        let mut content_sprite = content_query.get_mut(*children.first().unwrap()).unwrap();
                        content_sprite.index = sprite_sheet.get_content_sprite_index(tile.content.clone());
                    }
                    */
                    warn!("I've discovered some tiles! i'm going to update them!!");
                    for (tile, (col, row)) in x {
                        let (col, row) = (*col, *row);
                        world_tiles.vec[col][row].0 = Some(tile.clone());
                        let entity = world_tiles.vec[col][row].1;

                        let mut tile_query = param_set.p0();
                        let (mut sprite, children) = tile_query.get_mut(entity).unwrap();
                        sprite.index =
                            sprite_sheet.get_tiletype_sprite_index(Some(tile.tile_type.clone()));
                        let child = *children.first().unwrap();

                        let mut content_query = param_set.p1();
                        let mut content_sprite = content_query.get_mut(child).unwrap();
                        content_sprite.index =
                            sprite_sheet.get_content_sprite_index(tile.content.clone());
                    }
                }

                Message::LibEvent(LibEvent::TileContentUpdated(tile, (col, row))) => {
                    /*
                    let entity: Entity = world_tiles.vec[*col][*row].1;
                    world_tiles.vec[*col][*row].0 = Some(tile.clone());
                    let (a,b)= tiles_query.get(entity).unwrap();
                    let mut content_sprite = content_query.get_mut(*b.first().unwrap()).unwrap();
                    content_sprite.index = sprite_sheet.get_content_sprite_index(tile.content.clone());
                    */
                    warn!("The content of a tile has been updated! wish me luck!!");
                    let (col, row) = (*col, *row);
                    world_tiles.vec[col][row].0 = Some(tile.clone());
                    let entity = world_tiles.vec[col][row].1;
                    let mut tiles_query = param_set.p0();
                    let (mut sprite, children) = tiles_query.get_mut(entity).unwrap();
                    sprite.index =
                        sprite_sheet.get_tiletype_sprite_index(Some(tile.tile_type.clone()));
                    let child = *children.first().unwrap();

                    let mut content_query = param_set.p1();
                    let mut content_sprite = content_query.get_mut(child).unwrap();
                    content_sprite.index =
                        sprite_sheet.get_content_sprite_index(tile.content.clone());
                }
                _ => {
                    return;
                }
            }
        }
    }

    curr_tick.pop();
}
