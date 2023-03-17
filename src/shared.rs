use std::collections::VecDeque;
use std::io::Error;

// struct Action
pub struct Action {
    pub code: u8,
    pub user: u8,
}

// struct Shared
pub struct Shared {
    pub map: Vec<u8>,
    pub actions: VecDeque<Action>,
}

impl Shared {
    // creates Shared struct that will be used in Arc<Mutex<Shared>>
    // to be shared across threads (Arc) and across memory <Mutex>
    pub fn new() -> Result<Self, Error> {
        // 1000 MAP AREA
        let mut map = Vec::with_capacity(1000);
        map.push(255); // TEST VALUE

        // 100 players can make 4 moves at once?
        let actions = VecDeque::with_capacity(400);

        Ok(Shared { map, actions })
    }

    // returns map Vec<u8>
    pub async fn get_map(self) -> Vec<u8> {
        return self.map;
    }

    // returns actions VecDeque<Action>
    pub async fn get_actions(self) -> VecDeque<Action> {
        return self.actions;
    }
}
