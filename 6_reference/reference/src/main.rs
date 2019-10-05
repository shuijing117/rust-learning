fn main() {
    let s = String::from("Hello Wrold");
    println!("s : {}", s);

    let world = &s[6..11];
    println!("world : {}", world);
}
