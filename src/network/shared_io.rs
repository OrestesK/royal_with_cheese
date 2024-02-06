use crate::{
    board::Cell,
    network::action_processing,
    network::game_processing,
    network::shared::{Action, Shared},
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Instant,
};

// gets and clears queued actions
pub fn get_and_clear_actions(shared: Arc<Mutex<Shared>>) -> VecDeque<Action> {
    let mut shared_guard = shared.lock().unwrap();
    let data = shared_guard.actions.clone();
    shared_guard.actions.clear();

    drop(shared_guard);
    data
}

// pushes an action to the queue
pub fn push_action(shared: Arc<Mutex<Shared>>, user: u8, code: u8) {
    let action = Action { user, code };

    let mut shared_guard = shared.lock().unwrap();
    shared_guard.actions.push_back(action);
    drop(shared_guard);
}

// gets the current active tiles
pub fn get_server_active_tiles(shared: Arc<Mutex<Shared>>) -> Vec<Cell> {
    let shared_guard = shared.lock().unwrap();
    let data = shared_guard.active_tiles.clone();

    drop(shared_guard);
    data
}

// replaces active tiles with the given updated value
pub fn update_active_tiles(shared: Arc<Mutex<Shared>>, active_tiles: Vec<Cell>) {
    let mut shared_guard = shared.lock().unwrap();
    shared_guard.active_tiles = active_tiles;
    drop(shared_guard)
}

// serializes active tiles to data
pub fn active_tiles_to_data(shared: Arc<Mutex<Shared>>) -> Vec<u8> {
    let active_tiles = get_server_active_tiles(shared.clone());

    let mut data_to_send = Vec::with_capacity(active_tiles.len() * 4);
    for tile in active_tiles {
        data_to_send.push(tile.owner);
        data_to_send.push(tile.cell_type);
        data_to_send.push(tile.x);
        data_to_send.push(tile.y);
    }
    data_to_send
}
// deserializes data into active tiles
pub async fn data_to_active_tiles(shared: Arc<Mutex<Shared>>, data: Vec<u8>) {
    let mut active_tiles = Vec::<Cell>::with_capacity(data.len() / 4);

    let mut index = 0;
    while index < data.len() {
        active_tiles.push(Cell {
            owner: data[index],
            cell_type: data[index + 1],
            x: data[index + 2],
            y: data[index + 3],
        });
        index += 4;
    }

    update_active_tiles(shared.clone(), active_tiles)
}

pub async fn add_tile(shared: Arc<Mutex<Shared>>, owner: u8, cell_type: u8, x: u8, y: u8) {
    let mut active_tiles = get_server_active_tiles(shared.clone());
    active_tiles.push(Cell {
        owner,
        cell_type,
        x,
        y,
    });
    update_active_tiles(shared.clone(), active_tiles);
}

// processes actions
pub async fn process_actions(shared: Arc<Mutex<Shared>>) {
    let actions = get_and_clear_actions(shared.clone());
    if actions.len() == 0 {
        return;
    }

    let active_tiles = get_server_active_tiles(shared.clone());
    let updated_tiles = action_processing::process_actions(active_tiles, actions);

    update_active_tiles(shared.clone(), updated_tiles);
}

// processes game
pub async fn process_game(shared: Arc<Mutex<Shared>>, delay: &mut Instant) {
    //TODO
    // maybe in the future block others from using active tiles while processing the game loop
    if delay.elapsed().as_secs_f64() < 0.1 {
        return;
    }

    let active_tiles = get_server_active_tiles(shared.clone());
    let updated_tiles = game_processing::process_game(active_tiles);
    update_active_tiles(shared.clone(), updated_tiles);

    *delay = Instant::now();
}
