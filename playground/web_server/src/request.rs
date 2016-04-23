use std::net::TcpStream;
use std::io::prelude::*;
use std::str;
use std::env;
use std::collections::HashMap;
use std::error::Error;
use error::{RequestError,RequestResult,RequestErrorKind};

const BUFFER_SIZE: usize = 20;

#[derive(Debug)]
pub struct Request {
    method: String,
    filename: String,
    protocol: String,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> RequestResult {
        let mut request = String::new();

        let mut read_buf = [0; BUFFER_SIZE];

        loop {
            let bytes_read = stream.read(&mut read_buf);
            match bytes_read {
                Ok(bytes_read) => {
                    match str::from_utf8(&read_buf) {
                        Ok(string_result) => {
                            request.push_str(string_result);
                            if bytes_read < BUFFER_SIZE {
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Error converting bytes to string: {:?}", e.description());
                        }
                    }
                }
                Err(e) => {
                    println!("Error reading stream {}", e.description());
                    break;
                }
            };
        }
        Request::parse_request(&request[..])
    }

    #[allow(dead_code)]
    pub fn get_method(&self) -> &String {
        &self.method
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    #[allow(dead_code)]
    pub fn get_protocol(&self) -> &String {
        &self.protocol
    }

    #[allow(dead_code)]
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn parse_request(request: &str) -> RequestResult {
        // hashmap to store header values
        let mut header_map: HashMap<String, String> = HashMap::new();
        // get current directory path
        // let curr_dir = env::current_dir().unwrap();
        let curr_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("Error getting current directory: {:?}", e.description());
                panic!();
            }
        };
        // split request by line
        let mut line_split = request.split("\n");

        // parse the method, path, and protocol from the first line of the request
        // TODO: error handling, what if there is a space in file path?
        // let mut first_line = line_split.next().unwrap().split_whitespace();
        let mut first_line = match line_split.next() {
            Some(line) => line.split_whitespace(),
            None => {
                panic!("Request is empty");
            }
        };

        let method = first_line.next().unwrap().to_string();

        // let rel_path = first_line.next().unwrap();
        let rel_path = match first_line.next() {
            Some(path) => path,
            None => {
                return Err(RequestError::new("Request is empty".to_string(), RequestErrorKind::EmptyRequest))
            }
        };
        let filename = curr_dir.display().to_string() + &rel_path.to_string();

        // let protocol = first_line.next().unwrap().to_string();
        let protocol = match first_line.next() {
            Some(prot) => prot.to_string(),
            None => {
                panic!("No protocol was provided in request");
            }
        };

        // split each line by ":" to find the header name and header values.
        // if there are colons in header value, then we need to concatenate
        // the pieces again
        for line in line_split {
            let mut colon_split = line.split(":");
            // let key = colon_split.next().unwrap().to_string();
            let key = match colon_split.next() {
                Some(k) => k.to_string(),
                None => {
                    panic!("Missing header name");
                }
            };
            // don't insert into the hashmap if this is a blank line
            if key.trim().len() == 0 {
                continue;
            }
            let mut value = String::new();
            for s in colon_split {
                value = value + &s.trim().to_string();
                value = value + ":";
            }
            // remove last ":"
            value.pop();
            header_map.insert(key, value);
        }
        Ok(Request {
            method: method,
            filename: filename,
            protocol: protocol,
            headers: header_map,
        })
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{} {} {}", &self.method, &self.filename, &self.protocol)
    }
}
