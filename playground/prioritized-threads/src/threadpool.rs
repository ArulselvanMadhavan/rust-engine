use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel, SendError};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub struct ThreadPool {
    heap: Arc<Mutex<BinaryHeap<u32>>>,
    rx: Arc<Mutex<Receiver<u32>>>,
    tx: Sender<u32>,
}

impl ThreadPool {
    pub fn new(special_threads: usize, normal_threads: usize) -> ThreadPool {
        let heap = Arc::new(Mutex::new(BinaryHeap::<u32>::new()));
        let (tx, rx) = channel::<u32>();
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..special_threads {
            ThreadPool::spin_special_threads(rx.clone(), heap.clone());
        }

        for thread_id in 0..normal_threads {
            ThreadPool::spin_normal_threads(thread_id, heap.clone());
        }
        ThreadPool {
            heap: heap,
            rx: rx.clone(),
            tx: tx,
        }
    }

    pub fn spin_special_threads(rx: Arc<Mutex<Receiver<u32>>>, heap: Arc<Mutex<BinaryHeap<u32>>>) {
        thread::spawn(move || {
            println!("Special thread started");
            loop {
                let message = {
                    let job_receiver = rx.lock().unwrap();
                    job_receiver.recv()
                };
                match message {
                    Ok(job) => {
                        let mut heap_ref = heap.lock().unwrap();
                        heap_ref.push(job);
                    }
                    _ => {
                        println!("Invalid Job sent");
                    }
                }
            }
        });
    }

    pub fn spin_normal_threads(thread_id: usize, heap: Arc<Mutex<BinaryHeap<u32>>>) {
        thread::spawn(move || {
            sleep(Duration::new(1, 0));
            loop {
                let data = {
                    let mut heap_ref = heap.lock().unwrap();
                    heap_ref.pop()
                };
                sleep(Duration::new(0,10000));
                match data{
                    None =>{
                        continue;
                    },
                    data_u32 => println!("TID:{:?} Popped {:?}", thread_id, data_u32)
                }
            }
        });
    }

    pub fn print_heap(&self) {
        let heap = self.heap.clone();
        println!("{:?}", heap);
    }

    pub fn execute(&self, data: u32) -> Result<(), SendError<u32>> {
        self.tx.send(data)
    }
}
