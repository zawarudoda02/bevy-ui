

use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{EventReader, IntoSystemConfigs, KeyCode, NextState, Res, ResMut, State};
use bevy::prelude::{
    in_state, Condition, IntoSystemSetConfigs, OnEnter, OnExit, Startup, States, SystemSet, Update,
};
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use bevy::input::Input;

//Represents the states of the ui
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, Hash, States)]
pub enum UiStates {
    #[default]
    MainMenu,
    AwaitingFirstMessage,
    Setup,
    Lifecycle,
    End,
}

///
///System sets for a clean execution of the program
///
#[derive(Debug, Clone, Copy, Eq, PartialEq, SystemSet, Hash)]
pub enum UiSystemSet {
    //here will go the assetloader initialization, the server starting to listen etc
    UiStartup,
    //all the systems that will spawn the main menu
    MainMenuBegin,
    MainMenuCycle,
    //main menu cleanup
    MainMenuEnd,

    WorldSetup,
    GridSetup,
    RobotSetup,
    TilesSetup,
    BeginLifeCycle,
    LifeCycle,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, SystemSet, Hash)]
pub enum LifeCycleSets {
    UpdateTick,
    Robot,
    Tiles,
    Backpack,
    Errors,
    ControlFlow,
}
fn end_it(keys: Res<Input<KeyCode>>, mut state: ResMut<NextState<UiStates>>){
    if keys.just_pressed(KeyCode::Q){
        state.set(UiStates::End);
    }
}
pub struct SchedulePlugin;
impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, UiSystemSet::UiStartup)
            .configure_sets(OnEnter(UiStates::MainMenu), UiSystemSet::MainMenuBegin)
            .configure_sets(OnExit(UiStates::MainMenu), UiSystemSet::MainMenuEnd)
            .configure_sets(
                OnEnter(UiStates::Setup),
                (
                    UiSystemSet::WorldSetup,
                    UiSystemSet::GridSetup,
                    UiSystemSet::RobotSetup,
                    UiSystemSet::TilesSetup,
                    UiSystemSet::BeginLifeCycle,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (UiSystemSet::LifeCycle).run_if(
                    in_state(UiStates::Lifecycle).and_then(on_timer(Duration::from_millis(50))),
                ),
            )
            .configure_sets(
                Update,
                (
                    LifeCycleSets::UpdateTick,
                    LifeCycleSets::Robot,
                    LifeCycleSets::Tiles,
                    LifeCycleSets::Backpack,
                    LifeCycleSets::Errors,
                    LifeCycleSets::ControlFlow,
                )
                    .chain()
                    .in_set(UiSystemSet::LifeCycle),
            )
            .add_systems(Update,end_it);
    }
}
