mod utils;
use utils::first_word;
use utils::first_word2;

fn main() {
    let s = String::from("hello world");

    //String slice range indices must occur at valid UTF-8 character boundaries.
    //If you attempt to create a string slice in the middle of a multibyte character,
    //your program will exit with an error.
    let hello = &s[0..5];
    let world = &s[6..s.len()];
    println!("{}!", hello);
    println!("{}!", world);

    let world = &s[6..];
    println!("again, {}!", world);

    let hello = first_word(&s);
    println!("again, {}!", hello);

    let hello = first_word2(&s);
    println!("again, {}!", hello);
}
