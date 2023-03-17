use cursive::views::{Dialog, TextView};

use fps_clock;
use futures::executor::block_on;
use royal_with_cheese::client::Client;
use royal_with_cheese::server::Server;
use royal_with_cheese::shared::Shared;
use std::env;
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

const ADDRESS: &str = "localhost";
const PORT: &str = "7878";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "S" {
        server()
    } else {
        client()
    }
}

async fn testing(shared: Arc<Mutex<Shared>>) {
    eprintln!("IN TESTING");
    let shared = Arc::clone(&shared);
    let mut tt_shared = shared.lock().await;
    let map = &mut tt_shared.map;
    eprintln!("{:?}", *map);
    map.push(2);
}

// server side
fn server() {
    // builds server connection to socket
    let server: Server = block_on(Server::new(ADDRESS, PORT)).expect("Failed to create server");

    // initiates reading and writing from clients
    let shared = Shared::new().expect("Failed to initialize Shared");
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    let temp_shared = Arc::clone(&shared);
    tokio::spawn(Server::initiate(server, temp_shared));

    // loop so program does not end
    let mut i: u128 = 0;
    let mut fps = fps_clock::FpsClock::new(1);
    loop {
        //eprintln!("{:?}", i);
        if i == 500000000 {
            //let tt_shared = shared.clone();
            //tokio::spawn(testing(tt_shared));
            i = 99999999;
        }
        i += 1;
        fps.tick();
    }
}

// client side
fn client() {
    //cursive();

    // builds client connection to server
    let client: Client =
        block_on(Client::new(ADDRESS, PORT)).expect("Failed to connect to address");

    // initiates reading from server and returns a write connection
    let mut write_to_server_connection =
        block_on(Client::initiate(client)).expect("Failed to initialize client");

    // main loop
    loop {
        let input = terminal_input();
        block_on(Client::write_data_to_server(
            &mut write_to_server_connection,
            input,
        ))
        .expect("Failed to send data to server");
    }
}

// TEMPORARY FOR TESTING
fn terminal_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read");

    input
}

// GUI
fn cursive() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );
    //siv.run();
}
