use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
};

use futures::*;

pub struct MDServer {
    path: String,
    listener: TcpListener,
}

impl MDServer {
    pub fn new(path: String) -> Self {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        Self {
            path: path,
            listener: listener,
        }
    }

    pub async fn listen(&self, listener: &TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            futures::join!(self.handle_connection(stream));
        }
    }

    pub fn get_listener(&self) -> &self::TcpListener {
        &self.listener
    }

    async fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("{:#?}", http_request);

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(&self.path).unwrap();
        let length = contents.len();
        
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
        stream.write_all(response.as_bytes()).unwrap();
        println!("Responce has been sent");
    }
}

/*pub async fn bind_listener() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("Responce has been sent");
}*/