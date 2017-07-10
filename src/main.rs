extern crate zmq;

mod command_server_socket;

use command_server_socket::CommandServerSocket;

fn main() {
    println!("Starting Block Chain Server...");

    let ctx = zmq::Context::new();

    let blockchain_socket = ctx.socket(zmq::PUB).unwrap();
    match blockchain_socket.bind("tcp://*:5665") {
        Ok(()) => println!("PUB Server started..."),
        Err(e) => panic!("Panicking while starting PUB server with {}", e),
    };

    let mut cli_socket = CommandServerSocket::new(ctx);
    cli_socket.bind_socket();

    loop {
        cli_socket.process_command();
    }
}
