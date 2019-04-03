fn match_check(i: i32) -> i32 {
    match i {
        0 | 2 => 1,
        1 => 2,
        _ => 0
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("match {} => {}", 0, match_check(0));
    println!("match {} => {}", 2, match_check(2));
    println!("match {} => {}", 1, match_check(1));
    println!("match {} => {}", 3, match_check(3));

    let five = Some(5);
    let six = plus_one(five);
    match six {
        Some(i) => println!("{} + 1 = {}", 5, i),
        None => println!("None"),
    }

    let none = plus_one(None);
    match none {
       Some(i) => println!("{} + 1 = {}", 0, i),
       None => println!("None"),
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // If need check one equatation, shorter way using 'if let'
    let some_u8_value = Some(0u8);
    if let Some(0u8) = some_u8_value {
        println!("8");
    }
}
