/*extern crate threadpool;

use std::io::prelude::*;
use std::fs::File;
use threadpool::ThreadPool;

pub struct LogManager {
    log_filename: String,
    log_file_handle: File,
    log_threadpool: ThreadPool
}

impl LogManager {
    
    pub fn new(log_filename: String) -> LogManager {

        let file_handle = File::open(log_filename).unwrap(); //try!(File::open(log_filename));
        let pool = ThreadPool::new(1);

        LogManager {
            log_filename: log_filename,
            log_file_handle: file_handle,
            log_threadpool: pool
        }
    }

    pub fn log(&self, message: String) {
        self.log_threadpool.execute(move|| {
            self.log_file_handle.write(message.as_bytes());
        });
    }
}*/
