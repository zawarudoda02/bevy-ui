//resource describing the grid layout

use bevy::prelude::*;
use crate::states::{UiStates, UiSystemSet};

use crate::world::res::WorldInfo;

const TILE_SIZE: f32 = 32.0;
const BOTTOM_LEFT: Vec2 = Vec2::ZERO;
#[derive(Resource)]
pub struct Grid{
    grid_edge_size:u32,
    rect: Rect
}
impl Grid{
    pub fn new(rect: Rect,grid_edge_size:u32)->Self{
        Self{
            grid_edge_size,
            rect
        }
    }
    pub fn compute_position(&self, col:u32,row:u32)->Vec2{
        let tile_side = self.rect.width()/ self.grid_edge_size as f32;
        return Vec2::new(col as f32, row as f32) * tile_side;
    }
    pub fn get_tile_size()->f32{
        TILE_SIZE
    }
}

pub fn create_grid(mut commands: Commands,world_info :Res<WorldInfo>){
    let top_right = Vec2::new(world_info.edge_size as f32 * TILE_SIZE, world_info.edge_size as f32 * TILE_SIZE);
    let grid_rect = Rect::from_corners(BOTTOM_LEFT, top_right);
    commands.insert_resource(Grid::new(grid_rect,world_info.edge_size as u32));
}

pub struct GridPlugin;
impl Plugin for GridPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiStates::Setup),create_grid.in_set(UiSystemSet::GridSetup));
    }
}