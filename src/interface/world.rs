use crate::asset_loader::SpriteSheet;
use crate::grid::Grid;
use crate::states::{LifeCycleSets, UiStates};
use crate::world::res::{CurrentWeather, WorldTiles};
use crate::world::tiles::{GridPosition, TileMarker};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::Display::Flex;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_mouse_tracking_plugin::{MousePos, MousePosWorld};
use robotics_lib::world::tile::{Content, Tile};

#[derive(Component)]
struct WeatherUiMarker;

#[derive(Component)]
struct CurrentTimeMarker;
#[derive(Component)]
struct CurrentWeatherMarker;
pub fn spawn_weather_ui() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    Box::new(|parent| {
        //current time
        parent.spawn((
            TextBundle::from_section(
                "Time : ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            CurrentTimeMarker,
        ));

        parent.spawn((
            TextBundle::from_section(
                "Weather : ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            CurrentWeatherMarker,
        ));
    })
}
fn update_time(weather: Res<CurrentWeather>, mut query: Query<&mut Text, With<CurrentTimeMarker>>) {
    if let Some(weather) = &weather.conditions {
        let mut text = query.get_single_mut().unwrap();
        text.sections[0].value = format!(
            "Time : {:?} || {:?}",
            weather.get_time_of_day(),
            weather.get_time_of_day_string()
        );
    }
}
fn update_weather(
    weather: Res<CurrentWeather>,
    mut query: Query<&mut Text, With<CurrentWeatherMarker>>,
) {
    if let Some(weather) = &weather.conditions {
        let mut text = query.get_single_mut().unwrap();
        text.sections[0].value = format!("Weather : {:?}", weather.get_weather_condition());
    }
}

pub struct WorldUiPlugin;

impl Plugin for WorldUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_weather, update_time)
                .chain()
                .in_set(LifeCycleSets::Tiles),
        )
        .add_systems(
            OnExit(UiStates::Setup),
            (spawn_hovered_img_and_text, apply_deferred).chain(),
        )
        .add_systems(
            Update,
            (update_hovered).run_if(in_state(UiStates::Lifecycle)),
        );
    }
}

#[derive(Component)]
struct HoveredTileContainerMarker;
#[derive(Component)]
struct HoveredTileTypeImgMarker;

#[derive(Component)]
struct HoveredContentImgMarker;
#[derive(Component)]
struct HoveredTileTextMarker;
pub fn spawn_hovered_tile() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    Box::new(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    align_items: AlignItems::FlexEnd,
                    flex_direction: FlexDirection::Row,

                    width: Val::Percent(100.0),
                    margin: UiRect::bottom(Val::Auto),
                    ..default()
                },
                visibility: Visibility::Visible,
                border_color: BorderColor(Color::WHITE),
                ..default()
            },
            HoveredTileContainerMarker,
        ));
    })
}

fn spawn_hovered_img_and_text(
    mut commands: Commands,
    atlas: Res<SpriteSheet>,
    query: Query<Entity, With<HoveredTileContainerMarker>>,
) {
    let container_entity = query.single();
    commands.entity(container_entity).with_children(|parent| {
        parent
            .spawn((
                AtlasImageBundle {
                    style: Style {
                        width: Val::Percent(20.),
                        height: Val::Auto,
                        ..default()
                    },

                    texture_atlas: atlas.atlas.clone(),
                    texture_atlas_image: UiTextureAtlasImage {
                        index: atlas.get_content_sprite_index(Content::None),
                        ..default()
                    },
                    ..default()
                },
                HoveredTileTypeImgMarker,
            ))
            .with_children(|parent| {
                parent.spawn((
                    AtlasImageBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },

                        texture_atlas: atlas.atlas.clone(),
                        texture_atlas_image: UiTextureAtlasImage {
                            index: atlas.get_content_sprite_index(Content::None),
                            ..default()
                        },
                        ..default()
                    },
                    HoveredContentImgMarker,
                ));
            });
        parent.spawn((
            TextBundle::from_section(
                "Hover over a tile to see information",
                TextStyle {
                    font_size: 15.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            HoveredTileTextMarker,
        ));
    });
}

fn update_hovered(
    mouse_pos: Res<MousePosWorld>,
    grid: Res<Grid>,
    mut hovered_tt_img: Query<
        &mut UiTextureAtlasImage,
        (
            With<HoveredTileTypeImgMarker>,
            Without<HoveredContentImgMarker>,
        ),
    >,
    mut hovered_content_img: Query<
        &mut UiTextureAtlasImage,
        (
            With<HoveredContentImgMarker>,
            Without<HoveredTileTypeImgMarker>,
        ),
    >,
    mut hovered_text: Query<&mut Text, With<HoveredTileTextMarker>>,

    sprite_sheet: Res<SpriteSheet>,
    map: Res<WorldTiles>,
) {
    let mut hovered_tt_img = hovered_tt_img.single_mut();
    let mut hovered_content_img = hovered_content_img.single_mut();
    let mut hovered_text = hovered_text.single_mut();

    let position = grid.compute_inverse_position(Vec2::new(mouse_pos.x, mouse_pos.y));

    if let Some(position) = position {
        let (col, row) = (position.0 as usize, position.1 as usize);
        let tile = map.vec[col][row].0.clone();
        hovered_tt_img.index =
            sprite_sheet.get_tiletype_sprite_index(tile.clone().map(|x| x.tile_type));
        hovered_content_img.index = sprite_sheet
            .get_content_sprite_index(tile.clone().map(|x| x.content).unwrap_or(Content::None));
        let (tiletype, content, elevation) = match tile {
            None => (
                "unknown".to_string(),
                "unknown".to_string(),
                "unknown".to_string(),
            ),
            Some(t) => (
                format!("{:?}", t.tile_type),
                format!("{:?}", t.content),
                format!("{:?}", t.elevation),
            ),
        };
        let mut text = format!(
            "position: {:?} \n tile type: {} \n content: {} \n elevation: {}",
            (col, row),
            tiletype,
            content,
            elevation
        );

        hovered_text.sections[0].value = text;
    }
}
