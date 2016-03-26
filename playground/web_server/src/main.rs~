use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::fs::File;
use std::str;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    fn handle_client(mut stream: TcpStream) {
        // string to hold request body
        let mut request = String::new();
        println!("Ready to read");
       
        let mut contents = [0; 1024];
        // read request into string
        let result = stream.read(&mut contents);
        match result {
            Ok(result) => {
                // convert byte array to byte vector
                // convert byte vector to string
                let string_result = str::from_utf8(&contents).unwrap();
                println!("{:?}", string_result);
                println!("{:?}", result);
            }
            Err(e) => { 
                println!("Error when reading from stream");
            }
        }
        //stream.read_to_string(&mut request).unwrap();
        //println!("{:?}", request);
        let mut f = File::open("foo.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s);
        println!("{}",s);
        stream.write(s.as_bytes());
        println!("handling client!");
    }

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // use move closure to give ownership of the stream to the
                // child thread
                thread::spawn(move|| {
                    println!("connection succeeded");
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}
