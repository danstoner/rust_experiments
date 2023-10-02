use std::{io, process::exit};


fn main() {

    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");





    println!("============");
    println!("");



    println!("Please input first number:");

    let mut line: String = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");

    let first: i8 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: Must type a number!");
            exit(1);
        }
    };

    line.clear();
    println!("Please input second number:");

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");

    let second: i8 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: Must type a number!");
            exit(1);
        }
    };

    println!("{first}, {second}");
    println!("Addition result: {}", add(first,second));
}


fn add (a: i8, b: i8) -> i8 {
    return a + b;
}