use std::io;

fn main() {
	loop {
    println!("Enter a number:");
    let mut num = String::new();

    io::stdin().read_line(&mut num)
    .expect("A number is expected");

    let num:i32 = match num.trim().parse(){
    	Ok(val)=>val,
    	Err(_)=> {
    		println!("Invalid number");
    		continue
    	}
    };

    if num % 15 == 0{
    	println!("fizzbuzz");
    }
    else if num % 3 == 0{
    	println!("fizz");
    }
    else if num % 5 == 0{
    	println!("buzz");
    }
    else {
        println!("{}",num );
    }
    // match num {
    //     (num % 15 == 0) => println!("fizzbuzz"),
    //     (num % 3 == 0) => println!("fizz"),
    //     (num % 5 == 0) => println!("buzz"),
    // };
}
}
