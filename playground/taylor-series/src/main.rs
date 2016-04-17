extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::sync::mpsc::{channel, Sender, Receiver};

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::error::Error;

const USAGE: &'static str = "
Taylor Series Calculator.

Usage:
  ./hw4 <x> <n> <threads>

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_x: u32,
    arg_n: u32,
    arg_threads: u8,
}
fn factorial_iterative(n: u64) -> u64 {
    (1..n + 1).fold(1, |p, n| p * n)
}

fn init_threads(num_threads: u8, x: u32) -> (Sender<f64>, Receiver<f64>) {
    let (result_tx, result_rx) = channel::<f64>();
    let (input_tx, input_rx) = channel::<f64>();
    let input_rx: Arc<Mutex<Receiver<f64>>> = Arc::new(Mutex::new(input_rx));
    for _ in 0..num_threads {
        let data_rx = input_rx.clone();
        let output_tx = result_tx.clone();
        thread::spawn(move || {
            loop {
                let message = {
                    let job_receiver = data_rx.lock().unwrap();
                    job_receiver.recv()
                };
                match message {
                    Ok(data) => {
                        let x: f64 = x as f64;
                        let num = x.powi(data as i32);
                        let denom: u64 = factorial_iterative(data as u64);
                        output_tx.send(num / denom as f64).unwrap()
                    }
                    Err(e) => {
                        println!("{:?}", e.description());
                    }
                }
            }
        });
    }
    (input_tx, result_rx)
}
fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    println!("{:?}\t{:?}\t{:?}", args.arg_x, args.arg_n, args.arg_threads);
    let (input_tx, result_rx) = init_threads(args.arg_threads, args.arg_x);
    for n in 0..args.arg_n + 1 {
        input_tx.send(n as f64).unwrap();
    }
    println!("{:?}",
             result_rx.iter().take((args.arg_n + 1) as usize).fold(0.0, |a, b| a + b));
}
