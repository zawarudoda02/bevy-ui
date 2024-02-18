use bevy::prelude::*;
use crate::robot::res::RobotMarker;
use crate::states::{UiStates, UiSystemSet};

const CAMERA_SPEED: f32 = 200.0;
pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){

        app.add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(UiStates::Lifecycle),center_on_robot)
            .add_systems(Update, follow_robot.in_set(UiSystemSet::LifeCycle));

    }
}

#[derive(Component)]
struct CameraMarker;
fn spawn_camera(mut commands: Commands){
    commands.spawn((Camera2dBundle::default(),CameraMarker));
}

fn center_on_robot(query: Query<&Transform,With<RobotMarker>>,mut camera_query: Query<&mut Transform,(With<CameraMarker>,Without<RobotMarker>)>){
    let mut camera = camera_query.get_single_mut().unwrap();
    let robot = query.get_single().unwrap();
    camera.translation = robot.translation;
}
fn follow_robot(query: Query<&Transform,With<RobotMarker>>,mut camera_query: Query<&mut Transform,(With<CameraMarker>,Without<RobotMarker>)>, time:Res<Time>){
    let mut camera = camera_query.get_single_mut().unwrap();
    let robot = query.get_single().unwrap();
    let distance = robot.translation.distance(camera.translation);
    if distance> 1.0{
        let vector = (robot.translation- camera.translation).normalize_or_zero();

        camera.translation = camera.translation.lerp(robot.translation,0.1);
    }
}
