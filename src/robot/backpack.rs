use crate::lifecycle::CurrentTick;
use crate::states::LifeCycleSets;
use bevy::app::App;
use bevy::prelude::{warn, IntoSystemConfigs, Plugin, ResMut, Resource, Update};
use robotics_lib::world::tile::Content;
use std::collections::HashMap;
use ui_and_robot_communication::{LibEvent, Message};

#[derive(Default, Resource)]
pub struct BackPack {
    contents: HashMap<Content, usize>,
}

impl BackPack {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_to_backpack(&mut self, c: Content, quantity: usize) {
        let stripped_content = c.to_default();

        self.contents
            .entry(stripped_content)
            .and_modify(|x| *x += quantity)
            .or_insert(quantity);
    }
    fn remove_from_backpack(&mut self, c: Content, quantity: usize) {
        let stripped_content = c.to_default();

        self.contents
            .entry(stripped_content)
            .and_modify(|x| *x = x.checked_sub(quantity).unwrap_or_else(|| 0))
            .or_insert(0);
    }
    pub fn get(&mut self, c: Content) -> Option<usize> {
        self.contents.get(&c.to_default()).map(|x| *x)
    }
    pub fn get_all(&self) -> Vec<(Content, usize)> {
        self.contents.clone().into_iter().collect()
    }
}

pub struct BackPackPlugin;
impl Plugin for BackPackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackPack>()
            .add_systems(Update, update_backpack.in_set(LifeCycleSets::Backpack));
    }
}

fn update_backpack(mut backpack: ResMut<BackPack>, mut current_tick: ResMut<CurrentTick>) {
    match current_tick.peek() {
        None => {
            return;
        }
        Some(message) => match message {
            Message::LibEvent(LibEvent::AddedToBackpack(c, a)) => {
                warn!("content added to backpack");
                backpack.add_to_backpack(c.clone(), *a);
            }
            Message::LibEvent(LibEvent::RemovedFromBackpack(c, a)) => {
                warn!("content removed from backpack");
                backpack.remove_from_backpack(c.clone(), *a);
            }
            _ => {
                return;
            }
        },
    }

    current_tick.pop();
}
