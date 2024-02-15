use bevy::app::App;
use bevy::math::UVec2;
use bevy::prelude::{Commands, Entity, IntoSystemConfigs, NonSend, OnEnter, Plugin, Res, ResMut, Resource, Visibility};
use bevy::sprite::SpriteBundle;

use bevy::utils::default;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Tile;
use ui_and_robot_communication::Message;
use crate::robot::res::RobotPosition;
use crate::server::{Ticks, UiServer};
use crate::states::{UiStates, UiSystemSet};
use crate::world::tiles::{GridPosition, TileBundle};


#[derive(Resource)]
pub struct WorldInfo{
   pub  edge_size:usize
}
#[derive(Resource)]
pub struct WorldTiles{
    pub vec: Vec<Vec<(Option<Tile>,Entity)>>
}
#[derive(Resource)]
pub struct CurrentWeather{
    conditions: WeatherType
}
//function to be run after first messages retrieved
fn get_world_info(mut commands: Commands,mut ticks:  ResMut<Ticks>){
    if let Some(x) = ticks.pop(){
        let mut  iter = x.into_iter();
        if let (Some(_),Some(Message::WorldInfo {spawn_point,world_size})) = (iter.next(),iter.next()){
            commands.insert_resource(WorldInfo{
                edge_size: world_size
            });
            commands.insert_resource(RobotPosition{
                position: spawn_point
            });
        }
    }
}
fn create_tile_map(mut commands: Commands,world_info:  Res<WorldInfo>){
    let mut  vec=  Vec::with_capacity(world_info.edge_size);

    for col in 0..world_info.edge_size{
        let mut  v = Vec::with_capacity(world_info.edge_size);
        for row in 0..world_info.edge_size{
            v.push((None,commands.spawn( GridPosition(UVec2::new(col as u32, row as u32))).id()));
        }
        vec.push(v);
    }

    commands.insert_resource(WorldTiles{vec});
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(UiStates::Setup),(get_world_info,create_tile_map).chain().in_set(UiSystemSet::WorldSetup));

    }
}