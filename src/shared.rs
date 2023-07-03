use super::board::Cell;
use std::{collections::VecDeque, io::Error};

pub const FPS: u32 = 1; // GETS WONKY AT HIGH FPS

// creates debugging macro
#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => (if true { ::std::eprintln!($($arg)*); })
}
 
// struct Action
#[derive(Clone, Debug)]
pub struct Action {
    pub user: u8,
    pub code: u8,
}

// struct Shared
#[derive(Clone, Debug)]
pub struct Shared {
    pub active_tiles: Vec<Cell>,
    pub actions: VecDeque<Action>,
}

impl Shared {
    // creates Shared struct that will be used in Arc<Mutex<Shared>>
    // to be shared across threads (Arc) and across memory <Mutex>
    pub fn new() -> Result<Self, Error> {
        let active_tiles = Vec::<Cell>::with_capacity(100 * 100);

        let actions = VecDeque::with_capacity(400);

        Ok(Shared {
            active_tiles,
            actions,
        })
    }

    // returns active_tiles Vec<Cell>
    pub async fn get_active_tiles(self) -> Vec<Cell> {
        return self.active_tiles;
    }

    // returns actions VecDeque<Action>
    pub async fn get_actions(self) -> VecDeque<Action> {
        return self.actions;
    }
}
