//resource describing the grid layout

use crate::states::{UiStates, UiSystemSet};
use bevy::prelude::*;


use crate::world::res::WorldInfo;

const TILE_SIZE: f32 = 32.0;
const BOTTOM_LEFT: Vec2 = Vec2::ZERO;


///An abstract representation of the simulation's tile grid.
///Helpful for computing the pixel position of the tiles in the simulation world
#[derive(Resource)]
pub struct Grid {
    grid_edge_size: u32,
    rect: Rect,
}
impl Grid {
    pub fn new(rect: Rect, grid_edge_size: u32) -> Self {
        Self {
            grid_edge_size,
            rect,
        }
    }

    ///given the column and the row of a tile, it gives the 2D vector of it's position in simulation space
    pub fn compute_position(&self, col: u32, row: u32) -> Vec2 {
        let tile_side = self.rect.width() / self.grid_edge_size as f32;
        return ((Vec2::new(col as f32, row as f32)) * tile_side)
            + Vec2::new(tile_side / 2., tile_side / 2.);
    }


    pub fn compute_inverse_position(&self, vec: Vec2) -> Option<(u32, u32)> {
        let pos = ((vec - BOTTOM_LEFT) / (self.rect.width() / self.grid_edge_size as f32))
            .floor()
            .as_ivec2();
        if pos.x >= 0
            && pos.x < self.grid_edge_size as i32
            && pos.y >= 0
            && pos.y < self.grid_edge_size as i32
        {
            return Some((pos.x as u32, pos.y as u32));
        }
        None
    }
    pub fn get_tile_size() -> f32 {
        TILE_SIZE
    }
}

pub fn create_grid(mut commands: Commands, world_info: Res<WorldInfo>) {
    let top_right = Vec2::new(
        world_info.edge_size as f32 * TILE_SIZE,
        world_info.edge_size as f32 * TILE_SIZE,
    );
    let grid_rect = Rect::from_corners(BOTTOM_LEFT, top_right);
    commands.insert_resource(Grid::new(grid_rect, world_info.edge_size as u32));
}

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(UiStates::Setup),
            (create_grid, apply_deferred).in_set(UiSystemSet::GridSetup),
        );
    }
}
