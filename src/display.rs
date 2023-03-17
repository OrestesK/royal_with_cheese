use super::board::MainBoard;
use super::shared::Shared;
use cursive::views::Panel;
use std::io;
use std::{io::Error, sync::Arc};
use tokio::sync::Mutex;

// GUI
pub async fn cursive(shared: Vec<Arc<Mutex<Shared>>>) {
    let mut siv = cursive::ncurses();
    siv.add_global_callback('q', |s| s.quit());

    let main_board = MainBoard::new(shared, 9).expect("Failed to create board");
    // siv.add_fullscreen_layer(Panel::new(board));
    siv.add_fullscreen_layer(Panel::new(main_board));
    siv.run();
}

// TEMPORARY FOR TESTING
pub fn terminal_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read");

    input
}
