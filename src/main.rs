mod server;
mod client;
use std::thread;

use crate::{client::client, server::server};

fn main() {
    let client_thread = thread::spawn(move || {
        client();
    });

    let server_thread = thread::spawn(move || {
        server();
    });

    client_thread.join().expect_err("Error");
    server_thread.join().expect_err("Error");

}
