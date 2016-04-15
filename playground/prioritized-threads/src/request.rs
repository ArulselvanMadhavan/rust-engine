use std::io::prelude::*;
use std::net::TcpStream;

struct Request{
    file_size:u64,
    file_path:String,
    stream:TcpStream
}

impl Request{
    pub fn new(stream:&mut TcpStream) -> Request{
        
    }
}
