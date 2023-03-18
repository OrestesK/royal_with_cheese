use super::shared::Shared;
use cursive::{theme::ColorStyle, Vec2};
use std::sync::{Arc, Mutex};

pub const BOARD_WIDTH: usize = 180;
pub const BOARD_HEIGHT: usize = 60;
pub const NUM_BOARDS: usize = 9;
pub const PLAYER_NUM: u8 = 100;
pub const EMPTY_CELL: &str = " ";
// SIZES BASED ON MY THINKPAD FULL SCREEN
// TOO MUCH WORK TO MATCH RESOLUTIONS

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_type: u8,
    pub coordinate: Vec2,
}

pub struct Board {
    pub shared: Arc<Mutex<Shared>>,
    pub board_number: u8,
}

pub struct MainBoard {
    pub board_width: u8,
    pub board_height: u8,
    pub num_players: u8,
    pub total_boards: u8,
    pub background_style: ColorStyle,
    pub player_style: ColorStyle,
    pub boards: Vec<Board>,
}

impl Board {
    pub fn new(shared: Arc<Mutex<Shared>>, board_number: u8) -> Self {
        Board {
            shared,
            board_number,
        }
    }
}
