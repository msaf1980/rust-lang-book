enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.insert(0, -1);
    for i in 0..v.len() {
        println!("mut v[{}] = {}", i, v[i]);
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    let mut i = 0;
    for e in &v {
        println!("v[{}] = {}", i, e);
        i += 1;
    }

    let i = &v[2];
    println!("v[2] = {}", i);
    let k = v.get(3);
    match k {
        None => println!("element not exist!"),
        Some(n) => println!("v[3] = {}", n),
    }

    let rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &rows {
        match i {
            SpreadsheetCell::Int(n) => println!("int {}", n),
            SpreadsheetCell::Float(n) => println!("float {}", n),
            SpreadsheetCell::Text(n) => println!("str {}", n),
        }
    }
}
