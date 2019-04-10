extern crate add_one;
use add_one::add_one;

fn main() {
    let x = 1;
    println!("{} + 1 = {}", x, add_one(x));
}
