# `Rc<T>`, the Reference Counted Smart Pointer

This chapter is going to be somewhat interesting because the title isn't really saying anything to me. Interesting, apparently a value can have multiple owners, but in this case they are refering to a same value, for example a pie chart has multiple points that would need to be stored to multiple objects in order to draw it correctly *(A starts from 0 and ends at 45 degrees, B starts from 45 and ends to 360 degrees)*. Not a perfect example, but it shows where a value is used multiple times and could have multiple owners.

And this is where `Rc<T>` comes into our scope, in order to allow multiple ownerships you need to enable it. Enabling is done in Rust using type `Rc<T>`, **which is an abbreviation for reference counting**. The type keeps track of how many times the value is referenced to aka how many *ownerships* the value belongs to. If the type recognizes that there is no references to a value, the value can be cleaned up without creating any errors. *There's a fun example of how this would work in real life in the book.*

So, when would we need to use this type? We would need to use it when we need to read a value in multiple places where we don't know / can't determine at compile time which part will finish using the data last. If we knew when to switch the ownership we could just handle the situation by manually changing the owner.

## Using `Rc<T>` to Share Data

In action you could use `Rc<T>` in following situation.

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // A is used by B and C.
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
    // Because Rc<T> type isn't introduced the code doesn't compile.
}
```

Of course, this error could be resolved by using references in `Cons`, but that would defeat the purpose of this exercise and it would require some more changes. By changing `Cons` to use `Rc<T>` we can use our code with minimal additions. So, let's modify the code by replacing `Box`. Below you can see some more changes done to the `b` and `c` variables, if we didn't use clone and references we would get the same error as before and to explain why, try to remember what `Rc` is supposed to do. Keep track of the references and clean up when the value isn't being referenced any more.

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

And now it should compile. Let's go through some key points; `Rc` needs to be brought to the scope because it is not part of the prelude, `a.clone()` could have been used, but that would have created a deep copy of **all** the data, compared to `Rc::clone` which only increments the reference count (this is also faster to finish).

## Cloning an `Rc<T>` Increases the Reference Count

And now to demonstrate how `Rc` counts the references and adjusts when one of the references goes out of scope.

```rs
fn main() {
    ...
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

The code above would create following console output. Which creates an output that was expected, as the text shows. Because `Rc` includes multiple `count` functions we need to use `strong_count`. We can't show in code, but the `Rc` is going to decrease the count of references to 0 when the main goes out of scope.

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```