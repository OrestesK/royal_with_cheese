use crate::{
    dinput, network::shared::Shared, network::shared::FPS, network::shared_io::push_action,
};
use crossterm::event::{
    Event, EventStream, KeyCode,
    KeyCode::{Char, Esc},
    KeyEvent,
};
use futures::{future::FutureExt, select, StreamExt};
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn send_key_input(shared: Arc<Mutex<Shared>>, data: u8) {
    dinput!("{:?}", data);
    push_action(shared.clone(), 0, data);
}

fn key_delays(delay: &mut Instant, val: KeyEvent) -> Option<KeyCode> {
    let mut delay_value = 0.0;
    match val.code {
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => delay_value = 0.5,
        KeyCode::Char(_) => delay_value = 0.15,
        _ => {}
    }

    // TODO
    // need a delay Instant for each key that needs to be delayed
    // or just made player wait from last key press
    if delay.elapsed().as_secs_f64() < delay_value {
        // continue;
        return None;
    } else {
        *delay = Instant::now();
    }
    return Some(val.code);
}
async fn get_input(mut delay: &mut Instant) -> Option<KeyCode> {
    // reader
    let mut reader = EventStream::new();

    // loops because there might be keypresses queued up
    loop {
        // gets event from reader
        let mut event = reader.next().fuse();
        select! {
            maybe_event = event => {
                match maybe_event {
                    // Event available
                    Some(Ok(event)) => {
                        // Key Press
                        match event{
                            Event::Key(val) => return key_delays(&mut delay, val),
                            _ => {},
                        }
                    }
                    // Error
                    Some(Err(e)) => panic!("Error: {:?}\r", e),
                    // No event
                    None => break,
                }
            }
        };
    }
    None
}

pub async fn process_input(shared: Arc<Mutex<Shared>>) {
    let mut fps = fps_clock::FpsClock::new(FPS);
    let mut delay = &mut Instant::now();

    loop {
        match get_input(&mut delay).await {
            Some(Char(character)) => send_key_input(shared.clone(), character as u8),
            Some(KeyCode::Up) => send_key_input(shared.clone(), 1),
            Some(KeyCode::Down) => send_key_input(shared.clone(), 2),
            Some(KeyCode::Left) => send_key_input(shared.clone(), 3),
            Some(KeyCode::Right) => send_key_input(shared.clone(), 4),
            Some(Esc) => return,
            _ => {}
        }
        fps.tick();
    }
}
