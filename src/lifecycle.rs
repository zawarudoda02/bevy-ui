/*
IDEAS
resource current tick: contains the tick iterator
let's make it peekable for good measure
first system: we check if the tick is finished, and we take a new one
following systems: parsing the message based on their "area"
eg: robot system: will move the robot/change it's health
tile system: will update tiles
error system
etc
each one of these systems will read the "next" message and decide whether or not to pop it

 */

use crate::server::Ticks;
use crate::states::{LifeCycleSets, UiStates, UiSystemSet};
use bevy::app::{App, Plugin};
use bevy::prelude::{
    error, info, warn, IntoSystemConfigs, NextState, OnEnter, ResMut, Resource, Update,
};
use std::iter::Peekable;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;
use ui_and_robot_communication::Message::LibError;
use ui_and_robot_communication::{LibEvent, Message, Tick};

#[derive(Debug, Default, Resource)]
pub struct CurrentTick {
    tick: Option<Peekable<IntoIter<Message>>>,
}
impl CurrentTick {
    fn new() -> Self {
        Self { tick: None }
    }
    pub fn peek(&mut self) -> Option<&Message> {
        let tick = self.tick.as_mut()?;
        tick.peek()
    }
    pub fn pop(&mut self) -> Option<Message> {
        let tick = self.tick.as_mut()?;
        tick.next()
    }
    pub fn reload(&mut self, tick: Tick) {
        self.tick = Some(tick.into_iter().peekable());
    }
}
fn begin_lifecycle(mut next: ResMut<NextState<UiStates>>) {
    next.set(UiStates::Lifecycle);
}
fn update_tick(
    mut ticks: ResMut<Ticks>,
    mut curr_tick: ResMut<CurrentTick>,
    mut next_state: ResMut<NextState<UiStates>>,
) {
    warn!("I'm in system 'update_tick', trying to parse the message");
    if curr_tick.peek().is_none() {
        if let Some(x) = ticks.pop() {
            warn!("Reloaded tick!!");
            curr_tick.reload(x);
        }
    }
}
fn tick_control_flow(
    mut curr_tick: ResMut<CurrentTick>,
    mut next_state: ResMut<NextState<UiStates>>,
) {
    //warn!("I'm in system 'tick_control flow', trying to parse the message");
    match curr_tick.peek() {
        None => {
            return;
        }
        Some(message) => match message {
            Message::LibEvent(LibEvent::EndOfTick) => {
                info!("Tick ended! Awaiting next one")
            }
            Message::LibEvent(LibEvent::Terminated) => {
                info!("Lifecycle terminated!");
                next_state.set(UiStates::End);
            }
            _ => {
                return;
            }
        },
    }
    curr_tick.pop();
}

fn error_messages(mut curr_tick: ResMut<CurrentTick>) {
    match curr_tick.peek() {
        None => return,
        Some(x) => match x {
            Message::LibError(x) => {
                error!("{:?}", x);
            }
            _ => {
                return;
            }
        },
    }
    curr_tick.pop();
}
pub struct LifeCyclePlugin;
impl Plugin for LifeCyclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTick>()
            .add_systems(
                OnEnter(UiStates::Setup),
                begin_lifecycle.in_set(UiSystemSet::BeginLifeCycle),
            )
            .add_systems(Update, (update_tick).in_set(LifeCycleSets::UpdateTick))
            .add_systems(
                Update,
                (tick_control_flow).in_set(LifeCycleSets::ControlFlow),
            )
            .add_systems(Update, (error_messages).in_set(LifeCycleSets::ControlFlow));
    }
}
