// Invalid
//fn longest<'a>(x: & str, y: & str) -> & str {
//
// Valid - Lifetime Elision
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // The text of this string is stored directly in the binary of your program, which is always available. Therefore, the lifetime of all string literals is 'static.
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}
