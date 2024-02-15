use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemSetConfigs, OnEnter, OnExit, Schedule, Startup, States, SystemSet, Update};
use crate::states::UiStates::Setup;
use crate::states::UiSystemSet::UiStartup;


//Represents the states of the ui
#[derive(Default,Debug,Eq,PartialEq,Clone,Copy,Hash,States)]
pub enum UiStates{
    #[default]
    MainMenu,
    Setup,
    Lifecycle,
    End
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,SystemSet,Hash)]
pub enum UiSystemSet{
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

    LifeCycle

}



pub struct SchedulePlugin;
impl Plugin for SchedulePlugin{
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,UiSystemSet::UiStartup
        ).configure_sets(
            OnEnter(UiStates::MainMenu),
                UiSystemSet::MainMenuBegin
        ).configure_sets(
            OnExit(UiStates::MainMenu),
                UiSystemSet::MainMenuEnd
        ).configure_sets(
            OnEnter(UiStates::Setup),
            (UiSystemSet::WorldSetup,UiSystemSet::GridSetup,UiSystemSet::RobotSetup,UiSystemSet::TilesSetup).chain()
        ).configure_sets(

            Update,(UiSystemSet::LifeCycle)
        )
        ;

    }
}