// 4. Understanding Ownership

// pass reference only without copy or move
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(string: &mut String) {
    string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello"); // s is a new String

    s
    // move ownership for returned value
}

fn main() {
    let s1 = String::from("hello");

    let mut s2 = s1.clone();
    s2.push_str(" again");

    // move ownership, s2 is not valid
    let s3 = s2;

    let mut s4 = s1.clone();
    change(&mut s4);

    println!("{}, s1 ({})!", s1, calculate_length(&s1));
    println!("{}, s3 ({})!", s3, calculate_length(&s3));
    println!("{}, s4 ({})!", s4, calculate_length(&s4));

    let s5 = no_dangle();
    println!("{}, s5 ({})!", s5, calculate_length(&s5));

    // Error: value used here after move
    // println!("{}, s2!", s2);
}
