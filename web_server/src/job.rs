use std::io::prelude::*;
use std::fmt;
use std::fs;
use std::net::TcpStream;
use std::error::Error;
use std::cmp::Ordering;
use request::Request;
use std::fs::File;
use chrono::UTC;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use concurrent_hashmap::*;
use threadmanager::Cache;
use error::{FileJobError,FileJobErrorKind,FileJobResult};

const ERROR_FILESIZE: u64 = 0;
const BUFFER_SIZE: usize = 4096;
const CACHE_THRESHOLD: u64 = 50000;

/* Struct for the a response status code, consisting of a name such as OK
 * and a response_code such as 200
 */
struct StatusCode {
    name: String,
    response_code: u16,
}

/* Response status types */
enum Status {
    Ok,
    BadRequest,
    NotFound,
}

impl Status {
    /*
     * Given a response Status enum, return the appropriate StatusCode struct
     */
    fn get_info(status: Status) -> StatusCode {
        match status {
            Status::Ok => {
                StatusCode {
                    name: "OK".to_string(),
                    response_code: 200,
                }
            }
            Status::BadRequest => {
                StatusCode {
                    name: "BAD REQUEST".to_string(),
                    response_code: 400,
                }
            }
            Status::NotFound => {
                StatusCode {
                    name: "NOT FOUND".to_string(),
                    response_code: 404,
                }
            }
        }
    }
}

/*
 * A representation of a task for a worker thread.
 * The HTTP request can be
 * read from the stream, and the HTTP response and file contents should be
 * written to the stream.
 * The filesize should be the size of the file
 * in bytes of the requested file. This also determines the priority of the
 * FileJob in the priority queue.
 * The request_obj will hold the information about the HTTP request once it
 * is read from the stream.
 */
#[derive(Debug)]
pub struct FileJob {
    pub stream: TcpStream,
    pub filesize: u64,
    pub request_obj: Request,
}

impl FileJob {
    /*
     * Read the request from the given stream and return a new FileJob
     */
    #[allow(dead_code)]
    pub fn new(mut stream: TcpStream) -> FileJobResult {
        match Request::new(&mut stream) {
            Ok(request) => {
                // get metadata to get file size without opening file
                match fs::metadata(request.get_filename()) {
                    Ok(file_metadata) => {
                        Ok(FileJob {
                            stream: stream,
                            filesize: file_metadata.len(),
                            request_obj: request,
                        })
                    }
                    Err(e) => {
                        // Error happens when file doesn't exist. This will result in 404
                        println!("{:?}", e.description());
                        Ok(FileJob {
                            stream: stream,
                            filesize: ERROR_FILESIZE,
                            request_obj: request
                        })
                    }
                }
            }
            Err(_) => {
                // This occurs usually when request is empty, although can also happen if request
                // is invalid
                FileJob::send_bad_request_error(stream);
                Err(FileJobError::new("Empty FileJob".to_string(), FileJobErrorKind::EmptyFileJob))
            }
        }
    }

