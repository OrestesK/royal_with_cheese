use cursive::views::{Dialog, TextView};

use futures::executor::block_on;
use royal_with_cheese::client::Client;
use royal_with_cheese::server::Server;
use std::env;
use std::io;

const ADDRESS: &str = "localhost";
const PORT: &str = "7878";

// unwraps panic on error

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "S" {
        server()
    } else {
        client()
    }
}

fn server() {
    let mut server: Server = block_on(Server::new(ADDRESS, PORT)).expect("Failed to create server");
    loop {
        block_on(Server::initiate(&mut server)).unwrap();
    }
}

fn client() {
    cursive();

    let mut client: Client =
        block_on(Client::new(ADDRESS, PORT)).expect("Couldn't connect to address");

    loop {
        let input = terminal_input();
        block_on(Client::send_data(&mut client, input)).expect("Failed to send data");
    }
}

// TEMPORARY FOR TESTING
fn terminal_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read");

    input
}

fn cursive() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );
    //siv.run();
}
