// extern crate threadpool;
extern crate num_cpus;
extern crate chrono;
extern crate rand;

mod request;
mod threadmanager;
mod job;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::fs::{OpenOptions, File};
use std::str;
use std::env;
use std::path::PathBuf;
// use threadpool::ThreadPool;
use threadmanager::ThreadPool;
use request::Request;
use job::FileJob;
use std::sync::mpsc::{Sender, Receiver, channel};
use rand::Rng;
use chrono::*;

const BUFFER_SIZE: usize = 4096;
const LOGGER_FILE: &'static str = "log.txt";

fn init_server() -> ThreadPool {
    println!("{}", num_cpus::get());
    let cpu_count = num_cpus::get();
    assert!(cpu_count > 0);

    // initialize threadpool with 2 times the number of threads as the number of cpus
    ThreadPool::new(cpu_count,cpu_count)
}

enum Status {
    OK,
    BAD_REQUEST,
    NOT_FOUND
}

impl Status {
    fn get_info(status: Status) -> StatusCode {
        match status {
            Status::OK => StatusCode { name: "OK".to_string(), response_code: 200 },
            Status::BAD_REQUEST => StatusCode { name: "BAD REQUEST".to_string(), response_code: 400 },
            Status::NOT_FOUND => StatusCode { name: "NOT FOUND".to_string(), response_code: 404 }
        }
    }
}

struct StatusCode {
    name: String,
    response_code: u8
}

fn handle_client(mut stream: TcpStream, tx: Sender<String>) {

    let request_obj = Request::new(&mut stream);

    let f = match File::open(request_obj.get_filename()) {
        Ok(mut f) => {
            //let mut content = String::new();
            //f.read_to_string(&mut content);

            let status = Status::get_info(Status::OK);
            let response_header = format!("{} {}", status.response_code, status.name);
            // write response header to stream
            stream.write(format!("{} {}\n\n", request_obj.get_protocol(), response_header).as_bytes());

            let mut read_buf = [0; BUFFER_SIZE];

            loop {
                let bytes_read = f.read(&mut read_buf);
                match bytes_read {
                    Ok(bytes_read) => {
                        stream.write(&read_buf);
                        if bytes_read < BUFFER_SIZE {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Error reading file contents");
                        break;
                    }
                };
            }


            //let response_str = format!("{} {}\n\n{}", request_obj.get_protocol(), response_header, content);
            //stream.write(response_str.as_bytes());
            let mut log: String = String::new();
            let dt = UTC::now();
            let timestamp = dt.format("%Y-%m-%d %H:%M:%S").to_string();
            let request_str = request_obj.to_string();
            let log = format!("{}\t{}\t{}\n", timestamp, request_str, response_header);

            tx.send(log);
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

fn init_logger_thread() -> Sender<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    thread::spawn(move|| {
        logger(rx);
    });
    tx
}

fn logger(rx: Receiver<String>) {
    let mut log_file = OpenOptions::new().create(true).write(true).append(true).open(LOGGER_FILE.to_string()).unwrap();
    loop {
        let log = rx.recv().unwrap();
        log_file.write(log.as_bytes());
    }
}

fn main() {

    let pool = init_server();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // let tx: Sender<String> = init_logger_thread();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                // let tx_clone = tx.clone();

                // use move closure to give ownership of the stream to the
                // child thread
                // pool.execute(move|| {
                //     println!("connection succeeded");
                //     handle_client(stream, tx_clone)
                // });
                let mut rng = rand::thread_rng();
                pool.execute(FileJob::new_test(stream,rng.gen::<u64>()));

            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}