    /*
     * Request has been deemed invalid so send BadRequest error back to client
     */
    pub fn send_bad_request_error(mut stream: TcpStream) -> String {
        // get current time for log statement
        let dt = UTC::now();
        let timestamp = dt.format("%Y-%m-%d %H:%M:%S").to_string();
        // get BadRequest struct info
        let status = Status::get_info(Status::BadRequest);
        // generate response header for BadRequest error
        let response_header = format!("{} {}", status.response_code, status.name);
        // write response to client
        match stream.write(format!("{} {}\n\n", "HTTP/1.1", response_header).as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error writing bad request response: {:?}", e.description());
            }
        };
        format!("{}\t{}\t{}\n", timestamp, "EMPTY REQUEST", response_header)
    }


    /*
     * Serve the file specified by this FileJob, using the given cache if possible.
     * If the file is not in the cache and smaller than the cache threshold, then
     * use the given cache thread transmitter to send a caching request to the
     * cache thead.
     * Return the log statement that will be sent to the logger thread.
     */
    pub fn handle_client_with_cache(&mut self,
                                    cache: &mut Arc<ConcHashMap<String, Cache>>,
                                    cache_tx: &mut Sender<(String, Cache)>)
                                    -> String {
        // get the current time for the log statement
        let dt = UTC::now();
        let timestamp = dt.format("%Y-%m-%d %H:%M:%S").to_string();
        // get the HTTP request for the log statement
        let request_str = self.request_obj.to_string();
        // check if the file for this FileJob is in the cache
        let acc = cache.find(self.request_obj.get_filename());
        match acc {
            Some(acc) => {
                // file is in the cache, serve the file with a status 200 OK
                let status = Status::get_info(Status::Ok);
                let response_header = format!("{} {}", status.response_code, status.name);
                // write the response status to the stream
                self.stream.write(format!("{} {}\n\n",
                                          self.request_obj.get_protocol(),
                                          response_header)
                                      .as_bytes());
                // write the cached file contents to the stream
                self.stream.write(&acc.get().data[..]);
                // Return the log statement containing the time, request, and response status
                format!("{}\t{}\t{}\n", timestamp, request_str, response_header)
            }
            None => {
                // file is not in the cache, so try to open the file
                match File::open(self.request_obj.get_filename()) {
                    Ok(mut f) => {
                        // file exists, so serve file with 200 OK status
                        let status = Status::get_info(Status::Ok);
                        let response_header = format!("{} {}", status.response_code, status.name);
                        self.stream.write(format!("{} {}\n\n",
                                                  self.request_obj.get_protocol(),
                                                  response_header)
                                              .as_bytes());

                        if self.filesize < CACHE_THRESHOLD {
                            // file is small enough so cache file contents
                            let mut file_contents = Vec::with_capacity(self.filesize as usize);
                            match f.read_to_end(&mut file_contents) {
                                Ok(_) => {
                                    self.stream.write(&file_contents);
                                    let cache_obj = Cache { data: file_contents };
                                    // Send this to a cache thread and have it write to the cache
                                    cache_tx.send((self.request_obj.get_filename().to_owned(),cache_obj));
                                }
                                Err(e) => {
                                    println!("Error reading file: {:?}", e.description());
                                }
                            }
                        } else {
                            // file is too big to cache so stream file contents to client
                            let mut read_buf = [0; BUFFER_SIZE];

                            loop {
                                let bytes_read = f.read(&mut read_buf);
                                match bytes_read {
                                    Ok(bytes_read) => {
                                        self.stream.write(&read_buf);
                                        // stop writing to stream once all the file contents have
                                        // been written
                                        if bytes_read < BUFFER_SIZE {
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error reading file contents {}", e.description());
                                        break;
                                    }
                                };
                            }
                        }
                        // Return the log statement
                        format!("{}\t{}\t{}\n", timestamp, request_str, response_header)
                    }
                    Err(_) => {
                        // file was not found, so send 404
                        // get NotFound response status info
                        let status = Status::get_info(Status::NotFound);
                        let response_header = format!("{} {}", status.response_code, status.name);
                        // write response header
                        self.stream.write(format!("{} {}\n\n",
                                                  self.request_obj.get_protocol(),
                                                  response_header)
                                              .as_bytes());
                        // open 404 error file
                        let mut error_file = File::open("error.html").unwrap();
                        let mut error_vec = Vec::new();
                        match error_file.read_to_end(&mut error_vec) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error occured while error file {:?}", e.description());
                            }
                        }
                        let error_byte_array = error_vec.as_slice();
                        // write error file contents
                        match self.stream.write(error_byte_array) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error occured while writing response to stream{:?}",
                                         e.description());
                            }
                        }
                        // Return log statement
                        format!("{}\t{}\t{}\n", timestamp, request_str, response_header)
                    }
                }
            }
        }
    }
}


/*
 * Implement the Ord trait for the FileJob struct such that
 * smaller files have a higher value, resulting in a higher
 * priority in the priority queue
 */
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
        write!(f, "{}\t{}", self.request_obj.get_filename(), self.filesize)
    }
}
