use std::io;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// define struct methods
impl Rectangle {
    fn rectangle(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h }
    }

   fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let user1 = User {
        email: String::from("someuser1@example.com"),
        username: String::from("someuser1"),
        active: true,
        sign_in_count: 1,
    };
    println!("Hello, {}!", user1.username);

    let user2 = build_user(String::from("someuser2@example.com"), String::from("someuser2"));
    println!("Hello, {}!", user2.username);

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    println!("rect2 is {:?}", rect2);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::rectangle( 60, 45 );
    println!("rect3 is {:?}", rect3);
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq is {:?}", sq);
}
