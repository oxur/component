use std::io::Write;
use std::net::TcpListener;
use std::thread;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new() -> Server {
        let listener = TcpListener::bind("0.0.0.0:1234").unwrap();
        println!("TCP listener set up to run on port 1234 ...");
        Server { listener: listener }
    }

    pub fn start(&self) {
        println!("Starting server component ...");
        for stream in self.listener.incoming() {
            thread::spawn(|| {
                println!("Connected ...");
                let mut stream = stream.unwrap();
                stream.write(b"Connected ...\r\n").unwrap();
            });
        }
    }

    // XXX Would be nice to find an example of how to really
    //     stop the thread ...
    pub fn stop(&self) {
        println!("Stopping server component ...");
    }
}

impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            listener: self.listener.try_clone().unwrap(),
        }
    }
}
