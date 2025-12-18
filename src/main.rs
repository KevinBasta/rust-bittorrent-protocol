use std::{env, io::{Read, Write}, net::{TcpListener, TcpStream}, thread};



fn protocol(stream: &mut TcpStream) {
    let mut buffer: [u8; 128] = [0; 128]; 
    let mut result = stream.read(&mut buffer).unwrap();
    println!("read request: ");
    
    for byte in buffer.iter() {
        let char = *byte as char;
        print!("{char}");
    }
    println!("");
    
    result = stream.write(b"peer server response").unwrap();
    println!("sent response");
}

fn requestPeer() {
    let mut stream = TcpStream::connect("localhost:9090").unwrap();
    println!("connected");
    
    let mut result = stream.write(b"request").unwrap();
    println!("sent request");
    
    let mut buffer: [u8; 128] = [0; 128]; 
    result = stream.read(&mut buffer).unwrap();
    println!("got response: ");

    for byte in buffer.iter() {
        let char = *byte as char;
        print!("{char}");
    }
    println!("");

}

fn acceptPeer() {
    let listener = TcpListener::bind("localhost:9090").unwrap();
    let addr = listener.local_addr().unwrap();
    println!("{addr}");
    
    // spawn a detatched thread for each incoming connection
    while (true) {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Spawning thread for {addr}");
        thread::spawn(move || protocol(&mut stream));

    }
}

fn main() {
    println!("Hello world");

    let args: Vec<String> = env::args().collect();
    let doRequest = &args[1];
    
    if (doRequest == "1") {
        requestPeer();
    } else {
        acceptPeer();
    }
}