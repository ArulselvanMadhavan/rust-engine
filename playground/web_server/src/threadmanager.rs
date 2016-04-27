#[allow(dead_code)]
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel, RecvError};
use std::thread;
use std::fs::OpenOptions;
use std::error::Error;
use job::FileJob;
use concurrent_hashmap::*;
use std::default::Default;
use std::net::TcpStream;
const LOGGER_FILE: &'static str = "log.txt";

#[derive(Clone,Debug)]
pub struct Cache {
    pub data: Vec<u8>
}

pub struct SpecialThreadStats {
    thread_name: String,
    rx: Arc<Mutex<Receiver<TcpStream>>>,
    heap: Arc<Mutex<BinaryHeap<FileJob>>>,
    logger_tx: Sender<String>,
}

impl Drop for SpecialThreadStats {
    fn drop(&mut self) {
        ThreadPool::spin_special_threads(self.thread_name.to_owned(),
                                         self.rx.to_owned(),
                                         self.heap.to_owned(),
                                         self.logger_tx.to_owned());
    }
}

pub struct NormalThreadStats {
    thread_name: String,
    heap: Arc<Mutex<BinaryHeap<FileJob>>>,
    logger_tx: Sender<String>,
    cache: Arc<ConcHashMap<String, Cache>>,
    cache_tx: Sender<(String, Cache)>,
}

impl Drop for NormalThreadStats {
    fn drop(&mut self) {
        println!("Restarting normal thread");
        ThreadPool::spin_normal_threads(self.thread_name.to_owned(),
                                        self.heap.to_owned(),
                                        self.logger_tx.to_owned(),
                                        self.cache.to_owned(),
                                        self.cache_tx.to_owned());
    }
}


pub struct LoggerThreadStats {
    thread_name: String,
    logger_rx: Arc<Mutex<Receiver<String>>>,
}

impl Drop for LoggerThreadStats {
    fn drop(&mut self) {
        println!("Restarting Logger thread");
        ThreadPool::spin_logger_thread(self.thread_name.to_owned(), self.logger_rx.clone());
    }
}

pub struct CacheThreadStats {
    thread_name: String,
    cache: Arc<ConcHashMap<String, Cache>>,
    cache_rx: Arc<Mutex<Receiver<(String, Cache)>>>,
}

impl Drop for CacheThreadStats {
    fn drop(&mut self) {
        println!("Restarting Cache thread");
        ThreadPool::spin_cache_thread(self.thread_name.to_owned(),
                                      self.cache.to_owned(),
                                      self.cache_rx.to_owned())
    }
}

#[allow(dead_code)]
pub struct ThreadPool {
    heap: Arc<Mutex<BinaryHeap<FileJob>>>,
    rx: Arc<Mutex<Receiver<TcpStream>>>,
    tx: Sender<TcpStream>,
    logger_tx: Sender<String>,
    cache: Arc<ConcHashMap<String, Cache>>,
    cache_tx: Sender<(String, Cache)>,
}

