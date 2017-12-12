extern crate ws;

use std::thread;

use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

struct Server {
    out: Sender,
}

impl Handler for Server {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        Ok(println!("The client has connected"))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message to all connected clients
        self.out.broadcast(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

}

fn main() {
  let ws_server = thread::Builder::new().name("ws_server".to_string()).spawn(move || {
      listen("127.0.0.1:3012", |out| { Server { out: out } }).unwrap()
  }).unwrap();
  let app_router = thread::Builder::new().name("app_router".to_string()).spawn(move || {
      println!("Router")
  }).unwrap();
  let _ = ws_server.join();
  let _ = app_router.join();
  println!("Server closing down..");
}
