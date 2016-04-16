use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;
use std::error::Error;
use std::fs::metadata;

const ERROR_FILENAME: &'static str = "error.html";

struct FileJob {
    stream: TcpStream,
    //file: File,
    filesize: u64,
    request_obj: Request
}

impl FileJob {
    pub fn new(stream:TcpStream) -> FileJob {
        // let file = File::open(filename).unwrap();
        // let mut meta = file.metadata().unwrap();
        // let mut filesize = meta.len();
        let request_obj = Request::new(&mut stream);
        let file_metadata = match fs::metadata(request_obj.get_filename()) {
            Ok(file_metadata) => {
                file_metadata
            }
            Err(e) => {
                println!("{:?}", e.description());
                fs::metadata(ERROR_FILENAME)
            }
        }
        // let file = match File::open(request_obj.get_filename()) {
        //     Ok(mut file) => {
        //         file
        //     }
        //     Err(e) => {
        //         println!("{:?}", e.description());
        //         File::open(ERROR_FILENAME).unwrap()
        //     }
        // };
        // let file_metadata = file.metadata()
        FileJob {
            //file: file,
            filename: request_obj.get_filename(),
            filesize: file_metadata.len(),
            request_obj: request_obj
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
