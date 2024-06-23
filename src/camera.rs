use crate::robot::res::RobotMarker;
use crate::states::{UiStates, UiSystemSet};

use bevy::prelude::*;


use bevy_mouse_tracking_plugin::prelude::InitWorldTracking;
use bevy_mouse_tracking_plugin::{MainCamera};

const CAMERA_SPEED: f32 = 32.0;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CameraState>()
            .init_resource::<ScrollAmount>()
            .add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(UiStates::Lifecycle), center_on_robot)
            .add_systems(
                Update,
                follow_robot
                    .in_set(UiSystemSet::LifeCycle)
                    .run_if(in_state(CameraState::Following)),
            )
            .add_systems(Update, (switch_camera_mode, camera_scroll))
            .add_systems(Update, move_camera.run_if(in_state(CameraState::FreeMove)));
    }
}

#[derive(Component)]
pub struct CameraMarker;
fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((Camera2dBundle::default(), CameraMarker))
        .add(InitWorldTracking)
        .insert(MainCamera);
}

fn center_on_robot(
    query: Query<&Transform, With<RobotMarker>>,
    mut camera_query: Query<&mut Transform, (With<CameraMarker>, Without<RobotMarker>)>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();
    let robot = query.get_single().unwrap();
    camera.translation = robot.translation;
}
fn follow_robot(
    query: Query<&Transform, With<RobotMarker>>,
    mut camera_query: Query<&mut Transform, (With<CameraMarker>, Without<RobotMarker>)>,
    _time: Res<Time>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();
    let robot = query.get_single().unwrap();
    let distance = robot.translation.distance(camera.translation);
    if distance > 1.0 {
        let _vector = (robot.translation - camera.translation).normalize_or_zero();

        camera.translation = camera.translation.lerp(robot.translation, 0.1);
    }
}


fn switch_camera_mode(
    current_state: Res<State<CameraState>>,
    mut next_state: ResMut<NextState<CameraState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        next_state.set(match current_state.get() {
            CameraState::Following => CameraState::FreeMove,
            CameraState::FreeMove => CameraState::Following,
        })
    }
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut movement = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::Up) {
        movement += Vec2::new(0., CAMERA_SPEED);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        movement += Vec2::new(0., -CAMERA_SPEED);
    }
    if keyboard_input.pressed(KeyCode::Left) {
        movement += Vec2::new(-CAMERA_SPEED, 0.);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        movement += Vec2::new(CAMERA_SPEED, 0.);
    }
    let mut camera = camera_query.single_mut();
    camera.translation += movement.extend(0.);
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, Hash, States)]
enum CameraState {
    #[default]
    Following,
    FreeMove,
}

const MIN_SCROLL_AMOUNT: usize = 0;
const MAX_SCROLL_AMOUNT: usize = 10;

const DEFAULT_SCROLL_AMOUNT: usize = 5;

const CAMERA_SCROLL_SPEED: f32 = 1.5;

#[derive(Resource)]
struct ScrollAmount(usize);
impl Default for ScrollAmount {
    fn default() -> Self {
        Self(DEFAULT_SCROLL_AMOUNT)
    }
}

fn camera_scroll(
    mut camera_query: Query<&mut OrthographicProjection, With<CameraMarker>>,
    keys: Res<Input<KeyCode>>,
    mut amount: ResMut<ScrollAmount>,
) {
    let mut camera = camera_query.single_mut();
    let mut log_scale = camera.scale.ln();
    if keys.just_pressed(KeyCode::NumpadAdd) && amount.0 < MAX_SCROLL_AMOUNT {
        log_scale += CAMERA_SCROLL_SPEED.ln();
        amount.0 += 1;
    }
    if keys.just_pressed(KeyCode::NumpadSubtract) && amount.0 > MIN_SCROLL_AMOUNT {
        log_scale -= CAMERA_SCROLL_SPEED.ln();
        amount.0 -= 1
    }
    camera.scale = log_scale.exp();
}
