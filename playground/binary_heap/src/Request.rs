use std::io::prelude::*;
use std::fs::File;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct FileJob {
    // stream: TcpStream,
    // file: File,
    filename: String,
    filesize: u8,
}

impl FileJob {
    pub fn new(file_name: String, file_size: u8) -> FileJob {
        // let file = File::open(filename).unwrap();
        // let mut meta = file.metadata().unwrap();
        // let mut filesize = meta.len();
        FileJob {
            // file: file,
            filename: file_name,
            filesize: file_size,
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
