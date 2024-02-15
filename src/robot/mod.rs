pub mod res;

use bevy::prelude::*;
use crate::world::tiles::GridPosition;

#[derive(Component)]
pub struct Robot;

#[derive(Bundle)]
pub struct RobotBundle{
    robot: Robot,
    sprite: SpriteBundle,
    position: GridPosition
}

impl RobotBundle{
    pub fn new(sprite:SpriteBundle,position: GridPosition)->Self{
        Self{
            robot: Robot,
            sprite,
            position
        }
    }
}
