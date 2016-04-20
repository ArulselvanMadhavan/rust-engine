extern crate concurrent_hashmap;
extern crate rand;
extern crate chrono;

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
use chrono::UTC;
use chrono::datetime::DateTime;

const CACHE_THRESHOLD: u8 = 50;

#[derive(Debug)]
struct Cache {
    data: String,
    last_modified: DateTime<UTC>,
}

fn get_cache_objects(num_objects: usize) -> Vec<(String)> {
    let mut cache_objects: Vec<String> = Vec::with_capacity(num_objects);
    let mut rng = rand::thread_rng();
    for _ in 0..num_objects {
        let file_size = rng.gen::<u8>();
        let key = format!("File_PATH_{}", file_size);
        cache_objects.push(key);
    }
    cache_objects
}
fn main() {
    let num_objects: usize = 10;
    let num_iterations = 20;
    let nthreads: usize = 4;
    let mut threads = Vec::with_capacity(nthreads);

    let cache_objects = Arc::new(get_cache_objects(num_objects));
    let cache: Arc<ConcHashMap<String, Cache>> = Default::default();
    let (tx, rx) = channel::<usize>();
    let rx = Arc::new(Mutex::new(rx));
    // TODO-Spawn threads in separate method
    for _ in 0..nthreads {
        let rx = rx.clone();
        let cache_copy = cache.clone();
        let cache_objects_copy = cache_objects.clone();
        threads.push(thread::spawn(move || {
            loop {
                let message = {
                    let job = rx.lock().unwrap();
                    job.recv()
                };
                match message {
                    Ok(id) => {
                        let ref obj = cache_objects_copy[id];
                        let key = obj.to_owned();

                        // let (key, cache_obj) = obj.to_owned();
                        let val = cache_copy.find(&key);
                        match val {
                            Some(acc) => {
                                println!("Reading the {:?}\t{:?}",
                                         acc.get().data,
                                         acc.get().last_modified);
                            }
                            None => {
                                println!("Element not found");
                                // Open a file
                                // Create a cache object
                                // Save it in the cache(hash map)
                                let cache_obj = Cache {
                                    data: format!("Data in {}", key),
                                    last_modified: UTC::now(),
                                };
                                cache_copy.insert(key, cache_obj);
                            }
                        }
                    }
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
        let element_id: usize = between.ind_sample(&mut rng);
        tx.send(element_id).unwrap();
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

    sleep(Duration::new(10, 0));
    println!("Sleep ends");
    for cache_item in cache.iter() {
        println!("{:?}", cache_item);
    }
}
