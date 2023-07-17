# Advanced Functions and Closures

## Function Pointers

**you can also pass regular functions to functions!** Yes, please! This functionality is sometimes so neat to have when you just want to make the code as dry and readable as possible. So, what is function pointer? A pointer that points to a function, but in serious sense and for practical example `fn` type is called a function pointer (which is also case sensitive).

```rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

In above example, which is easy to follow so I'm focusing on the `do_twice` function. We have a `do_twice` function which takes a function pointer as an argument, which needs to return `i32`, inside the function we trigger the `f` function twice with the `arg` as the argument, which is passed to the `do_twice` as the second argument.

**Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.**

**Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure.**

That's... pretty neat, confusing a little, but neat. But is there a point where declaring the traits manually would be more useful?

This might come with a couple of chapter worth of delay, but isn't closure used in Rust as an alternative for nameless functions? If so, it's pretty easy to grasp when to use them.

Aaah, so enum variants become initializer functions when defined, which can be used as function pointers that implement the closure traits, which in turn means we can use the value the initializer functions as arguments.

```rs
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

Like, so.

**Here we create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value. Some people prefer this style, and some people prefer to use closures. They compile to the same code, so use whichever style is clearer to you.**

So it really doesn't matter if some people like to use closures, just make sure the code is consistent as to which one is used.

## Returning Closures

Why would you want to return closures? Wouldn't the result of a function pointer or a closure be more than enough?

**In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function. However, you can’t do that with closures because they don’t have a concrete type that is returnable.**

```rs
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

Which means the example is illegal and will result in Rust prompting an error, which indicates that the Rust is unable to determine the size of the return value.

```
1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
```

And the solution to fix this issue is to wrap the function pointer inside a Box, which is a trait object `fn returns_closure() -> Box<dyn Fn(i32) -> i32>`. If I am not incorrect the `Box` will have determined size on compile time so Rust won't be prompting an error about that again.