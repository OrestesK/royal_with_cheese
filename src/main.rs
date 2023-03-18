use fps_clock;
use futures::executor::block_on;
use royal_with_cheese::{client_network::Client, display, server_network::Server, shared::Shared};
use std::{
    env,
    sync::{Arc, Mutex},
};

const ADDRESS: &str = "0.0.0.0";
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
    eprintln!("Stored Moves:");
    let shared = Arc::clone(&shared);
    let tt_shared = shared.lock().unwrap();
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

    // creates shared 'Shared' data
    let shared = Shared::new().unwrap();
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    // initializes reading and writing from clients (loop)
    tokio::spawn(Server::initialize_server(server, shared.clone()));

    // initializes GUI
    tokio::spawn(display::cursive(shared.clone()));

    // loop so program does not end
    let mut fps = fps_clock::FpsClock::new(1);
    loop {
        //let tt_shared = shared.clone();
        //tokio::spawn(testing(tt_shared));
        fps.tick();
    }
}

// client side
fn client() {
    // builds client connection to server
    let client: Client =
        block_on(Client::new(ADDRESS, PORT)).expect("Failed to connect to address");

    // creates shared 'Shared' data
    let shared = Shared::new().unwrap();
    let shared = Arc::new(Mutex::new(shared)); //creates shared 'Shared' Struct

    let (read, mut write) = client.connection.into_split();

    // initializes reading data from server (loop)
    tokio::spawn(Client::read_data_from_server(shared.clone(), read));

    // main loop
    loop {
        let input = display::terminal_input();
        block_on(Client::write_data_to_server(&mut write, input));
    }
    //cursive();
}
