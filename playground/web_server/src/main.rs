// extern crate threadpool;
extern crate num_cpus;
extern crate chrono;
extern crate rand;
extern crate concurrent_hashmap;

mod request;
mod threadmanager;
mod job;

use std::net::{TcpListener};
// use std::thread;
// use std::io::prelude::*;
// use std::fs::{OpenOptions, File};

use threadmanager::ThreadPool;
// use request::Request;
use job::FileJob;
// use std::sync::mpsc::{Sender, Receiver, channel};
use rand::Rng;

use std::error::Error;


// const LOGGER_FILE: &'static str = "log.txt";

fn init_server() -> ThreadPool {
    println!("{}", num_cpus::get());
    let cpu_count = num_cpus::get();
    assert!(cpu_count > 0);

    // initialize threadpool with 2 times the number of threads as the number of cpus
    ThreadPool::new(cpu_count,cpu_count)
    // ThreadPool::new(1, 4)
}


// fn init_logger_thread() -> Sender<String> {
//     let (tx, rx): (Sender<String>, Receiver<String>) = channel();
//     thread::spawn(move|| {
//         logger(rx);
//     });
//     tx
// }
//
// fn logger(rx: Receiver<String>) {
//     let mut log_file = OpenOptions::new().create(true).write(true).append(true).open(LOGGER_FILE.to_string()).unwrap();
//     loop {
//         match rx.recv() {
//             Ok(log) => {log_file.write(log.as_bytes());},
//             Err(e) => {println!("Logger-Recv attempt {:?}",e.description());}
//         };
//     }
// }

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
                pool.execute(FileJob::new(stream));

            }
            Err(e) => { println!("{:?}",e.description() ); }
        }
    }

    // close the socket server
    drop(listener);
}
