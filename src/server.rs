use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, RecvError};
use std::thread;
use std::thread::JoinHandle;
use bevy::prelude::*;
use ui_and_robot_communication::{CommError, Message, Server, Tick};
use crate::states::UiSystemSet;

#[test]
fn test(){

}


pub struct UiServer{
    pub receiver: Receiver<Result<Tick,CommError>>,
    thread: JoinHandle<()>
}
impl UiServer{
    fn new()-> Self{
        let (tx,rx) = channel();
        let th = thread::spawn(move || {
            let mut  server = Server::new();
            server.begin_listening().expect("Client can't be accepted");
            loop {
                let a = server.get_tick();
                match a{
                    Ok(x) => {tx.send(Ok(x)).expect("Send between threads failed");}
                    Err(e) => {tx.send(Err(e)).expect("Send between threads failed"); break;}
                }
            }
        } );
        Self{
            receiver:rx,
            thread: th
        }
    }
}

fn setup_server(world: &mut World){
   let server = UiServer::new();
    world.insert_non_send_resource(server);

}

#[derive(Resource)]
pub struct Ticks{
    ticks: VecDeque<Tick>
}
impl Ticks{
    pub fn push(&mut self,tick: Tick){
        self.ticks.push_front(tick)
    }
    pub fn pop(&mut self) ->Option<Tick>{
        self.ticks.pop_back()
    }
}

fn retrieve_ticks(server: NonSend<UiServer>,mut ticks: ResMut<Ticks>){
    while let Ok(x)= server.receiver.try_recv() {
       if let Ok(y)= x{
           ticks.push(y)
       }
    }
    //TODO ERROR HANDLING
}
pub struct ServerPlugin;
impl Plugin for ServerPlugin{
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(UiServer::new())
            .add_systems(Update,retrieve_ticks.in_set(UiSystemSet::LifeCycle));
    }
}