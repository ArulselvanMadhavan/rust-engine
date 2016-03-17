use std::env;


// TODO:1. Implement this with memoization
// 2.Figure out how to do imports from other projects
fn main() {
    let args: Vec<String> = env::args().collect();
    let num: i32 = match args[1].trim().parse() {
        Ok(result) => result,
        Err(_) => return,
    };
    let mut counter = 1;
    loop {
        if num == collatz(counter) {
            println!("Found the lowest number:{}", counter);
            break;
        }
        println!("Current counter:{}", counter);
        counter += 1;
    }
}

fn collatz(mut i: i64) -> i32 {
    let mut counter: i32 = 0;
    while i != 1 {
        // println!("Current i:{}", i);
        if i % 2 == 1 {
            // i is odd
            i = 3 * i + 1;
        } else {
            i = i / 2;
        }
        counter += 1;
    }
    counter
}
