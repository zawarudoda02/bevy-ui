use crate::asset_loader::SpriteSheet;
use crate::grid::Grid;
use crate::lifecycle::CurrentTick;
use crate::states::{LifeCycleSets, UiStates, UiSystemSet};
use crate::world::tiles::GridPosition;
use bevy::prelude::*;
use ui_and_robot_communication::{LibEvent, Message};
const DEFAULT_ENERGY: usize = 1000;
#[derive(Resource)]
pub struct RobotPosition {
    pub position: (usize, usize),
}
#[derive(Resource)]
pub struct RobotEnergy {
    pub energy: usize,
}

pub fn setup_robot(mut commands: Commands, position: Res<RobotPosition>) {
    commands.insert_resource(RobotEnergy {
        energy: DEFAULT_ENERGY,
    })
}

#[derive(Bundle)]
pub struct RobotBundle {
    marker: RobotMarker,
    sprite: SpriteSheetBundle,
    position: GridPosition,
}
#[derive(Component)]
pub struct RobotMarker;

pub fn spawn_robot(
    mut commands: Commands,
    position: Res<RobotPosition>,
    sheet: Res<SpriteSheet>,
    grid: Res<Grid>,
) {
    let robot = RobotBundle {
        marker: RobotMarker,
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sheet.get_robot_sprite_index()),

            texture_atlas: sheet.atlas.clone(),
            transform: Transform::from_translation(
                grid.compute_position(position.position.0 as u32, position.position.1 as u32)
                    .extend(3.0),
            ),
            ..default()
        },
        position: GridPosition(UVec2::new(
            position.position.0 as u32,
            position.position.1 as u32,
        )),
    };
    commands.spawn(robot);
}

pub struct RobotPlugin;
impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(UiStates::Setup),
            (setup_robot, apply_deferred, spawn_robot, apply_deferred)
                .chain()
                .in_set(UiSystemSet::RobotSetup),
        )
        .add_systems(
            Update,
            (update_robot, update_all_energy_in_a_row, update_energy)
                .chain()
                .in_set(LifeCycleSets::Robot),
        );
    }
}

fn update_robot(
    mut current_tick: ResMut<CurrentTick>,
    mut robot_transform: Query<&mut Transform, With<RobotMarker>>,
    mut robot_position: ResMut<RobotPosition>,
    grid: Res<Grid>,
) {
    match current_tick.peek() {
        None => {
            return;
        }
        Some(message) => match message {
            Message::LibEvent(LibEvent::Moved(_, (col, row))) => {
                warn!(
                    "Robot is moving from {:?} to {:?}",
                    robot_position.position,
                    (col, row)
                );

                let (col, row) = (*col, *row);
                robot_position.position = (col, row);
                robot_transform.get_single_mut().unwrap().translation =
                    grid.compute_position(col as u32, row as u32).extend(3.0);
            }

            _ => {
                return;
            }
        },
    }
    current_tick.pop();
}

fn update_energy(mut current_tick: ResMut<CurrentTick>, mut robot_energy: ResMut<RobotEnergy>) {
    match current_tick.peek() {
        None => {
            return;
        }
        Some(message) => match message {
            Message::LibEvent(LibEvent::EnergyConsumed(x)) => {
                warn!("The robot has consumed {:?} energy", x);
                robot_energy.energy = robot_energy.energy.checked_sub(*x).unwrap_or_else(|| 0);
            }
            Message::LibEvent(LibEvent::EnergyRecharged(x)) => {
                warn!("The robot has recharged {:?} energy", x);
                robot_energy.energy += x;
            }
            _ => {
                return;
            }
        },
    }
    current_tick.pop();
}

fn update_all_energy_in_a_row(
    mut current_tick: ResMut<CurrentTick>,
    mut robot_energy: ResMut<RobotEnergy>,
) {
    while let Some(Message::LibEvent(LibEvent::EnergyConsumed(x))) = current_tick.peek() {
        warn!("The robot has consumed {:?} energy", x);
        robot_energy.energy = robot_energy.energy.checked_sub(*x).unwrap_or_else(|| 0);
        current_tick.pop();
    }
    while let Some(Message::LibEvent(LibEvent::EnergyRecharged(x))) = current_tick.peek() {
        warn!("The robot has consume {:?} energy", x);
        robot_energy.energy += x;
        current_tick.pop();
    }
}
