use crate::interface::backpack::BackPackUiPlugin;
use crate::interface::robot::RobotUiPlugin;
use crate::interface::world::WorldUiPlugin;
use bevy::prelude::*;
use interface::UiPlugin;

mod backpack;
mod interface;
mod robot;
mod world;

pub struct InterfacePlugin;
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, BackPackUiPlugin, WorldUiPlugin, RobotUiPlugin));
    }
}
