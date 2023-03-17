use crate::board;

use super::board::Cell;
use std::{collections::VecDeque, io::Error};

// struct Action
#[derive(Clone, Debug)]
pub struct Action {
    pub user: u8,
    pub code: u8,
}

// struct Shared
#[derive(Clone, Debug)]
pub struct Shared {
    pub map: Vec<Vec<Cell>>,
    pub actions: VecDeque<Action>,
}

impl Shared {
    // creates Shared struct that will be used in Arc<Mutex<Shared>>
    // to be shared across threads (Arc) and across memory <Mutex>
    pub fn new() -> Result<Self, Error> {
        let map = board::initiate_cells();

        let actions = VecDeque::with_capacity(400);

        Ok(Shared { map, actions })
    }

    // returns map Vec<Vec<Cell>>
    pub async fn get_map(self) -> Vec<Vec<Cell>> {
        return self.map;
    }

    // returns actions VecDeque<Action>
    pub async fn get_actions(self) -> VecDeque<Action> {
        return self.actions;
    }
}
