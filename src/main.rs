mod world;
mod states;
mod main_menu;
mod camera;
mod server;
mod robot;
mod grid;
mod asset_loader;


use bevy::DefaultPlugins;
use bevy::prelude::App;
use ui_and_robot_communication;
use crate::camera::CameraPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::states::{SchedulePlugin, UiStates};
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
    App::new()


        .add_plugins(DefaultPlugins)
        .add_state::<UiStates>()
        .add_plugins(SchedulePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MainMenuPlugin)

        .run();

    println!("Hello, world!");
}
