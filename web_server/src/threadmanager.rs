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

/*
 * A struct representing a cache entry. Currently holds file contents as a Vec of bytes,
 * but can be extended to hold more information, such as last accessed time, etc.
 */
#[derive(Clone,Debug)]
pub struct Cache {
    pub data: Vec<u8>
}

/*
 * Struct to hold data for a scheduler thread.
 * thread_name is the name of the thread that will display if the thread panics
 * rx is the receiver end of the channel that the main thread sends TcpStreams through
 * heap is the priority queue
 * logger_tx is the transmitter end of the channel to send logging requests through
 */
pub struct SchedulerThreadStats {
    thread_name: String,
    rx: Arc<Mutex<Receiver<TcpStream>>>,
    heap: Arc<Mutex<BinaryHeap<FileJob>>>,
    logger_tx: Sender<String>,
}

/*
 * When a scheduler thread panics, spin up a new scheduler thread before exiting
 */
impl Drop for SchedulerThreadStats {
    fn drop(&mut self) {
        ThreadPool::spin_scheduler_threads(self.thread_name.to_owned(),
                                         self.rx.to_owned(),
                                         self.heap.to_owned(),
                                         self.logger_tx.to_owned());
    }
}

/*
 * Struct to hold data for a worker thread.
 * thread_name is the name of the thread that will display if the thread panics
 * heap is the priority queue
 * logger_tx is the transmitter end of the channel to send logging requests through
 * cache is the web server cache
 * cache_tx is the transmitter end of the channel to send caching requests through
 */
pub struct WorkerThreadStats {
    thread_name: String,
    heap: Arc<Mutex<BinaryHeap<FileJob>>>,
    logger_tx: Sender<String>,
    cache: Arc<ConcHashMap<String, Cache>>,
    cache_tx: Sender<(String, Cache)>,
}

/*
 * When a worker thread panics, spin up a new worker thread before exiting
 */
impl Drop for WorkerThreadStats {
    fn drop(&mut self) {
        println!("Restarting worker thread");
        ThreadPool::spin_worker_threads(self.thread_name.to_owned(),
                                        self.heap.to_owned(),
                                        self.logger_tx.to_owned(),
                                        self.cache.to_owned(),
                                        self.cache_tx.to_owned());
    }
}

/*
 * Struct to hold data for a logger thread.
 * thread_name is the name of the thread that will display if the thread panics
 * logger_rx is the receiver end of the channel to send logging requests through
 */
pub struct LoggerThreadStats {
    thread_name: String,
    logger_rx: Arc<Mutex<Receiver<String>>>,
}

/*
 * When a logger thread panics, spin up a new logger thread before exiting
 */
impl Drop for LoggerThreadStats {
    fn drop(&mut self) {
        println!("Restarting Logger thread");
        ThreadPool::spin_logger_thread(self.thread_name.to_owned(), self.logger_rx.clone());
    }
}

/*
 * Struct to hold data for a cache thread.
 * thread_name is the name of the thread that will display if the thread panics
 * cache is the web server cache
 * cache_rx is the receiver end of the channel to send caching requests through
 */
pub struct CacheThreadStats {
    thread_name: String,
    cache: Arc<ConcHashMap<String, Cache>>,
    cache_rx: Arc<Mutex<Receiver<(String, Cache)>>>,
}

/*
 * When a cache thread panics, spin up a new cache thing before exiting
 */
impl Drop for CacheThreadStats {
    fn drop(&mut self) {
        println!("Restarting Cache thread");
        ThreadPool::spin_cache_thread(self.thread_name.to_owned(),
                                      self.cache.to_owned(),
                                      self.cache_rx.to_owned())
    }
}

