use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel, SendError};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::fs::{OpenOptions, File};
use std::error::Error;

const LOGGER_FILE: &'static str = "log.txt";

pub struct ThreadPool {
    heap: Arc<Mutex<BinaryHeap<u32>>>,
    rx: Arc<Mutex<Receiver<u32>>>,
    tx: Sender<u32>,
    logger_tx: Sender<String>,
    // logger_rx: Receiver<String>,
}

impl ThreadPool {
    pub fn new(special_threads: usize, normal_threads: usize) -> ThreadPool {
        let heap = Arc::new(Mutex::new(BinaryHeap::<u32>::new()));
        let (tx, rx) = channel::<u32>();
        let (logger_tx, logger_rx) = channel::<String>();
        let rx = Arc::new(Mutex::new(rx));
        ThreadPool::spin_logger_thread("logger".to_string(), logger_rx);
        for thread_id in 0..special_threads {
            let thread_name = format!("special_{}", thread_id);
            ThreadPool::spin_special_threads(thread_name,
                                             rx.clone(),
                                             heap.clone(),
                                             logger_tx.clone());
        }

        for thread_id in 0..normal_threads {
            let thread_name = format!("special_{}", thread_id);
            ThreadPool::spin_normal_threads(thread_name, heap.clone(), logger_tx.clone());
        }
        ThreadPool {
            heap: heap,
            rx: rx.clone(),
            tx: tx,
            logger_tx: logger_tx.clone(),
            // logger_rx: logger_rx,
        }
    }

    fn spin_logger_thread(thread_name: String, logger_rx: Receiver<String>) {
        thread::Builder::new().name(thread_name).spawn(move || {
            ThreadPool::logger(logger_rx);
        });
    }


    fn logger(logger_rx: Receiver<String>) {
        let mut log_file = OpenOptions::new()
                               .create(true)
                               .write(true)
                               .append(true)
                               .open(LOGGER_FILE.to_string())
                               .unwrap();
        loop {
            // let log = logger_rx.recv().unwrap();
            match logger_rx.recv() {
                Ok(log) => log_file.write(log.as_bytes()),
                Err(e) => {
                    println!("{:?}", e.description());
                    break;
                }
            };
        }
    }

    fn spin_special_threads(thread_name: String,
                            rx: Arc<Mutex<Receiver<u32>>>,
                            heap: Arc<Mutex<BinaryHeap<u32>>>,
                            logger_tx: Sender<String>) {
        let result = thread::Builder::new().name(thread_name).spawn(move || {
            loop {
                let message = {
                    println!("Special unwrap receiver");
                    let job_receiver = rx.lock().unwrap();
                    job_receiver.recv()
                };
                match message {
                    Ok(job) => {
                        println!("Special unwrap heap");
                        let mut heap_ref = heap.lock().unwrap();
                        heap_ref.push(job);
                    }
                    _ => {
                        println!("Invalid Job sent");
                    }
                }
            }
        });
        match result {
            Ok(_) => println!("Special thread started"),
            Err(e) => println!("Special thread creation failed"),
        }
    }

    fn spin_normal_threads(thread_name: String,
                           heap: Arc<Mutex<BinaryHeap<u32>>>,
                           logger_tx: Sender<String>) {
        thread::Builder::new().name(thread_name.clone()).spawn(move || {
            sleep(Duration::new(1, 0));
            loop {
                let data = {
                    let mut heap_ref = heap.lock().unwrap();
                    heap_ref.pop()
                };
                sleep(Duration::new(0, 10000));
                match data {
                    None => {
                        continue;
                    }
                    Some(data_u32) => println!("TID:{:?} Popped {:?}", thread_name, data_u32),
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
