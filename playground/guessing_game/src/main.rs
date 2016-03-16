extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess Game");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret Number if {}", secret_number);

    loop {

    println!("Input a number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read a line from the user");

    let guess:u32 = match guess.trim().parse() {
    	Ok(num)=>num,
    	Err(_)=>continue,
    };

    println!("You guessed {}",guess);

    match guess.cmp(&secret_number) {
    	Ordering::Less=>println!("Too small"),
    	Ordering::Greater=>println!("Too big"),
    	Ordering::Equal=>{
    		println!("Match found");
    		break;
    	}
    }
}
}
