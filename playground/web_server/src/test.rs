extern crate threadpool;
extern crate num_cpus;


use threadpool::ThreadPool;
use request::Request;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

// fn init_server() -> ThreadPool {
//     println!("{}", num_cpus::get());
//     let cpu_count = num_cpus::get();
//     assert!(cpu_count > 0);

// let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

//     // initialize threadpool with 2 times the number of threads as the number of cpus
//     // save a thread for the logger thread
//     ThreadPool::new(2 * cpu_count, tx)

// }



fn main() {

    println!("{}", num_cpus::get());
    let cpu_count = num_cpus::get();
    assert!(cpu_count > 0);

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    // initialize threadpool with 2 times the number of threads as the number of cpus
    // save a thread for the logger thread
    let pool = ThreadPool::new(2 * cpu_count - 1, tx);

    pool.execute(move || {
        println!("Job is executing on a thread");
    });

    loop {
        println!("{:?}", rx.recv().unwrap());
    }
}
