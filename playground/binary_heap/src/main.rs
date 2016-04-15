extern crate rand;
use rand::Rng;
mod threadsmanager;
mod Request;
use Request::FileJob;
use threadsmanager::ThreadPool;

pub fn init_threads(thread_count: usize)->ThreadPool {
    ThreadPool::new(thread_count)
}

fn main() {
    let mut rng = rand::thread_rng();
    let thread_pool = init_threads(8 as usize);
    for iter_count in 0..100 {
        let filejob = FileJob::new(format!("file{}.txt", rng.gen::<u8>()), rng.gen::<u8>());
    }
}
