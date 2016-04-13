use std::io::prelude::*;
use std::fs::File;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::net::TcpStream;
use std::fs::Metadata;
#[derive(Debug)]
struct FileJob {
    // stream: TcpStream,
    file: File,
    filesize: u64,
}

impl FileJob {
    pub fn new(filename: &str) -> FileJob {
        let file = File::open(filename).unwrap();
        let mut meta = file.metadata().unwrap();
        let mut filesize = meta.len();
        FileJob {
            file: file,
            filesize: filesize,
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

fn main() {
    let filejob1 = FileJob::new("file1.txt");
    let filejob2 = FileJob::new("file2.txt");
    let filejob3 = FileJob::new("file3.txt");
    assert_eq!(filejob1.cmp(&filejob2), Ordering::Less);
    assert_eq!(filejob2.cmp(&filejob1), Ordering::Greater);
    assert_eq!(filejob2.cmp(&filejob3), Ordering::Equal);

    let mut heap = BinaryHeap::new();
    heap.push(filejob1);
    heap.push(filejob2);
    let topjob = heap.peek();
    println!("{:?}", topjob);
}
