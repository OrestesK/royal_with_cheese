use fps_clock;
use futures::executor::block_on;
use royal_with_cheese::{client::Client, display, server::Server, shared::Shared};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

const ADDRESS: &str = "0.0.0.0"; //"localhost";
const PORT: &str = "8080";

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
    //eprintln!("Stored Moves:");
    let shared = Arc::clone(&shared);
    let tt_shared = shared.lock().await;
    let len = tt_shared.actions.len();
    for index in 0..len {
        let action = tt_shared.actions.get(index).unwrap();
        eprintln!("User: {:?} || Move: {:?}", action.user, action.code);
    }
}

// server side
fn server() {
    // builds server connection to socket
    let server: Server = block_on(Server::new(ADDRESS, PORT)).expect("Failed to create server");

    // initiates reading and writing from clients
    let shared = Shared::new().expect("Failed to initialize Shared");
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    let temp_shared_server = Arc::clone(&shared);
    let temp_shared_gui = Arc::clone(&shared);
    tokio::spawn(Server::initiate(server, temp_shared_server));

    tokio::spawn(display::cursive(tmp_shared_gui));

    // loop so program does not end
    let mut fps = fps_clock::FpsClock::new(1);
    loop {
        let tt_shared = shared.clone();
        tokio::spawn(testing(tt_shared));
        fps.tick();
    }
}

// client side
fn client() {
    // builds client connection to server
    let client: Client =
        block_on(Client::new(ADDRESS, PORT)).expect("Failed to connect to address");

    // initiates reading from server and returns a write connection
    let mut write_to_server_connection =
        block_on(Client::initiate(client)).expect("Failed to initialize client");

    // main loop
    loop {
        let input = display::terminal_input();
        block_on(Client::write_data_to_server(
            &mut write_to_server_connection,
            input,
        ))
        .expect("Failed to send data to server");
    }
    //cursive();
}
