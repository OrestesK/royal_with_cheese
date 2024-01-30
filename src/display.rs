use crate::{
    board::Cell, dfile, input::process_input, network::shared::Shared, network::shared::FPS,
    network::shared_io::get_server_active_tiles,
};
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    style::{self, Color, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use futures::future::FutureExt;
use std::{
    io::{stdout, Stdout, Write},
    sync::{Arc, Mutex},
    task::Poll,
};

const COLORS: [Color; 21] = [
    Color::Black,
    Color::DarkGrey,
    Color::Grey,
    Color::White,
    Color::DarkRed,
    Color::Red,
    Color::DarkGreen,
    Color::Green,
    Color::DarkYellow,
    Color::Yellow,
    Color::DarkBlue,
    Color::Blue,
    Color::DarkMagenta,
    Color::Magenta,
    Color::DarkCyan,
    Color::Cyan,
    Color::AnsiValue(0),
    Color::AnsiValue(15),
    Color::Rgb { r: 255, g: 0, b: 0 },
    Color::Rgb { r: 0, g: 255, b: 0 },
    Color::Rgb { r: 0, g: 0, b: 255 },
];

pub fn print(stdout: &mut Stdout, tile: Cell) -> Result<()> {
    let color = COLORS[tile.owner as usize + 3];
    stdout
        .queue(SetForegroundColor(color))?
        .queue(cursor::MoveTo((tile.x) as u16, (tile.y) as u16))?
        .queue(style::Print("█"))?;

    if tile.owner == 1 {
        stdout
            .queue(SetForegroundColor(color))?
            .queue(cursor::MoveTo(50, 10))?
            .queue(style::Print(tile.x))?
            .queue(cursor::MoveTo(50, 11))?
            .queue(style::Print(tile.y))?;
    }

    Ok(())
}
pub async fn display(shared: Arc<Mutex<Shared>>, is_client: bool) -> Result<()> {
    // if !is_client {
    //     return Ok(());
    // }
    // } else {
    //     return Ok(());
    // }

    let mut stdout = stdout();
    stdout = init(stdout)?;

    let mut input = tokio::spawn(process_input(shared.clone())).fuse();

    let mut fps = fps_clock::FpsClock::new(FPS);

    // while not pressed 'Esc'
    loop {
        // following render calls will keep rendering the last rendered state.
        stdout.execute(terminal::BeginSynchronizedUpdate)?;

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        let active_tiles = get_server_active_tiles(shared.clone());
        dfile!(
            "{:?} {:?} {:?} \n",
            is_client,
            active_tiles.capacity(),
            active_tiles.len()
        );

        for tile in active_tiles {
            print(&mut stdout, tile)?;
        }
        stdout.flush()?;

        stdout.execute(terminal::EndSynchronizedUpdate)?;

        // if input returns (designed to do so when Esc), end program
        match futures::poll!(&mut input) {
            Poll::Pending => {}
            Poll::Ready(_) => break,
        }

        fps.tick();
    }

    deinit(stdout)?;
    Ok(())
}

pub fn init(mut stdout: Stdout) -> Result<Stdout> {
    terminal::enable_raw_mode()?;

    stdout.queue(DisableMouseCapture)?;
    stdout.queue(cursor::Hide)?;

    stdout.queue(terminal::Clear(terminal::ClearType::Purge))?;

    stdout.queue(SetForegroundColor(Color::Blue))?; // block color

    stdout.flush()?;

    Ok(stdout)
}

pub fn deinit(mut stdout: Stdout) -> Result<Stdout> {
    terminal::disable_raw_mode()?;

    stdout.queue(EnableMouseCapture)?;
    stdout.queue(cursor::Hide)?;

    stdout.queue(terminal::Clear(terminal::ClearType::Purge))?;

    stdout.queue(terminal::LeaveAlternateScreen)?;

    stdout.flush()?;

    Ok(stdout)
}
