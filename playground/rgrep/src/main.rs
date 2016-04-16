use std::io::prelude::*;
use std::{io, fs, thread};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::Arc;

#[derive(Clone,Copy)]
pub enum OutputMode {
    Print,
    SortAndPrint,
    Count,
}

use self::OutputMode::*;

pub struct Options {
    pub files: Vec<String>,
    pub pattern: String,
    pub output_mode: OutputMode,
}

// The first function reads the files, and sends every line over the `out_channel`.
fn read_files(options: Arc<Options>, out_channel: SyncSender<String>) {
    for file in options.files.iter() {
        // First, we open the file, ignoring any errors.
        let file = fs::File::open(file).unwrap();
        // Then we obtain a `BufReader` for it, which provides the `lines` function.
        let file = io::BufReader::new(file);
        for line in file.lines() {
            let line = line.unwrap();
            // Now we send the line over the channel, ignoring the possibility of `send` failing.
            out_channel.send(line).unwrap();
        }
    }
    // When we drop the `out_channel`, it will be closed, which the other end can notice.
}

fn filter_lines(options: Arc<Options>,
                in_channel: Receiver<String>,
                out_channel: SyncSender<String>) {
    for line in in_channel.iter() {
        if line.contains(&options.pattern) {
            out_channel.send(line).unwrap();
        }
    }
}


fn output_lines(options: Arc<Options>, in_channel: Receiver<String>) {
    match options.output_mode {
        Print => {
            for line in in_channel.iter() {
                println!("{:?}", line);
            }
        }
        Count => {
            let count = in_channel.iter().count();
            println!("{:?} hits for count {:?}", count, options.pattern);
        }
        SortAndPrint => {
            let mut data: Vec<String> = in_channel.iter().collect();
            unimplemented!()
        }
    }
}

pub fn run(options: Options) {
    let options = Arc::new(options);
    let (line_sender, line_receiver) = sync_channel::<String>(16);
    let (filtered_sender, filtered_receiver) = sync_channel::<String>(16);
    let options1 = options.clone();
    let handle1 = thread::spawn(move || read_files(options1, line_sender));
    let options2 = options.clone();
    let handle2 = thread::spawn(move || filter_lines(options2, line_receiver, filtered_sender));
    let options3 = options.clone();
    let handle3 = thread::spawn(move || output_lines(options3, filtered_receiver));
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}

pub fn main(){
    let options = Options{
        files:vec!["src/part10.rs".to_string(),
        "src/part11.rs".to_string(),
        "src/part12.rs".to_string()],
        pattern:"let".to_string(),
        output_mode:Count
    };
    run(options);
}
