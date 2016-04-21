#[allow(dead_code)]
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::fs::OpenOptions;
use std::error::Error;
use job::FileJob;
use chrono::datetime::DateTime;
use chrono::UTC;
use concurrent_hashmap::*;
use std::default::Default;
const LOGGER_FILE: &'static str = "log.txt";

#[derive(Clone,Debug)]
pub struct Cache {
    pub data: Vec<u8>, // last_modified: DateTime<UTC>,
}

// impl Clone for Receiver<Cache>{
//     fn clone(&self)->Self{
//         Cache{
//             data:self.data.clone(),
//         }
//     }
// }

#[allow(dead_code)]
pub struct ThreadPool {
    heap: Arc<Mutex<BinaryHeap<FileJob>>>,
    rx: Arc<Mutex<Receiver<FileJob>>>,
    tx: Sender<FileJob>,
    logger_tx: Sender<String>,
    cache: Arc<ConcHashMap<String, Cache>>,
    cache_tx: Sender<(String, Cache)>,
}

impl ThreadPool {
    pub fn new(special_threads: usize, normal_threads: usize) -> ThreadPool {
        let heap = Arc::new(Mutex::new(BinaryHeap::<FileJob>::new()));
        let (tx, rx) = channel::<FileJob>();
        let (logger_tx, logger_rx) = channel::<String>();
        let (cache_tx, cache_rx) = channel::<(String, Cache)>();

        let rx = Arc::new(Mutex::new(rx));
        ThreadPool::spin_logger_thread("logger".to_string(), logger_rx);
        for thread_id in 0..special_threads {
            let thread_name = format!("special_{}", thread_id);
            ThreadPool::spin_special_threads(thread_name,
                                             rx.clone(),
                                             heap.clone(),
                                             logger_tx.clone());
        }

        let cache: Arc<ConcHashMap<String, Cache>> = Default::default();
        let thread_name = format!("cache_{}", 1);
        ThreadPool::spin_cache_thread(thread_name, cache.clone(), cache_rx);
        for thread_id in 0..normal_threads {
            let thread_name = format!("normal_{}", thread_id);
            ThreadPool::spin_normal_threads(thread_name,
                                            heap.clone(),
                                            logger_tx.clone(),
                                            cache.clone(),
                                            cache_tx.clone());
        }
        ThreadPool {
            heap: heap,
            rx: rx.clone(),
            tx: tx,
            logger_tx: logger_tx.clone(),
            cache: cache,
            cache_tx: cache_tx.clone(),
        }
    }

    fn spin_cache_thread(thread_name: String,
                         cache: Arc<ConcHashMap<String, Cache>>,
                         cache_rx: Receiver<(String, Cache)>) {
        let result = thread::Builder::new().name(thread_name).spawn(move || {
            loop {
                match cache_rx.recv() {
                    Ok(tuple_obj) => {
                        let key = tuple_obj.0;
                        let cache_obj = tuple_obj.1;
                        println!("Caching key {:?}", key);
                        cache.insert(key, cache_obj);
                    }
                    Err(e) => {
                        println!("Error while caching\t{:?}", e.description());
                    }
                }
            }
        });
        match result {
            Err(e) => {
                println!("{:?}", e.description());
            }
            Ok(_) => {}
        }
    }
    fn spin_logger_thread(thread_name: String, logger_rx: Receiver<String>) {
        let result = thread::Builder::new().name(thread_name).spawn(move || {
            ThreadPool::logger(logger_rx);
        });
        match result {
            Err(e) => {
                println!("{:?}", e.description());
            }
            Ok(_) => {}
        }
    }


    fn logger(logger_rx: Receiver<String>) {
        let mut log_file = OpenOptions::new()
                               .create(true)
                               .write(true)
                               .append(true)
                               .open(LOGGER_FILE.to_string())
                               .unwrap();
        loop {
            match logger_rx.recv() {
                Ok(log) => {
                    match log_file.write(log.as_bytes()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error in writing to log file {:?}", e.description());
                        }
                    }
                }
                Err(e) => {
                    println!("{:?}", e.description());
                    break;
                }
            };
        }
    }

    fn spin_special_threads(thread_name: String,
                            rx: Arc<Mutex<Receiver<FileJob>>>,
                            heap: Arc<Mutex<BinaryHeap<FileJob>>>,
                            logger_tx: Sender<String>) {
        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            loop {
                let message = {
                    let job_receiver = rx.lock().unwrap();
                    job_receiver.recv()
                };
                match message {
                    Ok(job) => {
                        let mut heap_ref = heap.lock().unwrap();
                        logger_tx.send(format!("Pushing job {} from special thread {}\n",
                                               &job,
                                               thread_name))
                                 .unwrap();
                        heap_ref.push(job);

                    }
                    Err(e) => {
                        println!("{:?}", e.description());
                    }
                }
            }
        });
        match result {
            Ok(_) => println!("Special thread started"),
            Err(e) => println!("Error:{}", e.description()),
        }
    }

    fn spin_normal_threads(thread_name: String,
                           heap: Arc<Mutex<BinaryHeap<FileJob>>>,
                           logger_tx: Sender<String>,
                           mut cache: Arc<ConcHashMap<String, Cache>>,
                           mut cache_tx: Sender<(String, Cache)>) {

        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            loop {
                let data = {
                    let mut heap_ref = heap.lock().unwrap();
                    heap_ref.pop()
                };
                match data {
                    None => {
                        continue;
                    }
                    Some(mut filejob) => {
                        let log = filejob.handle_client_with_cache(&mut cache, &mut cache_tx);
                        match logger_tx.send(log) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Normal Logger Send Error{:?}", e.description());
                            }
                        }
                    }
                }
            }
        });
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e.description());
            }
        }
    }

    // pub fn print_heap(&self) {
    //     let heap = self.heap.clone();
    //     println!("{:?}", heap);
    // }

    pub fn execute(&self, data: FileJob) {
        match self.tx.send(data) {
            Ok(_) => {}
            Err(e) => {
                println!("Error occured while sending job {}\n", e.description());
            }
        }
    }
}
