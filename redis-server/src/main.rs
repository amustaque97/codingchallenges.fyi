use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};

use parser::Value;

mod parser;

/// Basic setup on how to handle the connections and reply accordingly
fn handle_connection(stream: TcpStream) {
    let mut reader = io::BufReader::new(stream.try_clone().expect("fail to clone tcpstream..."));
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());

    // println!("{}", String::from_utf8_lossy(&received[0..received.len()]));
    let command_str = String::from_utf8_lossy(&received[0..received.len()]).to_string();
    let mut parser = parser::Parser::new(command_str);
    let value: Value = parser.parse();

    let command = value.array[0].value.clone().unwrap_or("".to_string());

    match command.as_str() {
        "PING" => {
            ping_command(stream);
        }
        _ => panic!("Invalid command {}", command),
    }
}

/// Below method replies the `PING` command sent by redis client
fn ping_command(mut stream: TcpStream) {
    let pong = Value {
        value: Some("PONG".to_string()),
        value_type: parser::ValueType::SimpleString,
        null: false,
        array: Vec::new()
    };
    let _ = stream.write_all(parser::stringify(&pong).as_bytes());
}

/// Main entry point of the program, here in the code we're creating a server
/// listening to the default redis port `6379`
fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Unable to bind address ::6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => panic!("{}", e),
        }
    }
}
