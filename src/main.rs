mod world;
mod states;
mod main_menu;
mod camera;
mod server;
mod robot;
mod grid;
mod asset_loader;
mod lifecycle;
mod interface;


use std::process::Command;
use bevy::DefaultPlugins;
use bevy::prelude::App;
use ui_and_robot_communication;
use crate::asset_loader::SpriteMapPlugin;
use crate::camera::CameraPlugin;
use crate::grid::GridPlugin;
use crate::interface::UiPlugin;
use crate::lifecycle::LifeCyclePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::robot::backpack::BackPackPlugin;
use crate::robot::res::RobotPlugin;
use crate::server::ServerPlugin;
use crate::states::{SchedulePlugin, UiStates};
use crate::world::res::WorldPlugin;
use crate::world::tiles::TilesPlugin;
/*
IDEA DUMPING
the ui starts on the main menu, the tcplistener already ON
after clicking on a button, the selected ai will start
start listening for the first message, with world info
generate tiles, set their sprite and position on the grid
position robot at spawn point
start processing ticks
PROFIT



 */

fn main() {
    println!("{:?}",Command::new("..\\advanced_programming_ai-main\\target\\debug\\advanced_programming_ai.exe"));
    App::new()


        .add_plugins(DefaultPlugins)
        .add_state::<UiStates>()
        .add_plugins(SpriteMapPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(ServerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(TilesPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(RobotPlugin)
        .add_plugins(LifeCyclePlugin)
        .add_plugins(BackPackPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(UiPlugin)
        .run();

    println!("Hello, world!");
}
