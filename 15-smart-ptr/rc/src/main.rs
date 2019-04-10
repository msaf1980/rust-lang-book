enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // mutability
    let mut data = Rc::new(5);

    *Rc::make_mut(&mut data) += 1;        // Won't clone anything
    let mut other_data = Rc::clone(&data);    // Won't clone inner data
    *Rc::make_mut(&mut data) += 1;        // Clones inner data
    *Rc::make_mut(&mut data) += 1;        // Won't clone anything
    *Rc::make_mut(&mut other_data) *= 2;  // Won't clone anything

    // Now `data` and `other_data` point to different values.
    assert_eq!(*data, 8);
    assert_eq!(*other_data, 12);


    //ReftCell

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Rc::clone(&value));

    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    *value.borrow_mut() += 10;

    c.try_borrow_mut().unwrap_or_else(|err| {
        eprintln!("RefCell is locked!");
    });

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

