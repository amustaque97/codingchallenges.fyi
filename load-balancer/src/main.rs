use std::{
    io::Read,
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
};

struct RoundRobinLoadBalancer {
    servers: Vec<String>,
    current: usize,
}

impl RoundRobinLoadBalancer {
    fn new(servers: Vec<String>) -> RoundRobinLoadBalancer {
        RoundRobinLoadBalancer {
            servers,
            current: 0,
        }
    }

    fn next_server(&mut self) -> String {
        let server = &self.servers[self.current];
        self.current = (self.current + 1) % self.servers.len();
        server.to_string()
    }
}

fn handle_client(stream: &mut TcpStream, load_balancer: &mut RoundRobinLoadBalancer) {
    let server = load_balancer.next_server();

    let mut backend_server = TcpStream::connect(&server)
        .expect(format!("Unable to connect server: {}", &server).as_str());
    println!("[*] connected to server: {}", &server);

    let mut reader = io::BufReader::new(stream.try_clone().expect("fail to clone tcpstream..."));
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());

    backend_server.write_all(&received[0..received.len()]);
    backend_server.flush();
    println!("[*] Wrote data to server: {}", &server);

    let mut backend_reader = io::BufReader::new(&mut backend_server);
    let mut stream_writer =
        io::BufWriter::new(stream.try_clone().expect("fail to clone tcpstream..."));

    loop {
        let backend_received: Vec<u8> = backend_reader.fill_buf().unwrap().to_vec();
        backend_reader.consume(backend_received.len());

        stream_writer.write_all(&backend_received);
        stream_writer.flush();

        if backend_received.is_empty() {
            stream.shutdown(std::net::Shutdown::Both);
            return;
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    let servers = vec![
        "127.0.0.1:8080".to_string(),
        "127.0.0.1:8081".to_string(),
        "127.0.0.1:8082".to_string(),
    ];
    let mut load_balancer = RoundRobinLoadBalancer::new(servers);

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?, &mut load_balancer);
    }
    Ok(())
}
