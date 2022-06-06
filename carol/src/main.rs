fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
    "seventh", "eightth", "nineth", "tenth", "eleventh", "twelveth"];

    let list = ["And a partridge in a pear tree", "Two turtle-doves", "Three french hens", "Four calling birds", 
    "Five golden rings", "Six gees a-laying", "Seven swans a-swimming", "Eight maid a-milking", "Nine ladies dancing", 
    "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    
    let mut x = 0;


    while x < 12{
        println!();
        println!("On the {} day of christmas\nMy true love sent to me", days[x]);

        if x == 0 {
            println!("A partrige in a pear tree")
        } else{ 
            for i in (0..=x).rev(){
                println!("{}", list[i]);
            }
        }
        x += 1
    }
}
