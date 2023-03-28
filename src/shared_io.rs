use cursive::Vec2;

use super::{
    board::Cell,
    shared::{Action, Shared},
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub fn get_server_active_tiles(shared: Arc<Mutex<Shared>>) -> Vec<Cell> {
    let guard = shared.lock().unwrap();
    let data = guard.active_tiles.clone();
    drop(guard);
    data
}

pub fn get_and_clear_server_actions(shared: Arc<Mutex<Shared>>) -> VecDeque<Action> {
    let mut guard = shared.lock().unwrap();
    let data = guard.actions.clone();
    guard.actions.clear();
    drop(guard);
    data
}

pub async fn update_active_tiles(shared: Arc<Mutex<Shared>>) {
    let mut actions = get_and_clear_server_actions(shared.clone());
    let mut active_tiles = get_server_active_tiles(shared.clone());

    active_tiles.push(Cell {
        cell_type: 1,
        coordinate: Vec2 {
            x: actions.pop_front().unwrap().code as usize - 60,
            y: 20,
        },
    });

    let mut final_actions = shared.lock().unwrap();
    final_actions.active_tiles = active_tiles;
    drop(final_actions);
}

pub fn add_action(shared: Arc<Mutex<Shared>>, action: Action) {
    let mut shared = shared.lock().unwrap();
    shared.actions.push_back(action);
    drop(shared);
}

pub fn active_tiles_to_data(shared: Arc<Mutex<Shared>>) -> Vec<u8> {
    let active_tiles = get_server_active_tiles(shared.clone());
    let mut data_to_send = Vec::with_capacity(active_tiles.len() * 3);
    for tile in active_tiles.iter() {
        data_to_send.push(tile.cell_type);
        data_to_send.push(tile.coordinate.x as u8);
        data_to_send.push(tile.coordinate.y as u8);
    }
    data_to_send
}

pub async fn data_to_active_tiles(shared: Arc<Mutex<Shared>>, data: Vec<u8>) {
    let mut active_tiles = Vec::<Cell>::with_capacity(data.len() / 3);
    let mut index = 0;
    while index < data.len() {
        active_tiles.push(Cell {
            cell_type: *data.get(index).unwrap(),
            coordinate: Vec2 {
                x: *data.get(index + 1).unwrap() as usize,
                y: *data.get(index + 2).unwrap() as usize,
            },
        });
        index += 3;
    }

    let mut shared_active_tiles = shared.lock().unwrap();
    shared_active_tiles.active_tiles = active_tiles;
}
