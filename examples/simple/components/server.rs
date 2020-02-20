use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new() -> Server {
        let listener = TcpListener::bind("0.0.0.0:12345").unwrap();
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");
        println!("TCP listener set up to run on port 1234 ...");
        Server { listener: listener }
    }

    pub fn start(&self) {
        println!("Starting server component ...");
        for stream in self.listener.incoming() {
            match stream {
                Ok(s) => {
                    self.handle_connection(s);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Decide if we should exit
                    println!("Error: {:?}", e);
                    thread::sleep(time::Duration::from_millis(5000));
                    break;
                    // Decide if we should try to accept a connection again
                    // continue;
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }

    pub fn stop(&self) {
        println!("Stopping server component ...")
        // self.listener.
    }

    pub fn handle_connection(&self, mut s: TcpStream) {
        println!("Connected ...");
        s.write(b"Connected ...\r\n").unwrap();
        // thread::spawn(|| {
        //     let mut s = s;
        //     s.write(b"Connected ...\r\n").unwrap();
        // });
    }
}

impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            listener: self.listener.try_clone().unwrap(),
        }
    }
}
