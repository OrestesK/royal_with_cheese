use super::{
    board::{Board, MainBoard, BOARD_HEIGHT, BOARD_WIDTH, EMPTY_CELL, NUM_BOARDS, PLAYER_NUM},
    shared::Shared,
    shared_io,
};
use cursive::{
    theme::{BaseColor, ColorStyle},
    vec::Vec2,
    views::Panel,
    Printer,
};
use std::{
    io,
    sync::{Arc, Mutex},
};

// GUI
pub async fn cursive(shared: Arc<Mutex<Shared>>) {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let main_board = MainBoard::new(shared, NUM_BOARDS as u8);
    siv.add_fullscreen_layer(Panel::new(main_board));
    siv.set_autorefresh(true);
    siv.run();
}

// TEMPORARY FOR TESTING
pub fn terminal_input() -> Vec<u8> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.get(..1).unwrap().to_string().into_bytes()
}

impl MainBoard {
    pub fn new(shared: Arc<Mutex<Shared>>, total_boards: u8) -> Self {
        let mut boards = Vec::<Board>::with_capacity(NUM_BOARDS as usize);
        for i in 0..NUM_BOARDS {
            boards.push(Board::new(shared.clone(), i as u8));
        }

        let background_style =
            ColorStyle::new(BaseColor::White, BaseColor::light(BaseColor::White));
        let player_style = ColorStyle::new(BaseColor::White, BaseColor::light(BaseColor::Red));

        MainBoard {
            board_width: BOARD_WIDTH as u8,
            board_height: BOARD_HEIGHT as u8,
            num_players: PLAYER_NUM,
            total_boards,
            background_style,
            player_style,
            boards,
        }
    }
}

fn print_background(main_board: &MainBoard, printer: &Printer) {
    for i in 0 as usize..BOARD_WIDTH {
        for j in 0 as usize..BOARD_HEIGHT {
            printer.with_style(main_board.player_style, |printer| {
                printer.print((i, j), EMPTY_CELL)
            })
        }
    }
}

impl cursive::view::View for MainBoard {
    fn draw(&self, printer: &Printer) {
        //print_background(self, printer);

        let active_tiles = shared_io::get_server_active_tiles(self.boards[0].shared.clone());
        // eprintln!("HELLO {:?}", active_tiles.len());

        /*
        printer.with_color(self.player_style, |printer| {
            printer.print(
                (20 as usize, 20 as usize),
                &(cell.actions.len().to_string()),
            );
        });
        */

        for i in 0 as usize..active_tiles.len() {
            let cell = active_tiles.get(i).unwrap();

            printer.with_color(self.player_style, |printer| {
                printer.print(
                    (cell.coordinate.x as usize, cell.coordinate.y as usize),
                    EMPTY_CELL,
                )
            })
        }
    }
    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(BOARD_WIDTH, BOARD_HEIGHT)
    }
}
