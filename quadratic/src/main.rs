use std::io;

fn quadratic (a: i32, b: i32, c:i32) -> (f64, f64){
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;
    
    let x: f64 = ((-1.0 * b) + ((b * b) - (4.0 * a * c)).sqrt())/(2.0 * a);
    let y: f64 = ((-1.0 * b) - ((b * b) - (4.0 * a * c)).sqrt())/(2.0 * a);
    

    (x, y)

}

fn main() {
    let mut inp = String::new();
    
    println!("Enter coefficents seperated by spaces (a b c)");

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line.");
    
    let inp = inp.trim();
    
    let list: [&str; 3] = [&inp[0..1], &inp[2..3], &inp[4..5]];

    let a: i32 = list[0].parse().expect("not a number");
    let b: i32 = list[1].parse().expect("not a number");
    let c: i32 = list[2].parse().expect("not a number");

    //println!("{:?}", quadratic(a, b, c));
    println!("{:?}", quadratic(a, b, c));
}
