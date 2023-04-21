## Capturing the Environment with Closures

So, what are closures and how do they affect the program and what does capturing the environment mean? In the case of this Rust program the `giveaway` holds one closure case. 

```rs
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
}
```

**The `unwrap_or_else` is part of the standard library and it takes one argument, a closure without any arguments and returns a `Option<T>`. If the `Option<T>` is the `None` variant, `unwrap_or_else` calls the closure and returns the value returned by the closure.**

**We specify the closure expression || self.most_stocked() as the argument to unwrap_or_else. This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars).**

## Closure Type Inference and Annotation

Closures can seem a little hard to grasp, but the book has a great example how to identify closure from function.

**The first line shows a function definition, and the second line shows a fully annotated closure definition. In the third line, we remove the type annotations from the closure definition. In the fourth line, we remove the brackets, which are optional because the closure body has only one expression. These are all valid definitions that will produce the same behavior when they’re called. The add_one_v3 and add_one_v4 lines require the closures to be evaluated to be able to compile because the types will be inferred from their usage. This is similar to `let v = Vec::new();` needing either type annotations or values of some type to be inserted into the Vec for Rust to be able to infer the type.**

```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

And then the closure could be used as follows. Pretty neat way of creating functionalities that can be binded streight to the variable.

```rs
let n = add_one_v2(5);
```

## Capturing References or Moving Ownership

By default closures don't take ownership if the value is used within them, but they can also possess ownerships and caputre references. First a simple example how ownership doesn't change when the value is used within closure.

```rs
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
```

The code above prints following output, which shows that ownership doesn't change:

```
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
```

Even if the closure changes the contents of the list, the final print still has full access to the list and to the new values.

And then example where a closure needs to take the ownership in order to operate as expected, which means the variable is no longer usable in the main thread. Yes this example uses threads to show a scenario where it's practical to give an ownership to closure.

**We spawn a new thread, giving the thread a closure to run as an argument. The closure body prints out the list. In Listing 13-4, the closure only captured list using an immutable reference because that's the least amount of access to list needed to print it. In this example, even though the closure body still only needs an immutable reference, we need to specify that list should be moved into the closure by putting the move keyword at the beginning of the closure definition. The new thread might finish before the rest of the main thread finishes, or the main thread might finish first. If the main thread maintained ownership of list but ended before the new thread did and dropped list, the immutable reference in the thread would be invalid. Therefore, the compiler requires that list be moved into the closure given to the new thread so the reference will be valid.**

```rs
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

And as the book explained, the `move` keyword is used to move the ownership to the closure, and without this the compiler would advice using it, because the new thread can outlive the main thread, which means the list could go out of scope.



Closure is pretty neat tool, but grasping it completely will require some time with it to undestand when to use it and when not to. Highlight from the book: **The Fn traits are important when defining or using functions or types that make use of closures. In the next section, we’ll discuss iterators. Many iterator methods take closure arguments, so keep these closure details in mind as we continue!**