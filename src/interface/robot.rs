use crate::robot::res::{RobotEnergy, RobotPosition};
use crate::states::LifeCycleSets;
use crate::world::res::WorldTiles;
use bevy::prelude::*;

#[derive(Component)]
struct EnergyUiMarker;

#[derive(Component)]
struct PositionUiMarker;

pub fn spawn_robot_ui() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    Box::new(|parent| {
        parent.spawn(TextBundle::from_section(
            "Robot Info",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ));
        parent
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                visibility: Visibility::Visible,

                ..default()
            })
            .with_children(spawn_energy_ui())
            .with_children(spawn_position_ui());
    })
}
fn spawn_energy_ui() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    let text = "Energy: 1000";
    Box::new(move |parent| {
        parent.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            EnergyUiMarker,
        ));
    })
}

fn spawn_position_ui() -> Box<dyn FnOnce(&mut ChildBuilder)> {
    let text = "x: \n y: \n elevation: ";
    Box::new(move |parent| {
        parent.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 15.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            PositionUiMarker,
        ));
    })
}

fn update_energy_ui(energy: Res<RobotEnergy>, mut query: Query<&mut Text, With<EnergyUiMarker>>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Energy : {:?}", energy.energy);
}

fn update_position_ui(
    position: Res<RobotPosition>,
    world: Res<WorldTiles>,
    mut query: Query<&mut Text, With<PositionUiMarker>>,
) {
    let pos = position.position;
    let elevation = world.vec[pos.0][pos.1]
        .0
        .clone()
        .map(|x| x.elevation.to_string())
        .unwrap_or("Unknown".into());
    let mut text = query.single_mut();
    text.sections[0].value = format!(" x: {} \n y: {} \n elevation: {} ", pos.0, pos.1, elevation);
}

pub struct RobotUiPlugin;
impl Plugin for RobotUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_energy_ui, update_position_ui).in_set(LifeCycleSets::Robot),
        );
    }
}
