extern crate rand;
use std::io::prelude::*;
mod threadpool;
use threadpool::ThreadPool;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

fn main(){
    let pool = ThreadPool::new(4,4);
    let mut rng = rand::thread_rng();
    for _ in 0..25{
        pool.execute(rng.gen::<u32>());
    }

    sleep(Duration::new(2,0));
    pool.print_heap();
}
