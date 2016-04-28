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
  ./web_server <SchedulerThreadsCount> <WorkerThreadsCount> <LoggerThreadsCount> <CacheThreadsCount>
Options:
  -h --help     Show this screen.
";

/* Struct for storing main program arguments specifying number of each thread to spawn */
#[derive(Debug, RustcDecodable)]
struct Args {
    arg_SchedulerThreadsCount: usize,
    arg_WorkerThreadsCount: usize,
    arg_LoggerThreadsCount: usize,
    arg_CacheThreadsCount: usize,
}

/*
 * Given the number of each type of thread to spawn, return
 * a new thread pool with the corresponding number of each thread
 */
fn init_server(scheduler_threads_count:usize,
    worker_threads_count:usize,
    logger_threads_count:usize,cache_threads_count:usize) -> ThreadPool {
    let cpu_count = num_cpus::get();
    assert!(cpu_count > 0);

    ThreadPool::new(scheduler_threads_count,worker_threads_count,logger_threads_count,cache_threads_count)
}

/*
 * Main method that reads program arguments to determine number of
 * each thread to spawn, then constantly listens on TCP port for
 * incoming HTTP requests to serve. For each request, send the stream
 * to the thread pool.
 */
fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    let pool = init_server(args.arg_SchedulerThreadsCount,
    args.arg_WorkerThreadsCount,
    args.arg_LoggerThreadsCount,
    args.arg_CacheThreadsCount);
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and process them, sending the stream to the thread pool
    // for further handling
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
