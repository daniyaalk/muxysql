use crate::mysql::command::{Command};

pub struct Connection {  
    pub state: State,
    pub partial_data: Option<Vec<u8>>,
    pub last_command: Option<Box<Command>>
}

impl Connection {
    pub fn new() -> Connection {
        Connection{
            state: State::Initiated,
            partial_data: None,
            last_command: None
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn mark_auth_done(&mut self) {
        self.state = State::AuthDone
    }

    pub fn unset_partial_data(&mut self) {
        self.partial_data = None;
    }

    pub fn set_partial_data(&mut self, bytes: &[u8]) {
        let mut temp: Vec<u8> = Vec::new();
        temp.extend_from_slice(bytes);
        self.partial_data = Some(temp);
    }
}



pub enum State {
    Initiated,
    AuthDone,
    PendingResponse
}

#[derive(Debug)]
pub enum Direction {
    C2S, S2C
}