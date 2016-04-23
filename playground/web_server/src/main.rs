extern crate num_cpus;
extern crate chrono;
extern crate rand;
extern crate concurrent_hashmap;
extern crate rustc_serialize;
extern crate docopt;

mod request;
mod threadmanager;
mod job;
mod error;

use std::net::{TcpListener};
use threadmanager::ThreadPool;
use std::error::Error;
use docopt::Docopt;

const USAGE: &'static str = "
Rust Web server.
Usage:
  ./web_server <SpecialThreadsCount> <WorkerThreadsCount> <LoggerThreadsCount> <CacheThreadsCount>
Options:
  -h --help     Show this screen.
";


#[derive(Debug, RustcDecodable)]
struct Args {
    arg_SpecialThreadsCount: usize,
    arg_WorkerThreadsCount: usize,
    arg_LoggerThreadsCount: usize,
    arg_CacheThreadsCount: usize,
}

fn init_server(special_threads_count:usize,
    worker_threads_count:usize,
    logger_threads_count:usize,cache_threads_count:usize) -> ThreadPool {
    println!("CPU count:{}", num_cpus::get());
    let cpu_count = num_cpus::get();
    assert!(cpu_count > 0);

    // initialize threadpool with 2 times the number of threads as the number of cpus
    ThreadPool::new(special_threads_count,worker_threads_count,logger_threads_count,cache_threads_count)
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    let pool = init_server(args.arg_SpecialThreadsCount,
    args.arg_WorkerThreadsCount,
    args.arg_LoggerThreadsCount,
    args.arg_CacheThreadsCount);
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(stream);
            }
            Err(e) => { println!("{:?}",e.description() ); }
        }
    }

    // close the socket server
    drop(listener);
}
