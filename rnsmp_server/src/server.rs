use core::panic;
use std::net::{ TcpListener, TcpStream };
use std::io::Error;
use std::io;

pub struct Server {

    pub tick: u32,

    listener: TcpListener,
    streams: Vec<TcpStream>

}

impl Server {

    pub fn new(port: &str) -> Result<Server, Error> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        listener.set_nonblocking(true)?;

        let server = Self {
            tick: 0,

            listener: listener,
            streams: Vec::new()
        };

        return Ok(server);
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        self.accept_connections()?;
        
        self.tick += 1;
        return Ok(());
    }

    fn accept_connections(&mut self) -> Result<(), Error> {
        for incoming_stream in self.listener.incoming() {
            match incoming_stream {
                Ok(is) => {
                    self.streams.push(is);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }

        return Ok(());
    }

}