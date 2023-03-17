use super::shared::Shared;
use cursive::{
    theme::{BaseColor, ColorStyle},
    vec::Vec2,
    Printer,
};
use std::{
    io::Error,
    sync::{Arc, Mutex},
};

// SIZES BASED ON MY THINKPAD FULL SCREEN
// TOO MUCH WORK TO MATCH RESOLUTIONS
const BOARD_WIDTH: usize = 180;
const BOARD_HEIGHT: usize = 60;
const NUM_BOARDS: usize = 9;
const PLAYER_NUM: u8 = 100;
const EMPTY_CELL: &str = " ";

#[derive(Clone, Debug)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub color_style: ColorStyle,
    pub coordinate: Point,
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
    pub boards: Vec<Board>,
}

pub fn initiate_cells() -> Vec<Vec<Cell>> {
    let color_style = ColorStyle::new(BaseColor::White, BaseColor::White);
    let mut cells = Vec::<Vec<Cell>>::with_capacity(BOARD_HEIGHT);
    for i in 0u8..BOARD_HEIGHT as u8 {
        let mut cells_row = Vec::<Cell>::with_capacity(BOARD_WIDTH);
        for j in 0u8..BOARD_WIDTH as u8 {
            let cell = Cell {
                color_style,
                coordinate: Point { x: j, y: i },
            };
            cells_row.push(cell);
        }
        cells.push(cells_row);
    }
    cells
}

impl Board {
    pub fn new(shared: Arc<Mutex<Shared>>, board_number: u8) -> Result<Self, Error> {
        Ok(Board {
            shared,
            board_number,
        })
    }
}

impl MainBoard {
    pub fn new(shared: Arc<Mutex<Shared>>, total_boards: u8) -> Result<Self, Error> {
        let mut boards = Vec::<Board>::with_capacity(NUM_BOARDS as usize);
        for i in 0..NUM_BOARDS {
            let board = Board::new(shared.clone(), i as u8).expect("Failed to initialize board");
            boards.push(board);
        }

        Ok(MainBoard {
            board_width: BOARD_WIDTH as u8,
            board_height: BOARD_HEIGHT as u8,
            num_players: PLAYER_NUM,
            total_boards, // CHANGE
            boards,
        })
    }
}

impl cursive::view::View for MainBoard {
    fn draw(&self, printer: &Printer) {
        for i in 0 as usize..BOARD_HEIGHT {
            for j in 0 as usize..BOARD_WIDTH {
                let cell = self.boards[0].shared.lock().unwrap();
                let cell = &cell.map[i][j];

                printer.with_color(cell.color_style, |printer| {
                    printer.print(
                        (cell.coordinate.x as usize, cell.coordinate.y as usize),
                        EMPTY_CELL,
                    )
                })
            }
        }
    }
    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(BOARD_WIDTH, BOARD_HEIGHT)
    }
}
