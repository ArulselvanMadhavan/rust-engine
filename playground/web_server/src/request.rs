use std::net::TcpStream;
use std::io::prelude::*;
use std::str;
use std::path::PathBuf;
use std::env;

const BUFFER_SIZE: usize = 20;

pub struct Request {
    method: String,
    filename: String,
    host: String
}

impl Request {

    pub fn new(stream: &mut TcpStream) -> Request {
        let mut request = String::new();

        let mut read_buf = [0; BUFFER_SIZE];

        loop {
            let bytes_read = stream.read(&mut read_buf);
            match bytes_read {
                Ok(bytes_read) => {
                    let string_result = str::from_utf8(&read_buf).unwrap();
                    request.push_str(string_result);
                    if bytes_read < BUFFER_SIZE {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error reading stream");
                    break;
                }
            };
        }

        Request::create_obj("GET".to_string(), Request::get_path_from_request(&request[..]).to_string(),
        "localhost:8080".to_string())
    }

    fn create_obj(method: String, filename: String, host: String) -> Request {
        Request {
            method: method,
            filename: filename,
            host: host
        }
    }

    pub fn get_method(&self) -> &String {
        &self.method
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn get_host(&self) -> &String {
        &self.host
    }

    fn get_path_from_request(request: &str) -> &str {
        let mut abs_path = PathBuf::new();
        let curr_dir = env::current_dir().unwrap();
        abs_path.push(curr_dir);
        println!("{}", abs_path.display());
        let mut iter = request.split_whitespace();
        iter.next();
        let requested_path = iter.next().unwrap();
        abs_path.push(requested_path);
        println!("{}", abs_path.display());
        abs_path;
        // TODO: Rewrite to avoid creating new string
        &requested_path[1..]
    }

}

