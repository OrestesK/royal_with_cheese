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

impl Shared {
    // creates Shared struct that will be used in Arc<Mutex<Shared>>
    // to be shared across threads <Arc> and across memory <Mutex>, enables locking
    pub fn new() -> Result<Self, Error> {
        let mut active_tiles = Vec::<Cell>::with_capacity(100 * 100);

        for i in 0..60 {
            active_tiles.push(Cell {
                owner: 0,
                cell_type: 0,
                x: 9 + i,
                y: 3,
            });

            // active_tiles.push(Cell {
            //     owner: 0,
            //     cell_type: 0,
            //     x: 9 + i,
            //     y: 49,
            // });
        }

        // for i in 0..50 {
        //     active_tiles.push(Cell {
        //         owner: 0,
        //         cell_type: 0,
        //         x: 60,
        //         y: 10 + i,
        //     });
        // }

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
