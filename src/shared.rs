use std::collections::VecDeque;
use std::io::Error;
use std::sync::{Arc, Mutex};

pub struct Action {
    pub code: i8,
    pub user: i8,
}
pub struct Shared {
    //pub map: Vec<u8>,
    pub queue: Arc<Mutex<VecDeque<Action>>>,
}

impl Shared {
    pub fn new() -> Result<Self, Error> {
        //let map = Vec::new();
        let queue = Arc::new(Mutex::new(VecDeque::with_capacity(400)));
        Ok(Shared { queue })
    }
}
