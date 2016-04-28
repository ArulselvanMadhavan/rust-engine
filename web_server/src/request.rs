use std::net::TcpStream;
use std::io::prelude::*;
use std::str;
use std::env;
use std::collections::HashMap;
use std::error::Error;
use error::{RequestError,RequestResult,RequestErrorKind};

const BUFFER_SIZE: usize = 20;

/*
 * A struct representing an HTTP request.
 * method is the HTTP method, ie GET, POST, PUT, DELETE, etc.
 * filename is the path to the requested file
 * protocol is the protocol, ie HTTP/1.1
 * headers is a mapping from HTTP header names to values
 */
#[derive(Debug)]
pub struct Request {
    method: String,
    filename: String,
    protocol: String,
    headers: HashMap<String, String>,
}

impl Request {
    /*
     * Read the HTTP request from the given TcpStream and Result, being the
     * new Request struct if the request is valid and a RequestError if the
     * request is invalid
     */
    pub fn new(stream: &mut TcpStream) -> RequestResult {

        let mut request = String::new();
        let mut read_buf = [0; BUFFER_SIZE];

        // read entire HTTP request into request string
        loop {
            let bytes_read = stream.read(&mut read_buf);
            match bytes_read {
                Ok(bytes_read) => {
                    match str::from_utf8(&read_buf) {
                        Ok(string_result) => {
                            request.push_str(string_result);
                            // stop reading when no more to read from stream
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
        // pass request string to be parsed into Request struct and return result
        Request::parse_request(&request[..])
    }

    // getter for the Request struct's method
    #[allow(dead_code)]
    pub fn get_method(&self) -> &String {
        &self.method
    }

    // getter for the Request struct's path to file
    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    // getter for the Request struct's protocol
    #[allow(dead_code)]
    pub fn get_protocol(&self) -> &String {
        &self.protocol
    }

    // getter for the Request struct's mapping of request header values
    #[allow(dead_code)]
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /*
     * Given a string containing an HTTP request, parse out the method,
     * path to requested file, protocol, and headers. Return a Request
     * struct populated with the information if the request is valid, and
     * return a ResultError otherwise.
     */
    fn parse_request(request: &str) -> RequestResult {
        // hashmap to store header values
        let mut header_map: HashMap<String, String> = HashMap::new();
        
        // split request by line
        let mut line_split = request.split("\n");

        // parse the method, path, and protocol from the first line of the request
        let mut first_line = match line_split.next() {
            Some(line) => line.split_whitespace(),
            None => {
                return Err(RequestError::new("Request is empty".to_string(), RequestErrorKind::EmptyRequest))
            }
        };
        
        let method = first_line.next().unwrap().to_string();
        
        // get current directory path
        let curr_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("Error getting current directory: {:?}", e.description());
                panic!();
            }
        };

        let rel_path = match first_line.next() {
            Some(path) => path,
            None => {
                // return a RequestError since request isn't valid
                return Err(RequestError::new("Request is empty".to_string(), RequestErrorKind::EmptyRequest))
            }
        };

        let filename = curr_dir.display().to_string() + &rel_path.to_string();

        let protocol = match first_line.next() {
            Some(prot) => prot.to_string(),
            None => {
                return Err(RequestError::new("Request is empty".to_string(), RequestErrorKind::EmptyRequest))
            }
        };

        // split each line by ":" to find the header name and header values.
        // if there are colons in header value, then we need to concatenate
        // the pieces again
        for line in line_split {
            let mut colon_split = line.split(":");
            let key = match colon_split.next() {
                Some(k) => k.to_string(),
                None => {
                    return Err(RequestError::new("Request is empty".to_string(), RequestErrorKind::EmptyRequest))
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
        // return okay request
        Ok(Request {
            method: method,
            filename: filename,
            protocol: protocol,
            headers: header_map,
        })
    }

    // return the request method, path to file, and protocol as a string
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{} {} {}", &self.method, &self.filename, &self.protocol)
    }
}
