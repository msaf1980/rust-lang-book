use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut scores = HashMap::new();

    scores.insert(10, String::from("Blue"));
    scores.insert(50, String::from("Yellow"));

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(n) => println!("{} => {}", team_name, n),
        None => {},
    }
    for (key, value) in &scores {
        println!("{} => {}", key, value);
    }
}
