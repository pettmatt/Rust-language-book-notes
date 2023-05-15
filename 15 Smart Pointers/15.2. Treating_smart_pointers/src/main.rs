use std::ops::Deref;

// Implement Deref in MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    // Vanilla reference example
    // let x = 5;
    // let y = &x;

    // assert_eq!(5, *y);
    // assert_eq!(5, x);

    // Using smart pointer(s), which is basically the same
    // let x = 5;
    // let y = Box::new(x); // Isn't a reference point, but a copied value

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}