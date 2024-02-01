use crate::board::Cell;
use std::{collections::VecDeque, io::Error};

pub const FPS: u32 = 60; // GETS WONKY AT HIGH FPS

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

fn new_wall(x: u8, y: u8) -> Cell {
    Cell {
        owner: 0,
        cell_type: 0,
        x,
        y,
    }
}
impl Shared {
    // creates Shared struct that will be used in Arc<Mutex<Shared>>
    // to be shared across threads <Arc> and across memory <Mutex>, enables locking
    pub fn new() -> Result<Self, Error> {
        let mut active_tiles = Vec::<Cell>::with_capacity(100 * 100);

        const WIDTH: u8 = 192;
        const HEIGHT: u8 = 45;
        const START_X: u8 = 11;

        for i in 0..WIDTH {
            //top wall
            active_tiles.push(new_wall(START_X + i, 3));
            //bottom wall
            active_tiles.push(new_wall(START_X + i, 49));
        }
        for i in 0..HEIGHT {
            // left wall
            active_tiles.push(new_wall(START_X, 4 + i));
            active_tiles.push(new_wall(START_X + 1, 4 + i));
            //right wall
            active_tiles.push(new_wall(START_X + WIDTH - 2, 4 + i));
            active_tiles.push(new_wall(START_X + WIDTH - 1, 4 + i));
        }

        // dfile!("{:?} {:?} ", active_tiles.capacity(), active_tiles.len());
        // TODO
        // 400 is random number I picked
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
