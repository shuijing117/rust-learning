fn main() {
    let s = String::from("Hello World");
    let i = first_word_simple(&s);
    println!("i : {}", i);
    
    let word = first_word_slice(&s);
    println!("word : {}", word);
}

fn first_word_simple(s : &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item ==  b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}