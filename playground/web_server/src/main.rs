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

fn init_server() -> ThreadPool {
    println!("{}", num_cpus::get());
    let cpu_count = num_cpus::get();
    assert!(cpu_count > 0);

    // initialize threadpool with 2 times the number of threads as the number of cpus
    ThreadPool::new(2 * cpu_count)
}

fn get_path_from_request(request: &str) -> &str {

    let mut abs_path = PathBuf::new();
    let curr_dir = env::current_dir().unwrap();
    abs_path.push(curr_dir);
    println!("{}", abs_path.display());
    let mut iter = request.split_whitespace();
    iter.next();
    let requested_path = iter.next().unwrap();
    abs_path.push(requested_path);
    println!("{}", abs_path.display());
    abs_path;
    // TODO: Rewrite to avoid creating new string
    &requested_path[1..]

    //for word in request.split_whitespace() {
    //    println!("{:?}", word);
    //}
}

fn handle_client(mut stream: TcpStream) {

    /*
    let request = Request::new(stream);
    let filename = request.filename;
    try to open the file
    if success
       let response = Response::new(wofjaoij);
    else
       let response = Response::new(aoweifo);
    "send"


       */
    // string to hold request body
    let mut request = String::new();
    println!("Ready to read");
   
    // TODO: Content length should be inferred from the request, not hardcoded
    let mut contents = [0; 1000];

    // read request into string
    let result = stream.read(&mut contents);
    match result {
        Ok(result) => {
            let string_result = str::from_utf8(&contents).unwrap();
            println!("{}", string_result);
            let pathname = get_path_from_request(string_result);
            println!("{:?}", pathname);
            let f = match File::open(pathname) {
                Ok(mut f) => {
                    println!("OK!");
                    let mut s = String::new();
                    f.read_to_string(&mut s);
                    println!("{}",s);
                    stream.write(s.as_bytes());
                }
                Err(e) => {
                    println!("NOT OK!");
                    let mut error_file = File::open("error.html").unwrap();
                    let mut error_vec = Vec::new();
                    error_file.read_to_end(&mut error_vec);
                    let error_byte_array = error_vec.as_slice();
                    stream.write(error_byte_array);
                }
            };
        }
        Err(e) => { 
            println!("Error when reading request");
        }
    }
    
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
                /*
                thread::spawn(move|| {
                    println!("connection succeeded");
                    handle_client(stream)
                });*/

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