/*
 * Struct to represent a thread pool.
 * heap is the priority queue
 * rx is the receiver end of the channel to send TcpStreams through
 * tx is the transmitter end of the channel to send TcpStreams through
 * logger_tx is the transmitter end of the channel to send logging requests through
 * cache is the web server cache
 * cache_tx is the transmitter end of the channel to send caching requests through
 */
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
    /*
     * Given the number of each thread to spin up, spin up each thread and
     * return the new thread pool instance
     */
    pub fn new(scheduler_threads: usize,
               worker_threads: usize,
               logger_threads: usize,
               cache_threads: usize)
               -> ThreadPool {
        let heap = Arc::new(Mutex::new(BinaryHeap::<FileJob>::new()));
        let (tx, rx) = channel::<TcpStream>();
        let (logger_tx, logger_rx) = channel::<String>();
        let (cache_tx, cache_rx) = channel::<(String, Cache)>();

        // wrap receivers in Arc/Mutex
        let rx = Arc::new(Mutex::new(rx));
        let logger_rx = Arc::new(Mutex::new(logger_rx));
        let cache_rx = Arc::new(Mutex::new(cache_rx));

        // spin up logger threads
        for thread_id in 0..logger_threads {
            let thread_name = format!("logger_{}", thread_id);
            ThreadPool::spin_logger_thread(thread_name, logger_rx.clone());
        }

        // spin up scheduler threads
        for thread_id in 0..scheduler_threads {
            let thread_name = format!("scheduler_{}", thread_id);
            ThreadPool::spin_scheduler_threads(thread_name,
                                             rx.clone(),
                                             heap.clone(),
                                             logger_tx.clone());
        }

        // spin up cache threads
        let cache: Arc<ConcHashMap<String, Cache>> = Default::default();
        for thread_id in 0..cache_threads {
            let thread_name = format!("cache_{}", thread_id);
            ThreadPool::spin_cache_thread(thread_name, cache.clone(), cache_rx.clone());
        }

        // spin up worker threads
        for thread_id in 0..worker_threads {
            let thread_name = format!("worker_{}", thread_id);
            ThreadPool::spin_worker_threads(thread_name,
                                            heap.clone(),
                                            logger_tx.clone(),
                                            cache.clone(),
                                            cache_tx.clone());
        }

        // return the thread pool
        ThreadPool {
            heap: heap,
            rx: rx.clone(),
            tx: tx,
            logger_tx: logger_tx.clone(),
            cache: cache,
            cache_tx: cache_tx.clone(),
        }
    }

    /*
     * Spawn a new thread that will receive caching requests and update the cache
     * accordingly.
     */
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
                // get cache request from receiver. new scope created so lock
                // is only held to get the request and is released before
                // actually updating the cache
                let message = {
                    let obj_receiver = thread_stats.cache_rx.lock().unwrap();
                    obj_receiver.recv()
                };
                match message {
                    // if the message is valid, then insert the file contents into the cache
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

    /*
     * Spawn a new thread that will receive logging requests and write to the log
     * file accordingly.
     */
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

    /*
     * Routine for the logger thread that opens the log file and listens for
     * logging requests to write to the log file
     */
    fn logger(logger_rx: &mut Arc<Mutex<Receiver<String>>>) {
        let mut log_file = OpenOptions::new()
                               .create(true)
                               .write(true)
                               .append(true)
                               .open(LOGGER_FILE.to_string())
                               .unwrap();
        loop {
            // create new scope so that lock is only held to get
            // the logging request and not held while writing to
            // the log file
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

    /*
     * Spawn a new thread that will listen for TcpStreams from main thread,
     * read and parse the request, then add the FileJob to the priority queue.
     */
    fn spin_scheduler_threads(thread_name: String,
                            rx: Arc<Mutex<Receiver<TcpStream>>>,
                            heap: Arc<Mutex<BinaryHeap<FileJob>>>,
                            logger_tx: Sender<String>) {
        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            let mut thread_stats = SchedulerThreadStats {
                thread_name: thread_name,
                rx: rx,
                heap: heap,
                logger_tx: logger_tx,
            };
            loop {
                // listen for TcpStream from main thread. new scope created
                // so that lock isn't held while reading/parsing request from stream
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

    /*
     * Given TcpStream, read from stream, parse request, then push to priority queue
     */
    fn process_filejob(message: Result<TcpStream, RecvError>,
                       heap: &mut Arc<Mutex<BinaryHeap<FileJob>>>,
                       logger_tx: &mut Sender<String>,
                       thread_name: &str) {
        match message {
            Ok(stream) => {
                match FileJob::new(stream) {
                    Ok(job) => {
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
                    Err(_) => {}
                };
            }
            Err(e) => {
                println!("{:?}\tError in receiving filejob\t{:?}",
                         thread_name,
                         e.description());
            }
        };
    }
    /*
     * Send log statement request to logger thread
     */
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

    /*
     * Spawn a new thread that will take the job off of the priority queue with the
     * highest priority, then serve the appropriate file.
     */
    fn spin_worker_threads(thread_name: String,
                           heap: Arc<Mutex<BinaryHeap<FileJob>>>,
                           logger_tx: Sender<String>,
                           cache: Arc<ConcHashMap<String, Cache>>,
                           cache_tx: Sender<(String, Cache)>) {

        let result = thread::Builder::new().name(thread_name.clone()).spawn(move || {
            let mut thread_stats = WorkerThreadStats {
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
                                println!("Worker Logger Send Error{:?}", e.description());
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

    /*
     * Public function to allow user to send jobs to the thread pool for execution
     */
    pub fn execute(&self, stream: TcpStream) {
        // send stream to scheduler threads
        match self.tx.send(stream) {
            Ok(_) => {}
            Err(e) => {
                println!("Error occured while sending job {}\n", e.description());
            }
        }
    }
}
