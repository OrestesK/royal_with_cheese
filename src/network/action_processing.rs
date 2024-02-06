use crate::{board::Cell, dfile, dinput, network::shared::Action};
use std::cmp::min;
use std::collections::VecDeque;

// returns Cell at (x ,y)
fn find_tile(active_tiles: &mut Vec<Cell>, x: u8, y: u8) -> Option<&mut Cell> {
    let mut cell: Option<&mut Cell> = None;
    for tile in active_tiles {
        if tile.x == x && tile.y == y {
            cell = Some(tile);
        }
    }
    cell
}
// returns if tile is a player
fn is_player(tile: &mut Cell, user: u8) -> bool {
    return tile.owner == user && tile.cell_type == 0;
}
// returns top tile of the player
fn find_player(active_tiles: &mut Vec<Cell>, user: u8) -> Option<(u8, u8)> {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut found = true;
    for tile in active_tiles {
        if is_player(tile, user) {
            if found {
                x = tile.x;
                y = tile.y;
                found = false;
            } else {
                y = min(y, tile.y);
                break;
            }
        }
    }
    if found {
        return None;
    }
    Some((x, y))
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

fn player_shoot(active_tiles: &mut Vec<Cell>, action: u8, user: u8) {
    let bullet_x;
    let bullet_y;

    let player_location = find_player(active_tiles, user).expect("PLAYER SHOT WITHOUT EXISTING");
    bullet_x = player_location.0;

    match action {
        1 | 3 | 4 => bullet_y = player_location.1,
        2 => bullet_y = player_location.1 + 1,
        _ => panic!("impossible error"),
    }

    let shot = Cell {
        owner: user,
        cell_type: action,
        x: bullet_x,
        y: bullet_y,
    };
    active_tiles.push(shot);
}

pub fn process_actions(mut active_tiles: Vec<Cell>, actions: VecDeque<Action>) -> Vec<Cell> {
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