impl ThreadPool {
    pub fn new(special_threads: usize,
               normal_threads: usize,
               logger_threads: usize,
               cache_threads: usize)
               -> ThreadPool {
        let heap = Arc::new(Mutex::new(BinaryHeap::<FileJob>::new()));
        let (tx, rx) = channel::<TcpStream>();
        let (logger_tx, logger_rx) = channel::<String>();
        let (cache_tx, cache_rx) = channel::<(String, Cache)>();

        let rx = Arc::new(Mutex::new(rx));
        let logger_rx = Arc::new(Mutex::new(logger_rx));
        let cache_rx = Arc::new(Mutex::new(cache_rx));
        for thread_id in 0..logger_threads {
            let thread_name = format!("logger_{}", thread_id);
            ThreadPool::spin_logger_thread(thread_name, logger_rx.clone());
        }
        for thread_id in 0..special_threads {
            let thread_name = format!("special_{}", thread_id);
            ThreadPool::spin_special_threads(thread_name,
                                             rx.clone(),
                                             heap.clone(),
                                             logger_tx.clone());
        }

        let cache: Arc<ConcHashMap<String, Cache>> = Default::default();
        for thread_id in 0..cache_threads {
            let thread_name = format!("cache_{}", thread_id);
            ThreadPool::spin_cache_thread(thread_name, cache.clone(), cache_rx.clone());
        }
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
                         cache_rx: Arc<Mutex<Receiver<(String, Cache)>>>) {
        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            let thread_stats = CacheThreadStats {
                thread_name: thread_name,
                cache: cache,
                cache_rx: cache_rx,
            };
            loop {
                let message = {
                    let obj_receiver = thread_stats.cache_rx.lock().unwrap();
                    obj_receiver.recv()
                };
                match message {
                    Ok(tuple_obj) => {
                        let key = tuple_obj.0;
                        let cache_obj = tuple_obj.1;
                        thread_stats.cache.insert(key, cache_obj);
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
    fn spin_logger_thread(thread_name: String, logger_rx: Arc<Mutex<Receiver<String>>>) {
        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            let mut thread_stats = LoggerThreadStats {
                thread_name: thread_name,
                logger_rx: logger_rx,
            };
            ThreadPool::logger(&mut thread_stats.logger_rx);
        });
        match result {
            Err(e) => {
                println!("{:?}", e.description());
            }
            Ok(_) => {}
        }
    }


    fn logger(logger_rx: &mut Arc<Mutex<Receiver<String>>>) {
        let mut log_file = OpenOptions::new()
                               .create(true)
                               .write(true)
                               .append(true)
                               .open(LOGGER_FILE.to_string())
                               .unwrap();
        loop {
            let message = {
                let msg_receiver = logger_rx.lock().unwrap();
                msg_receiver.recv()
            };
            match message {
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
                            rx: Arc<Mutex<Receiver<TcpStream>>>,
                            heap: Arc<Mutex<BinaryHeap<FileJob>>>,
                            logger_tx: Sender<String>) {
        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            let mut thread_stats = SpecialThreadStats {
                thread_name: thread_name,
                rx: rx,
                heap: heap,
                logger_tx: logger_tx,
            };
            loop {
                let message = {
                    let job_receiver = thread_stats.rx.lock().unwrap();
                    job_receiver.recv()
                };
                ThreadPool::process_filejob(message,
                                            &mut thread_stats.heap,
                                            &mut thread_stats.logger_tx,
                                            &thread_stats.thread_name);
            }
        });
        match result {
            Ok(_) => println!("Scheduler thread started"),
            Err(e) => println!("Error:{}", e.description()),
        }
    }

    fn process_filejob(message: Result<TcpStream, RecvError>,
                       heap: &mut Arc<Mutex<BinaryHeap<FileJob>>>,
                       logger_tx: &mut Sender<String>,
                       thread_name: &str) {
        match message {
            Ok(stream) => {
                match FileJob::new(stream) {
                    Ok(job) => {
                        /*let message: String = format!("Attempting to push job {} from special thread {}\n",
                                              &job,
                                              thread_name);
                        ThreadPool::send_to_logger(logger_tx, message, thread_name);*/
                        match heap.lock() {
                            Ok(mut mut_heap_ref) => {
                                mut_heap_ref.push(job);
                            }
                            Err(e) => {
                                println!("{:?}\tUnable to acquire lock on the heap\t{:?}",
                                         thread_name,
                                         e.description());
                            }
                        };
                    }
                    Err(_) => {
                        //let message = FileJob::send_bad_request_error(stream);
                        //ThreadPool::send_to_logger(logger_tx, message, thread_name);
                    }
                };
            }
            Err(e) => {
                println!("{:?}\tError in receiving filejob\t{:?}",
                         thread_name,
                         e.description());
            }
        };
    }
    #[allow(dead_code)]
    fn send_to_logger(logger_tx: &mut Sender<String>, message: String, thread_name: &str) {
        match logger_tx.send(message) {
            Ok(_) => {}
            Err(e) => {
                println!("{:?} received an error when sending to logger.\t{:?}",
                         thread_name,
                         e.description());
            }
        };
    }
    fn spin_normal_threads(thread_name: String,
                           heap: Arc<Mutex<BinaryHeap<FileJob>>>,
                           logger_tx: Sender<String>,
                           cache: Arc<ConcHashMap<String, Cache>>,
                           cache_tx: Sender<(String, Cache)>) {

        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            let mut thread_stats = NormalThreadStats {
                thread_name: thread_name,
                heap: heap,
                logger_tx: logger_tx,
                cache: cache,
                cache_tx: cache_tx,
            };
            loop {
                let data = {
                    let mut heap_ref = thread_stats.heap.lock().unwrap();
                    heap_ref.pop()
                };
                match data {
                    None => {
                        continue;
                    }
                    Some(mut filejob) => {
                        let log = filejob.handle_client_with_cache(&mut thread_stats.cache,
                                                                   &mut thread_stats.cache_tx);
                        match thread_stats.logger_tx.send(log) {
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
            Ok(_) => {
                println!("Worker thread started");
            }
            Err(e) => {
                println!("{:?}", e.description());
            }
        }
    }

    pub fn execute(&self, stream: TcpStream) {
        match self.tx.send(stream) {
            Ok(_) => {}
            Err(e) => {
                println!("Error occured while sending job {}\n", e.description());
            }
        }
    }
}
