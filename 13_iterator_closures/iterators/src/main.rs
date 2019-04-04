use std::iter::Sum;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

struct Counter {
    count: u32,
    _stop: u32,
}

impl Counter {
    fn new(stop: u32) -> Counter {
        Counter { count: 0, _stop: stop }
    }
}

// 1..stop
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < self._stop {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    for v in v1_iter {
        println!("{}", v);
    }

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut v1_iter = v1.iter();
    println!("sum: {}", v1_iter.sum::<i32>());

    let mut i = 0;
    // Transform value, returned by iterator with closure |x| x + 1
    for v in v1.iter().map(|x| x + 1) {
        println!("{} + 1 = {}", v1[i], v);
        i += 1;
    }

    // Collect items from iterator to vector
    let v2: Vec<_> = v1.iter().map(|x| x * 2 ).collect();
    i = 0;
    for v in v2 {
        println!("{} * 2 = {}", v1[i], v);
        i += 1;
    }

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    for s in shoes_in_my_size(shoes, 10) {
        println!("{}", s.style);
    }

    let counter = Counter::new(7);
    for c in counter {
        println!("{}", c);
    }

    // zip produce:
    // 1 * 2 = 2 - skiped ( x % 3 != 0 )
    // 2 * 3 = 6
    // 3 * 4 = 12
    // 4 * 5 = 20  - skiped ( x % 3 != 0 )
    // 5 None - not produced, becouse end of second iterator was reached
    let sum: u32 = Counter::new(6).zip(Counter::new(6).skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
