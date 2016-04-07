extern crate threadpool;
extern crate num_cpus;

mod request;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::fs::File;
use std::str;
use std::env;
use std::path::PathBuf;
use threadpool::ThreadPool;
use request::Request;
use std::sync::mpsc::channel;

const BUFFER_SIZE: usize = 20;

fn init_server() -> ThreadPool {
    println!("{}", num_cpus::get());
    let cpu_count = num_cpus::get();
    assert!(cpu_count > 0);

    // initialize threadpool with 2 times the number of threads as the number of cpus
    let (tx, rx) = channel();
    ThreadPool::new(2 * cpu_count, tx)
}


fn handle_client(mut stream: TcpStream) {

    let request_obj = Request::new(&mut stream);

    let f = match File::open(request_obj.get_filename()) {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s);
            stream.write(s.as_bytes());
        }
        Err(e) => {
            let mut error_file = File::open("error.html").unwrap();
            let mut error_vec = Vec::new();
            error_file.read_to_end(&mut error_vec);
            let error_byte_array = error_vec.as_slice();
            stream.write(error_byte_array);
        }
    };

}


fn main() {

    let pool = init_server();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // use move closure to give ownership of the stream to the
                // child thread
                pool.execute(move|| {
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
