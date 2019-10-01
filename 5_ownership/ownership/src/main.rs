fn main() {
    //Move
    let s1 = String::from("Hello");
    let s2 = s1;

    //println!("s1 : {}", s1);  //You'll get an error in here.
    println!("s2 : {}", s2);

    //Clone
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 : {}", s1);
    println!("s2 : {}", s2);

    //Copy
    let x = 5;
    let y = x;

    println!("x : {}", x);
    println!("y : {}", y);

    let s1 = String::from("Hello");
    make_move(s1);
    //println!("s1 : {}", s1);//You'll get an error in here.

    let x = 32;
    make_copy(x);
    println!("x : {}", x);
}

fn make_copy(x : i32) {
    println!("x in make_copy : {}", x)
}

fn make_move(a_string : String) {
    println!("a_string in make_move : {}", a_string);
}
