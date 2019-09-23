//use std::io;

fn main() {
    let i:i8 = 7;
    let j:i8 = 8;

    println!("{} + {} = {}", i, j, i+j);

    let c = 'A';
    println!("{}", c);

    let c = '」';
    println!("{}", c);

    let c = '我';
    println!("{}", c);

    let t:(i8, f64, u8) = (-6, 5.34, 8);
    println!("{} {} {}", t.0, t.1, t.2);

    let arr = [1, 2, 3, 4, 5, 6, 7];
    println!("{}", arr[0]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}", months[11]);
}
