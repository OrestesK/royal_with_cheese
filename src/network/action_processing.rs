use crate::{board::Cell, dfile, dinput, network::shared::Action};
use std::collections::VecDeque;

// pub fn test_process_actions(mut active_tiles: Vec<Cell>, actions: VecDeque<Action>) -> Vec<Cell> {
//     for action in actions {
//         dinput!("{:#?}", action.code);
//         let updated_cell = Cell {
//             owner: action.user,
//             cell_type: action.user,
//             x: action.code,
//             y: action.user * 10,
//         };
//
//         let mut new_tile: i8 = -1;
//         for (i, tile) in active_tiles.clone().iter_mut().enumerate() {
//             if tile.x == updated_cell.x && tile.y == updated_cell.y {
//                 new_tile = i as i8;
//                 break;
//             }
//         }
//
//         if new_tile == -1 {
//             active_tiles.push(updated_cell);
//         } else {
//             active_tiles.remove(new_tile as usize);
//         }
//
//         dinput!("Active: {:#?}", active_tiles);
//     }
//     active_tiles
// }

fn is_player(tile: &mut Cell, user: u8) -> bool {
    return tile.owner == user && tile.cell_type == 0;
}

// MY SCREEN BORDERS
// X | >=2 | <=210
// Y | >=1 | <=52
fn move_up(tile: &mut Cell) {
    if tile.y >= 1 {
        tile.y -= 1;
    }
}
fn move_down(tile: &mut Cell) {
    if tile.y <= 52 {
        tile.y += 1;
    }
}
fn move_right(tile: &mut Cell) {
    if tile.x <= 210 {
        tile.x += 2;
    }
}
fn move_left(tile: &mut Cell) {
    if tile.x >= 2 {
        tile.x -= 2;
    }
}

// 199 | w | move up
// 115 | s | move down
// 97 | a | move left
// 100 | d | move right
//
// 1 | up key
fn player_move(active_tiles: &mut Vec<Cell>, action: u8, user: u8) {
    let move_function: &dyn Fn(&mut Cell);
    match action {
        119 => move_function = &move_up,
        115 => move_function = &move_down,
        97 => move_function = &move_left,
        100 => move_function = &move_right,
        _ => panic!("impossible error"),
    }
    for tile in active_tiles {
        if is_player(tile, user) {
            move_function(tile);
        }
    }
}

// TODO
// determine where shot spawns
fn player_shoot(active_tiles: &mut Vec<Cell>, action: u8, user: u8) {
    let user_x: u8;
    let user_y: u8;
    for tile in active_tiles {
        if is_player(tile, user) {}
    }

    let shot = Cell {
        owner: user,
        cell_type: action,
        x: user.x,
        y: 0,
    };
    active_tiles.push(shot);
}

// TODO
// process shot
fn game_loop(active_tiles: &mut Vec<Cell>) {
    for tile in active_tiles {
        match tile.cell_type {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            _ => {}
        }
    }
}

pub fn process_actions(mut active_tiles: Vec<Cell>, actions: VecDeque<Action>) -> Vec<Cell> {
    game_loop(&mut active_tiles);
    for action in actions {
        dinput!("{:#?}", action.code);
        match action.code {
            119 | 115 | 97 | 100 => {
                player_move(&mut active_tiles, action.code, action.user);
            }
            1 | 2 | 3 | 4 => player_shoot(&mut active_tiles, action.code, action.user),
            _ => {
                dfile!("{:?}", action.code)
            }
        }
    }
    active_tiles
}
