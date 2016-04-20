extern crate concurrent_hashmap;
extern crate rand;

use std::thread;
use std::default::Default;
use std::sync::{Arc, Mutex};
use concurrent_hashmap::*;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
struct Cache {
    key: String,
    value: u32,
}

fn get_cache_objects(num_objects: usize) -> Vec<String> {
    let mut cache_objects: Vec<String> = Vec::with_capacity(num_objects);
    let mut rng = rand::thread_rng();
    for _ in 0..num_objects {
        cache_objects.push(format!("Cache_object_{}", rng.gen::<u8>()));
    }
    cache_objects
}
fn main() {
    let num_objects: usize = 10;
    let num_iterations = 10000;
    let nthreads: usize = 4;
    let mut threads = Vec::with_capacity(nthreads);

    let cache_objects = Arc::new(get_cache_objects(num_objects));
    let cache: Arc<ConcHashMap<String, u32>> = Default::default();
    let (tx, rx) = channel::<Cache>();
    let rx = Arc::new(Mutex::new(rx));
    for _ in 0..nthreads {
        let rx = rx.clone();
        let cache_copy = cache.clone();
        threads.push(thread::spawn(move || {
            loop {
                let message = {
                    let job = rx.lock().unwrap();
                    job.recv()
                };
                match message {
                    Ok(job) => cache_copy.upsert(job.key.to_owned(), 1, &|count| *count += 1),
                    Err(e) => {
                        println!("{:?}", e.description());
                    }
                }
            }
        }))
    }

    let between = Range::new(0, num_objects);
    let mut rng = rand::thread_rng();
    for _ in 0..num_iterations {
        let element_id = between.ind_sample(&mut rng);
        let cache_obj = Cache {
            key: cache_objects[element_id].to_owned(),
            value: 0,
        };
        tx.send(cache_obj).unwrap();
    }

    // for _ in 0..nthreads{
    //     let cache_copy = cache.clone();
    //     let cache_objects_copy = cache_objects.clone();
    //     threads.push(thread::spawn(move || {
    //         let between = Range::new(0, num_objects);
    //         let mut rng = rand::thread_rng();
    //         for _ in 0..num_iterations {
    //             let element_id = between.ind_sample(&mut rng);
    //             cache_copy.upsert(cache_objects_copy[element_id].to_owned(),
    //                          1,
    //                          &|count| *count += 1);
    //         }
    //     }));
    // }

    // let val = cache.find(&cache_objects[0]);
    // match val {
    //     Some(acc) => {
    //         println!("{:?}", acc.get());
    //     }
    //     None => {
    //         println!("Element not found");
    //     }
    // }
    // for thread in threads {
    //     thread.join().unwrap();
    // }

    sleep(Duration::new(5,0));ss
    for cache_item in cache.iter() {
        println!("{:?}", cache_item);
    }
}
