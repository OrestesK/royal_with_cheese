use cursive::Vec2;

use super::{
    board::Cell,
    shared::{
        Action, 
        Shared
    },
    dprint,
};
use std::{
    collections::VecDeque,
    sync::{
        Arc, 
        Mutex
    },
};

// gets the current active tiles
pub fn get_server_active_tiles(shared: Arc<Mutex<Shared>>) -> Vec<Cell> {
    let shared_guard = shared.lock().unwrap();
    let data = shared_guard.active_tiles.clone();

    drop(shared_guard);
    data
}

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

// replaces active tiles with the given updated value
pub fn update_active_tiles(shared: Arc<Mutex<Shared>>, active_tiles: Vec<Cell>){
    let mut shared_guard = shared.lock().unwrap();
    shared_guard.active_tiles = active_tiles;
    drop(shared_guard)
}

pub async fn process_actions(shared: Arc<Mutex<Shared>>) {
    let mut actions = get_and_clear_actions(shared.clone());
    if actions.len() == 0 {
        return;

    }

    //TODO FIX, RIGHT NOW ONLY GETS THE FRONT ACTION

    let mut active_tiles = get_server_active_tiles(shared.clone());

    let action = actions.pop_front().unwrap();
    let test_cell = Cell {
        cell_type: action.user,
        coordinate: Vec2 {
            x: (action.code - 60) as usize,
            y: 20,
        },
    };


    let mut new_tile: i8 = -1;
    for (i, tile) in active_tiles.clone().iter_mut().enumerate(){
        if tile.coordinate == test_cell.coordinate{
            new_tile = i as i8;
            break;
        } 
    }

    if new_tile == -1{
        active_tiles.push(test_cell);

    }
    else{
        active_tiles.remove(new_tile as usize);
    }

    dprint!("Active: {:#?}", active_tiles);

    update_active_tiles(shared.clone(), active_tiles)
}

pub fn active_tiles_to_data(shared: Arc<Mutex<Shared>>) -> Vec<u8> {
    let active_tiles = get_server_active_tiles(shared.clone());

    let mut data_to_send = Vec::with_capacity(active_tiles.len() * 3);
    for tile in active_tiles {
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
            cell_type: data[index],
            coordinate: Vec2 {
                x: data[index + 1] as usize,
                y: data[index + 2] as usize,
            },
        });
        index += 3;
    }

    update_active_tiles(shared.clone(), active_tiles)
}
