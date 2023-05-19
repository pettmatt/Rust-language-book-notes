# `RefCell<T>` and the Interior Mutability Pattern

**Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.** What? Doesn't that defeat the point of having immutable values? Maybe it makes sense in a context. 

**To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.** Wow WOW! Isn't that a little too hack'ish way of implementing a functionality?

**Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us.** Aaah, well that makes sense, but how is the manual checking done?

If I understood correctly we can mutate values through `unsafe` code that has to make checks to ensure that the borrowing rules are being followed, even if we're mutating values using unsafe code it's happening through a safe API and the original type isn't being manipulated.

## Enforcing Borrowing Rules at Runtime with `RefCell<T>`

`RefCell` is only able to represent single ownership so the usage case is different compared to `Rc<T>`. Compared to `Box<T>`, `RefCell<T>` needs to follow borrowing rules or the compiler will panic. The borrowing rules will be checked at runtime.

**If Rust accepted an incorrect program, users wouldn’t be able to trust in the guarantees Rust makes. However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing catastrophic can occur.** Holyyy... That is... something. It makes sense, but it sounds funny and some how little bit backwards. "It's okey if Rust prompts incorrect error message when program should run, but if it allowed broken application, it could create bigger problems."

The book includes nice recap why and when to use `Box<T>`, `Rc<T>`, or `RefCell<T>`

- **`Rc<T> `enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.**
- **`Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.**
- **Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.**

## Interior Mutability: A Mutable Borrow to an Immutable Value

And then some actual code examples. Wohooo. Let's start with example that won't compile. For this example we're trying to borrow `x` variable and at the same time change it to be mutable. Which won't work because the `x` is inmutable which canno't be borrowed as mutable value.

```rs
fn main() {
    let x = 5;
    let y = &mut x;
}
```

Why would we even want to change change a inmutable value to be mutable. Well the book has good point that some times it makes sense to be able to change a value in a specific context and not allow the change in any other context. **However, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code.** If programmer breaks borrowing rules the compiler will panic instead of giving compiler error.

### A Use Case for Interior Mutability: Mock Objects

Okey, this part introduces some new terminology when it comes to testing code and so on.
    
- **Test double** - a placeholder type, think of it like a "stunt double".
- **Mock object** - types of test doubles that record what happens during a test.

Even if Rust doesn't include objects and mock object functionality, the same functionality can be implemented through `structs`. And here's what the excerise in book is trying to achieve. **Here’s the scenario we’ll test: we’ll create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is. This library could be used to keep track of a user’s quota for the number of API calls they’re allowed to make, for example.**

```rs
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger
                .send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

Pretty heavy, but the book explains things well.

### Keeping Track of Borrows at Runtime with `RefCell<T>`

Let's implement a mock object to test if book's implementation of `RefCell<T>` works. The code doesn't compile yet, but the panic prompt indicates that some borrow rules were not followed. In this case the `already borrowed: BorrowMutError` was broken. And this is how `RefCell<T>` handles rule violations.

```rs
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // borrow_mut() returns RefMut<T>
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

Even if errors and panics seem heart breaking and set the project back, these functionalities are in place for the developer. These kinds of breaks make sure that developers write good code that works when the code is compiled.

## Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

So, how do we implement `RefCell<T>` to make it possible to mutate values in `Rc<T>`? `Rc<T>` makes it possible to store and keep track of the references, but it doesn't allow changing values in it, but if we put `RefCell<T>` in it we can mutate that value, but the `Rc<T>` would be kept as inmutable.

```rs
#[derive(Debug)]
enum List {
    // Ofcourse the RefCell needs to be implemented in the enum itself.
    // After this point it would be impossible to implement it.
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

I don't think there is anything important to point out about the code itself. It works as intended, but the code could probably be streamlined, but it wouldn't be necessary it would just make sure the `Cons` receives the arguments correctly with less repetitive code.