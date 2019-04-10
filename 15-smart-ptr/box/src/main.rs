use std::any::Any;

fn print_if_string(value: Box<dyn Any>) {
    if let Ok(string) = value.downcast::<String>() {
        println!("String ({}): {}", string.len(), string);
    }

}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(u32, Box<List>),
    Nil
}

fn main() {
    let m = Box::new(String::from("Rust")); // alloc in heap
    hello(&m);
    drop(m); // manual drop, if needed.

    let my_string = "Hello World".to_string();
    print_if_string(Box::new(my_string));
    print_if_string(Box::new(0i8));
}
