use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Program requires an argument");
        println!("{}", args.len());
        return;
    }
    let num: i64 = match args[1].trim().parse() {
        Ok(result) => result,
        Err(_) => return,
    };

    println!("{} has {} Collatz steps", num, collatz(num));
}

fn collatz(mut i: i64) -> i32 {
    let mut counter: i32 = 0;
    while i != 1 {
        println!("Current i:{}", i);
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
