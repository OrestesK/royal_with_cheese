use super::{
    board::{Board, MainBoard, BOARD_HEIGHT, BOARD_WIDTH, EMPTY_CELL, NUM_BOARDS, PLAYER_NUM},
    shared::Shared,
    shared_io,
    dprint,
};
use cursive::{event::Event, event::EventResult, event::Key};
use cursive::{
    theme::{BaseColor, ColorStyle},
    vec::Vec2,
    views::Panel,
    Printer,
};
use std::{
    //io,
    sync::{Arc, Mutex},
};

fn send_key_input(shared: Arc<Mutex<Shared>>, data: u8) {
    shared_io::push_action(shared.clone(), 0, data);
}

// GUI
pub fn cursive(shared: Arc<Mutex<Shared>>, is_client: bool) {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let main_board = MainBoard::new(shared, NUM_BOARDS as u8, is_client);

    if false{
        siv.add_fullscreen_layer(Panel::new(main_board));
        siv.set_autorefresh(true);
        siv.set_fps(10);
        siv.run();
    }
    else{
        if is_client{
            testing(main_board);
     }

    }
}

impl MainBoard {
    pub fn new(shared: Arc<Mutex<Shared>>, total_boards: u8, is_client: bool) -> Self {
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
            is_client,
        }
    }
}

fn testing(main_board: MainBoard){
    let mut fps = fps_clock::FpsClock::new(1);
    send_key_input(main_board.boards.get(0).unwrap().shared.clone(), 100);
    loop {
        let active_tiles = shared_io::get_server_active_tiles(main_board.boards[0].shared.clone());
        dprint!("Testing: {:?}", active_tiles);
        send_key_input(main_board.boards.get(0).unwrap().shared.clone(), 100);

        fps.tick();
    }

}


impl cursive::view::View for MainBoard {
    fn draw(&self, printer: &Printer) {
        let active_tiles = shared_io::get_server_active_tiles(self.boards[0].shared.clone());

        for cell in active_tiles{

            let mut color = self.player_style;
            if cell.cell_type == 2{
                color = self.background_style;
            }


            printer.with_color(color, |printer| {
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

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(input) => {
                send_key_input(self.boards.get(0).unwrap().shared.clone(), input as u8)
            }

            Event::Key(Key::Enter) => {
                send_key_input(self.boards.get(0).unwrap().shared.clone(), 100)
            }

            _ => return EventResult::Ignored,
        }
        EventResult::Ignored
    }
}
