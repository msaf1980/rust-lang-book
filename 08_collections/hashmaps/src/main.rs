use std::collections::HashMap;

fn main() {
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let team_name = String::from("Blue");

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let score = scores.get(&team_name);
    match score {
        Some(n) => println!("{} => {}", team_name, n),
        None => {},
    }

    println!("-- recreate");
    let mut scores = HashMap::new();

    scores.insert(team_name.clone(), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{} => {}", key, value);
    }

    scores.insert(team_name.clone(), 20); // overwrite existing value
    println!("-- update {} => {}", team_name, scores[&team_name]);

    println!("-- update or insert {}", team_name);
    *scores.entry(team_name.clone()).or_insert(100) *= 2; // update existing value or insert new if not exist

    for (key, value) in &scores {
        println!("{} => {}", key, value);
    }

}
