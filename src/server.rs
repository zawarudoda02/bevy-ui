use crate::states::{UiStates};
use bevy::prelude::*;
use std::collections::VecDeque;


use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use ui_and_robot_communication::{CommError, Server, Tick};


/// Resource containing the tcp server thread and a receiver
pub struct UiServer {
    pub receiver: Receiver<Result<Tick, CommError>>,
    pub end_sender: Sender<()>,
    thread: Option<JoinHandle<()>>,
}
impl UiServer {
    fn new() -> Self {
        let (tx, rx) = channel();
        let (end_sender, end_receiver) = channel::<()>();
        let th = thread::spawn(move || {
            let mut server = Server::new();
            server.begin_listening().expect("Client can't be accepted");
            loop {
                if let Ok(_) = end_receiver.try_recv(){
                    server.stop_listening();
                    return;
                }
                let a = server.get_world_info();
                match a {
                    Ok(x) => {
                        tx.send(Ok(x)).expect("Send between threads failed");
                        break;
                    }
                    Err(e) => tx.send(Err(e)).expect("Send between threads failed"),
                }
            }
            loop {
                if let Ok(_) = end_receiver.try_recv(){
                    server.stop_listening();
                    return;
                }
                let a = server.get_tick();

                match a {
                    Ok(x) => {
                        tx.send(Ok(x)).expect("Send between threads failed");
                    }
                    Err(e) => {
                        if let CommError::DeserializationError(_) = e {
                            break;
                        }
                        tx.send(Err(e));
                    }
                }
            }
        });
        Self {
            receiver: rx,
            end_sender,
            thread: Some(th),
        }
    }
}

///
/// A queue containing the ticks, gathered using "retrieve ticks"
#[derive(Debug, Resource)]
pub struct Ticks {
    ticks: VecDeque<Tick>,
}
impl Ticks {
    fn new() -> Self {
        Self {
            ticks: VecDeque::new(),
        }
    }
    pub fn push(&mut self, tick: Tick) {
        self.ticks.push_front(tick)
    }
    pub fn pop(&mut self) -> Option<Tick> {
        self.ticks.pop_back()
    }
}


///System that retrieves the ticks from the server and inserts them in the queue
fn retrieve_ticks(
    server: NonSend<UiServer>,
    mut ticks: ResMut<Ticks>,
    state: Res<State<UiStates>>,
    mut next_state: ResMut<NextState<UiStates>>,
) {
    if let Ok(x) = server.receiver.try_recv() {
        if let Ok(y) = x {
            ticks.push(y);
            match state.get() {
                UiStates::AwaitingFirstMessage => {
                    info!("We are in state: {:?}", state);
                    next_state.set(UiStates::Setup)
                }
                _ => {}
            }
        } else {
            info!("{:?}", x);
        }
    }

    /*info!("No message to be found");*/
}
fn close_connection(server: NonSend<UiServer>){
    server.end_sender.send(());
}
pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(UiServer::new())
            .insert_resource(Ticks::new())
            .add_systems(
                Update,
                retrieve_ticks.run_if(not(in_state(UiStates::MainMenu))),
            )
            .add_systems(OnEnter(UiStates::End),close_connection);

    }
}
