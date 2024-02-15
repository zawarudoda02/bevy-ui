use bevy::prelude::*;
const CAMERA_DISTANCE: f32 = 80.0;
pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_camera);

    }
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}