use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use dictionary_server::DictionaryServer;
use parser::Value;

mod dictionary_server;
mod parser;

/// Basic setup on how to handle the connections and reply accordingly
fn handle_connection(stream: TcpStream, map: &mut DictionaryServer) {
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
        "ECHO" => {
            echo_command(stream, value.array[1..].to_vec());
        }
        "SET" | "set" => {
            set_command(stream, value.array[1..].to_vec(), map);
        }
        "GET" | "get" => {
            get_command(stream, value.array[1].clone(), map);
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
        array: Vec::new(),
    };
    let _ = stream.write_all(parser::stringify(&pong).as_bytes());
}

/// Method to echo the same string which was sent by the client. NOTE
/// `ECHO` command considers that the input will be only ECHO "<string>" where
/// `<string>` can have n characters but inside the quotes. There are no other strings
/// after that.
fn echo_command(mut stream: TcpStream, values: Vec<Value>) {
    let string = values[0].value.clone().unwrap_or("".to_string());
    let reply = Value {
        value: Some(string),
        value_type: parser::ValueType::SimpleString,
        null: false,
        array: Vec::new(),
    };
    let _ = stream.write_all(parser::stringify(&reply).as_bytes());
}

/// wrapper around the dictionary i.e. `HashMap` to set the key, value and reply back
/// in RESP protocol to the client. If it is success reply will be "OK" else it should panic
fn set_command(mut stream: TcpStream, values: Vec<Value>, map: &mut DictionaryServer) {
    let key = values[0]
        .value
        .clone()
        .expect("Unable to extract key from SET command");
    let val = values[1]
        .value
        .clone()
        .expect("Unable to extract value from SET command");

    let _ = map.set(&key, &val);
    let ok = Value {
        value: Some("OK".to_string()),
        value_type: parser::ValueType::SimpleString,
        null: false,
        array: Vec::new(),
    };
    let _ = stream.write_all(parser::stringify(&ok).as_bytes());
}

/// wrapper around the dictionary i.e. `HashMap` to retrive the key and reply back
/// in RESP protocol. If key is not present in the dictionary then return `nil` as response.
fn get_command(mut stream: TcpStream, value: Value, map: &mut DictionaryServer) {
    let key = value
        .value
        .clone()
        .expect("Unable to extract key from GET command");
    let reply = Value {
        value: Some(map.get(&key)),
        value_type: parser::ValueType::SimpleString,
        null: false,
        array: Vec::new(),
    };
    let _ = stream.write_all(parser::stringify(&reply).as_bytes());
}

/// Main entry point of the program, here in the code we're creating a server
/// listening to the default redis port `6379`
fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Unable to bind address ::6379");

    let map: Arc<Mutex<DictionaryServer>> = Arc::new(Mutex::new(DictionaryServer::new()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let m = map.clone();
                let _ = thread::spawn(move || {
                    handle_connection(stream, &mut m.lock().unwrap());
                })
                .join();
            }
            Err(e) => panic!("{}", e),
        }
    }
}
