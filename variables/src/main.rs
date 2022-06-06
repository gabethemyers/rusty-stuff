fn main() {
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    

    let _a: [u32; 6] = [1, 2, 3, 5, 6, 7];  
    let tup: (i32, u128, char) = (-2, 599, '☭');

    let num = tup.0;
    let large_num = tup.1;
    let emoji = tup.2;

    println!("{}{:?}{}", num, _a, emoji);

}
