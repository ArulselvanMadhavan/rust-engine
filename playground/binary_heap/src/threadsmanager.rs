use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::BinaryHeap;
use Request::FileJob;


// type Thunk<'a> = <Send + 'a>;

#[derive(Clone)]
pub struct ThreadPool {
    jobs: Sender<FileJob>,
    job_receiver: Arc<Mutex<Receiver<FileJob>>>,
    job_queue: Arc<Mutex<BinaryHeap<FileJob>>>,
}


impl ThreadPool {
    pub fn new(threads: usize) -> ThreadPool {
        assert!(threads >= 1);
        let (tx, rx) = channel::<FileJob>();
        let rx = Arc::new(Mutex::new(rx));
        let heap = Arc::new(Mutex::new(BinaryHeap::new()));
        for _ in 0..threads {
            // spawn_in_pool(rx.clone());
        }

        ThreadPool {
            jobs: tx,
            job_receiver: rx.clone(),
            job_queue: heap
        }
    }

    pub fn push(&mut self,filejob:FileJob) {
        self.jobs.send(filejob).unwrap();
    }
}
