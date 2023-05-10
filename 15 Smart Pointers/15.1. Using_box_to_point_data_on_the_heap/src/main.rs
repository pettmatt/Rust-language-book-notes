enum List {
    // Because Rust is unable to tell how much data List will require,
    // we can store it in a box which means we're indirectly using it.
    // Additionally Box will always tell how much space it needs.
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // One way of using Boxes, but this isn't the main use case. 
    // Storing just one value is a waste of Boxes.
    let b = Box::new(5);
    println!("b = {}", b);

    // The change done in enum List needs to be reflected here.
    // Old code: Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Compared to other smart pointers Boxes provide only the indirection and heap allocation.