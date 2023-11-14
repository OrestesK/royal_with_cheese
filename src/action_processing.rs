use super::{board::Cell, dclient, shared::Action};
use std::collections::VecDeque;

pub fn test_process_actions(mut active_tiles: Vec<Cell>, actions: VecDeque<Action>) -> Vec<Cell> {
    for action in actions {
        // eprintln!("{:#?}", action.code);
        let updated_cell = Cell {
            owner: action.user,
            cell_type: action.user,
            x: action.code,
            y: action.user * 10,
        };

        let mut new_tile: i8 = -1;
        for (i, tile) in active_tiles.clone().iter_mut().enumerate() {
            if tile.x == updated_cell.x && tile.y == updated_cell.y {
                new_tile = i as i8;
                break;
            }
        }

        if new_tile == -1 {
            active_tiles.push(updated_cell);
        } else {
            active_tiles.remove(new_tile as usize);
        }

        dclient!("Active: {:#?}", active_tiles);
    }
    active_tiles
}

fn player_move_up(active_tiles: &mut Vec<Cell>, user: u8) {
    for tile in active_tiles {
        if tile.owner == user {
            tile.y -= 1;
        }
    }
}
fn player_move_down(active_tiles: &mut Vec<Cell>, user: u8) {
    for tile in active_tiles {
        if tile.owner == user {
            tile.y += 1;
        }
    }
}
fn player_move_left(active_tiles: &mut Vec<Cell>, user: u8) {
    for tile in active_tiles {
        if tile.owner == user {
            tile.x -= 1;
        }
    }
}
fn player_move_right(active_tiles: &mut Vec<Cell>, user: u8) {
    for tile in active_tiles {
        if tile.owner == user {
            tile.x += 1;
        }
    }
}

pub fn process_actions(mut active_tiles: Vec<Cell>, actions: VecDeque<Action>) -> Vec<Cell> {
    for action in actions {
        match action.code {
            119 => {
                //up
                player_move_up(&mut active_tiles, action.user);
            }

            115 => {
                //down
                player_move_down(&mut active_tiles, action.user);
            }

            97 => {
                //left
                player_move_left(&mut active_tiles, action.user);
            }

            100 => {
                //right
                player_move_right(&mut active_tiles, action.user);
            }
            _ => {}
        }
    }
    active_tiles
}
