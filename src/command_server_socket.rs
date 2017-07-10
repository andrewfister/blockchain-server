extern crate zmq;

pub struct CommandServerSocket {
    socket: zmq::Socket,
}

impl CommandServerSocket {
    pub fn new(ctx: zmq::Context) -> CommandServerSocket {
        CommandServerSocket { socket: ctx.socket(zmq::REP).unwrap() }
    }

    pub fn bind_socket(&mut self) {
        match self.socket.bind("ipc://rep-server") {
            Ok(()) => println!("REP Server started..."),
            Err(e) => panic!("Panicking while starting REP server with {}", e),
        }
    }

    pub fn process_command(&self) {
        let mut command = zmq::Message::new().unwrap();

        self.socket.recv(&mut command, 0).unwrap();

        println!("Received a cli command!");

        let cli_command_text = command.as_str().unwrap();

        if cli_command_text == "status" {
            self.socket.send_str("OK", 0).unwrap();
        } else if cli_command_text == "create_user" {
            let user = user::User::new();
        } else {
            self.socket.send_str("Error", 0).unwrap();
        }
    }
}