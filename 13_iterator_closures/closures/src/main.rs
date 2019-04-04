use std::thread;
use std::time::Duration;

fn generate_workout(intensity: &u64, relax: &u64) {
    let expensive_closure = |num| {

        println!("wait a moment ...");
        thread::sleep(Duration::from_secs(num));
        num
    };

    let name = "Vasya";
    println!(
        "do {}!",
        expensive_closure(*intensity)
    );
    println!(
        "Next, relax {}!",
        expensive_closure(*relax)
    );
}

fn main() {
    generate_workout(&2, &1);

    fn  function            (i: i32) -> i32 { i + 1 }

    // Closures are anonymous functions you can save in a variable or pass as arguments to other functions.
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}
