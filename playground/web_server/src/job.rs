use std::io::prelude::*;
use std::fmt;
use std::fs;
use std::net::TcpStream;
use std::error::Error;
use std::cmp::Ordering;
use request::Request;
use std::fs::File;
use chrono::UTC;
use std::sync::mpsc::{Sender, Receiver, channel};

// const ERROR_FILENAME: &'static str = "error.html";
const ERROR_FILESIZE: u64 = 0;
const BUFFER_SIZE: usize = 4096;

enum Status {
    Ok,
    BadRequest,
    NotFound,
}

impl Status {
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

struct StatusCode {
    name: String,
    response_code: u16,
}

#[derive(Debug)]
pub struct FileJob {
    pub stream: TcpStream,
    pub filesize: u64,
    pub request_obj: Request,
}

impl FileJob {
    #[allow(dead_code)]
    pub fn new(mut stream: TcpStream) -> FileJob {
        let request_obj = Request::new(&mut stream);
        match fs::metadata(request_obj.get_filename()) {
            Ok(file_metadata) => {
                FileJob {
                    stream: stream,
                    filesize: file_metadata.len(),
                    request_obj: request_obj,
                }
            }
            Err(e) => {
                println!("{:?}", e.description());
                FileJob {
                    stream: stream,
                    filesize: ERROR_FILESIZE,
                    request_obj: request_obj,
                }
            }
        }
    }

    pub fn new_test(mut stream: TcpStream, filesize: u64) -> FileJob {
        let request_obj = Request::new(&mut stream);
        FileJob {
            stream: stream,
            filesize: filesize,
            request_obj: request_obj,
        }
    }


    pub fn handle_client(&mut self)->String {
        let dt = UTC::now();
        let timestamp = dt.format("%Y-%m-%d %H:%M:%S").to_string();
        let request_str = self.request_obj.to_string();
        match File::open(self.request_obj.get_filename()) {
            Ok(mut f) => {
                // let mut content = String::new();
                // f.read_to_string(&mut content);

                let status = Status::get_info(Status::Ok);
                let response_header = format!("{} {}", status.response_code, status.name);
                // write response header to stream
                self.stream.write(format!("{} {}\n\n",
                                          self.request_obj.get_protocol(),
                                          response_header)
                                      .as_bytes());

                let mut read_buf = [0; BUFFER_SIZE];

                loop {
                    let bytes_read = f.read(&mut read_buf);
                    match bytes_read {
                        Ok(bytes_read) => {
                            self.stream.write(&read_buf);
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


                // let response_str = format!("{} {}\n\n{}", request_obj.get_protocol(), response_header, content);
                // stream.write(response_str.as_bytes());
                // let mut log: String = String::new();

                format!("{}\t{}\t{}\n", timestamp, request_str, response_header)
                // match logger_tx.send(log){
                //     Ok(_)=>{},
                //     Err(e)=>{println!("Error while sending to logger {:?}",e.description());}
                // }
            }
            Err(_) => {
                // write response header
                let status = Status::get_info(Status::NotFound);
                let response_header = format!("{} {}", status.response_code, status.name);
                self.stream.write(format!("{} {}\n\n",
                                          self.request_obj.get_protocol(),
                                          response_header)
                                  .as_bytes());

                let mut error_file = File::open("error.html").unwrap();
                let mut error_vec = Vec::new();
                match error_file.read_to_end(&mut error_vec) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error occured while error file {:?}", e.description());
                    }
                }
                let error_byte_array = error_vec.as_slice();
                match self.stream.write(error_byte_array) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error occured while writing response to stream{:?}",
                                 e.description());
                    }
                }
                format!("{}\t{}\t{}\n", timestamp, request_str, response_header)
            }

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
        write!(f, "{}\t{}", self.request_obj.get_filename(), self.filesize)
    }
}
