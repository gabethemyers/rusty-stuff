use std::io;

fn c_to_f(input: i32) -> f64 {
    
    let ret = (input as f64 * (9.0/5.0)) + 32.0;
    ret
}

fn f_to_c(input: i32) -> f64 {
    let ret = (input as f64 - 32.0) * (5.0/9.0);
    ret
}

fn main() {
    let mut c_or_f = String::new();
    
    println!("Enter 0 for celcius to farenheit and 1 for farenheit to celcius");

    io::stdin()
        .read_line(&mut c_or_f)
        .expect("Failed to read line");

    let c_or_f: i32 = c_or_f.trim().parse().expect("Not a number lol");
    

    
    let mut degrees = String::new();
    
    println!("Enter the degrees");

    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");

    let degrees: i32 = degrees.trim().parse().expect("Not a number lol");

    if c_or_f == 0 {
        println!("{}c is {}f", degrees, c_to_f(degrees));
    } else {
        println!("{}f is {}c", degrees, f_to_c(degrees));
    }

}

