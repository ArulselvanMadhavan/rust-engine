/*use std::net::{TcpStream};
use std::io::prelude::*;
use std::str;
use std::path::PathBuf;
use std::env;

struct Request {
    method: String,
    filename: String,
    host: String
}

impl Request {
    fn new(&self, stream: TcpStream) -> Result<Request, &str> {

        let request_str: &str;
        match read_from_stream(stream) {
            Ok(output_str) => {
                request_str = output_str;

                let method_val = "GET".to_string();
                let filename_val = get_path_from_request(request_str).to_string();
                let host_val = "localhost:8080".to_string();

                Ok(Request {
                    method: method_val,
                    filename: filename_val,
                    host: host_val
                })
            }
            Err(e) => {
                // TODO: Actually send response back with 400 Bad Request status
                println!("400 Bad Request");
                Err("400 Bad Request")   
            }
        }
    }
}


fn read_from_stream(mut stream: TcpStream) -> Result<&str, & str> {
    // TODO: Content length should be inferred from the request, not hardcoded
    let mut contents = [0; 1024];
    // read request into buffer
    let result = stream.read(&mut contents);
    //let string_result = str::from_utf8(&contents).unwrap();

    match result {
        Ok(result) => {
            let string_result = str::from_utf8(&contents).unwrap();
            Ok(string_result)
        }
        Err(e) => {
            // TODO: Better error handling
            println!("Error when reading request");
            println!("{:?}", e);
            Err("Error")
        }
    }
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

*/
