fn main() {
    // strings are UTF-8 encoded

    let mut s = String::new();
    s.push_str("bar");
    println!("{}", s);

    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let hello = "Здравствуйте";

    for c in hello.chars() {
        println!("char {}", c);
    }
    for b in hello.bytes() {
        println!("byte {}", b);
    }
    let s = &hello[0..3];
    // panic - UTF8
    let s = &hello[0..4];
}
