use std::io;

fn main() {

    // all of this is just input from command line into a number
    let mut number = String::new();

    println!("Enter how many fibonacci numbers you would like.");

    io::stdin()
        .read_line(&mut number)
        .expect("Could not read line");

    let number: u32 = number.trim().parse().expect("not a number");
    let number = number + 1;

    // setting up varibles for loop
    let mut x: u32 = 0;
    let mut y = 1;
    let mut temp;

    for i in 0..number {
        // need first 2 numbers to be 0 and 1 so i had to do it like this
        match i {
            0 => println!(),
            1 => println!("0"),
            2 => println!("1"),
            // code for the actual generation of fibonacci numbers
            _ => {temp = y;
                y = x + y;
                x = temp;
                println!("{}", y)}
        }
    };
}
