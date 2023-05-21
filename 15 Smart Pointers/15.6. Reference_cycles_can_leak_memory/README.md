# Reference Cycles Can Leak Memory

Even if Rust is designed and builded like a modern language to prevent bad coding habits it's not impossible to write a program that has memory leaking problems. What is *memory leak* anyways? Well, when a process requires memory system reserves the amount that is needed and sometimes for some reason, it may never be reclaimed by the system. That is called memory leak, and when the process is executed multiple times it creates more memory leaking, which in turn creates performance issues (most notable examples can be seen in video games).

Weeeell how does memory leaks happen in Rust even if it tries to prevent it? **Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust.** By using `Rc<T>` and `RefCell<T>`. **it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.**

## Creating a Reference Cycle

So, the reason for *memory leaks* is reference cycles, how does it look like in Rust?

```rs
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // In this case Option can create memory leaks.
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// To show how things are progressing
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

Memory leaks go somewhat over my head so I let the book to explain why the code will create a cycle that never collects the memory.

**The reference count of the Rc<List> instances in both a and b are 2 after we change the list in a to point to b. At the end of main, Rust drops the variable b, which decreases the reference count of the b Rc<List> instance from 2 to 1. The memory that Rc<List> has on the heap won’t be dropped at this point, because its reference count is 1, not 0. Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well. This instance’s memory can’t be dropped either, because the other Rc<List> instance still refers to it. The memory allocated to the list will remain uncollected forever.**

**If you uncomment the last println! and run the program, Rust will try to print this cycle with a pointing to b pointing to a and so forth until it overflows the stack.**

In other words if the programs has `RefCell<T>` values that contain `Rc<T>` or other values that have the ability of interior mutability and reference counting. Rust can't catch memory leaks because they are logical mistakes, which can be caught with code reviews. The ability to notice and solve *memory leaking* problems come with experience.

## Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

