use std::io::prelude::*;
use std::fmt;
use std::fs;
use std::net::TcpStream;
use std::error::Error;
use std::cmp::Ordering;
use request::Request;

// const ERROR_FILENAME: &'static str = "error.html";
const ERROR_FILESIZE: u64 = 0;

#[derive(Debug)]
pub struct FileJob {
    stream: TcpStream,
    filesize: u64,
    request_obj: Request,
}

impl FileJob {
    #[allow(dead_code)]
    pub fn new(mut stream: TcpStream) -> FileJob {
        let request_obj = Request::new(&mut stream);
        match fs::metadata(request_obj.get_filename()) {
            Ok(file_metadata) => {
                FileJob {
                    stream:stream,
                    filesize: file_metadata.len(),
                    request_obj: request_obj,
                }
            }
            Err(e) => {
                println!("{:?}", e.description());
                FileJob {
                    stream:stream,
                    filesize: ERROR_FILESIZE,
                    request_obj: request_obj,
                }
            }
        }
    }

    pub fn new_test(mut stream: TcpStream, filesize: u64) -> FileJob {
        let request_obj = Request::new(&mut stream);
        FileJob {
            stream:stream,
            filesize: filesize,
            request_obj: request_obj,
        }
    }
}


impl Ord for FileJob {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.filesize < other.filesize {
            Ordering::Greater
        } else if self.filesize > other.filesize {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for FileJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FileJob {
    fn eq(&self, other: &Self) -> bool {
        self.filesize == other.filesize
    }
}

impl Eq for FileJob {}


impl fmt::Display for FileJob {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}\t{}", self.request_obj.get_filename(),self.filesize)
    }
}
