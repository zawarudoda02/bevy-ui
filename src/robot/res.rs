use bevy::prelude::*;
use crate::asset_loader::SpriteSheet;
use crate::grid::Grid;
use crate::states::{UiStates, UiSystemSet};
use crate::world::tiles::GridPosition;
const DEFAULT_ENERGY: usize = 1000;
 #[derive(Resource)]
pub struct RobotPosition{
    pub position:(usize,usize),
}
#[derive(Resource)]
pub struct RobotEnergy{
    pub energy: usize
}

pub fn setup_robot(mut commands: Commands, position: Res<RobotPosition>){
    commands.insert_resource(RobotEnergy{energy:DEFAULT_ENERGY})
}

#[derive(Bundle)]
pub struct RobotBundle{
    marker: RobotMarker,
    sprite: SpriteSheetBundle,
    position: GridPosition,
}
#[derive(Component)]
pub struct RobotMarker;

pub fn spawn_robot(mut commands: Commands,position:  Res<RobotPosition>,sheet: Res<SpriteSheet>,grid: Res<Grid>){
    let robot = RobotBundle{
        marker: RobotMarker,
        sprite: SpriteSheetBundle{
            sprite: TextureAtlasSprite::new(sheet.get_robot_sprite_index()),

            texture_atlas: sheet.atlas.clone(),
            transform: Transform::from_translation(grid.compute_position(position.position.0 as u32,position.position.1 as u32).extend(2.0)),
            ..default()
        },
        position: GridPosition(UVec2::new(position.position.0 as u32,position.position.1 as u32)),
    };
    commands.spawn(robot);
}

pub struct RobotPlugin;
impl Plugin for RobotPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiStates::Setup),(setup_robot,spawn_robot).chain().in_set(UiSystemSet::RobotSetup));

    }
}

