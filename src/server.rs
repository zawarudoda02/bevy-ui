use std::collections::VecDeque;
use std::io::{Error, ErrorKind};
use std::sync::mpsc::{channel, Receiver, RecvError};
use std::thread;
use std::thread::JoinHandle;
use bevy::prelude::*;
use ui_and_robot_communication::{CommError, Message, Server, Tick};
use crate::states::{UiStates, UiSystemSet};

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
                let a = server.get_world_info();
                match a {
                    Ok(x) => {tx.send(Ok(x)).expect("Send between threads failed"); break;}
                    Err(e) => {tx.send(Err(e)).expect("Send between threads failed")}
                }
            }
            loop {
                let a = server.get_tick();
                match a{
                    Ok(x) => {tx.send(Ok(x)).expect("Send between threads failed");}
                    Err(e) => {
                        if let CommError::DeserializationError(_) = e {
                            break;
                        }
                        tx.send(Err(e)).expect("Send between threads failed");
                    }
                }
            }
        } );
        Self{
            receiver:rx,
            thread: th
        }
    }
}



#[derive(Debug,Resource)]
pub struct Ticks{
    ticks: VecDeque<Tick>
}
impl Ticks{
    fn new()->Self{
        Self{ticks: VecDeque::new()}
    }
    pub fn push(&mut self,tick: Tick){
        self.ticks.push_front(tick)
    }
    pub fn pop(&mut self) ->Option<Tick>{
        self.ticks.pop_back()
    }
}

fn retrieve_ticks(server: NonSend<UiServer>,mut ticks: ResMut<Ticks>,state: Res<State<UiStates>>, mut next_state: ResMut<NextState<UiStates>>){
    if let Ok(x)= server.receiver.try_recv() {
       if let Ok(y)= x{
           ticks.push(y);
           match state.get(){

               UiStates::AwaitingFirstMessage => {info!("We are in state: {:?}",state);next_state.set(UiStates::Setup)}
               _ =>{}
           }
           info!("state : {:?},message: {:?}",state, ticks);
       }else {
           if let Err(CommError::DeserializationError(_)) = x{
           }
           info!("{:?}",x);
       }
    }

    /*info!("No message to be found");*/

}
pub struct ServerPlugin;
impl Plugin for ServerPlugin{
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(UiServer::new())
            .insert_resource(Ticks::new())
            .add_systems(Update,retrieve_ticks.run_if(not(in_state(UiStates::MainMenu))));
    }
}