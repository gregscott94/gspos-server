extern crate ws;
extern crate nickel;

use std::thread;
use std::time;
use std::env;

use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};
use nickel::{Nickel, Mountable, StaticFilesHandler};

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
  let mut prod_env = "".to_string();

  let ws_server_thread = thread::Builder::new().name("ws_server".to_string()).spawn(move || {
      println!("Starting websocket server..");
      listen("127.0.0.1:3012", |out| { Server { out: out } }).unwrap()
  }).unwrap();

  thread::sleep(time::Duration::from_millis(1000));
  match env::var("PROD_ENV".to_string()) {
      Ok(val) => prod_env = val,
      Err(e) => println!("Operating in dev mode due to: {}", e),
  }

  if prod_env != "" {
    let app_router_thread = thread::Builder::new().name("app_router".to_string()).spawn(move || {
        let mut app_router = Nickel::new();
        println!("Starting app router..");
        app_router.mount("/controller/", StaticFilesHandler::new("app/controller/"));
        app_router.mount("/display/", StaticFilesHandler::new("app/display/"));
        app_router.listen("127.0.0.1:6767").unwrap();
    }).unwrap();
    let _ = app_router_thread.join();
  }

  let _ = ws_server_thread.join();
  println!("Server closing down..");
}
