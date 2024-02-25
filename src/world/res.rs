use bevy::app::App;
use bevy::math::UVec2;
use bevy::prelude::{
    apply_deferred, info, warn, Commands, Entity, IntoSystemConfigs, NonSend, OnEnter, Plugin, Res,
    ResMut, Resource, Update, Visibility, World,
};
use bevy::sprite::SpriteBundle;

use crate::lifecycle::CurrentTick;
use crate::robot::res::RobotPosition;
use crate::server::{Ticks, UiServer};
use crate::states::{LifeCycleSets, UiStates, UiSystemSet};
use crate::world::tiles::{GridPosition, TileBundle};
use bevy::utils::default;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use robotics_lib::world::tile::Tile;
use ui_and_robot_communication::{LibEvent, Message};

#[derive(Debug, Resource)]
pub struct WorldInfo {
    pub edge_size: usize,
}
#[derive(Debug, Resource)]
pub struct WorldTiles {
    pub vec: Vec<Vec<(Option<Tile>, Entity)>>,
}
#[derive(Default, Resource)]
pub struct CurrentWeather {
    pub conditions: Option<EnvironmentalConditions>,
}
//function to be run after first messages retrieved
fn get_world_info(mut commands: Commands, mut ticks: ResMut<Ticks>) {
    if let Some(x) = ticks.pop() {
        let mut iter = x.into_iter();
        let (a, b) = (iter.next(), iter.next());
        info!("the next two messages are: {:?} ; {:?}", a, b);
        if let (
            Some(Message::LibEvent(LibEvent::Ready)),
            Some(Message::WorldInfo {
                spawn_point,
                world_size,
            }),
        ) = (a, b)
        {
            commands.insert_resource(WorldInfo {
                edge_size: world_size,
            });
            commands.insert_resource(RobotPosition {
                position: spawn_point,
            });
            commands.insert_resource(CurrentWeather { conditions: None });
            info!("SUCCESS");
        } else {
            panic!("COULDN'T READ WORLD INFO!!!");
        }
    } else {
        panic!("NO MESSAGES!!!");
    }
}
fn create_tile_map(mut commands: Commands, world_info: Res<WorldInfo>) {
    let mut vec = Vec::with_capacity(world_info.edge_size);

    for col in 0..world_info.edge_size {
        let mut v = Vec::with_capacity(world_info.edge_size);
        for row in 0..world_info.edge_size {
            v.push((None, commands.spawn_empty().id()));
        }
        vec.push(v);
    }
    commands.insert_resource(WorldTiles { vec });
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(UiStates::Setup),
            (
                get_world_info,
                apply_deferred,
                create_tile_map,
                apply_deferred,
            )
                .chain()
                .in_set(UiSystemSet::WorldSetup),
        )
        .init_resource::<CurrentWeather>()
        .add_systems(Update, update_weather.in_set(LifeCycleSets::Tiles));
    }
}

fn update_weather(mut weather: ResMut<CurrentWeather>, mut tick: ResMut<CurrentTick>) {
    match tick.peek() {
        None => {
            return;
        }
        Some(message) => match message {
            Message::LibEvent(LibEvent::TimeChanged(x))
            | Message::LibEvent(LibEvent::DayChanged(x)) => {
                warn!("I've updated the weather conditions!");
                weather.conditions = Some(x.clone());
            }
            _ => {
                return;
            }
        },
    }

    tick.pop();
}
