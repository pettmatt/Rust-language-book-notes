# 4.1. What is Ownership?

**Read main.rs file inside of `owenership/src` for previous comments.**

This file contains notes after the pointer redirectioning or "`the move`". Speaking of that, `the move` can be visualized as redirecting the previous variable into a new. "But why would I do that", well let's say you want to have readable code, you could use one and the same variable to contain mutated value of the original and hope you will remember what the value is in certain moment when your code breaks and you have no idea what the mutated value is. Well renaming the variable or "moving" the pointer when the name doesn't describe the contents would be a great way to lessen the amount of comments. Ofcourse there is probably a lot of better examples why you would/should redirect pointers to another variable.

**Important note (1)**: Just because Rust doesn't copy variable values automatically, it doesn't mean that it's not possible, but it needs to be a choice made by the programmer. Cloning can be done using the `clone` method.

```
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

**Important note (2)**: Different value types act somewhat differently. For example copying can be done without cloning method with integers. Check example below.

Do not worry it's not magic, but the size of an integer is declared on build time, so the cloning can be executed "automatically". 

```
    let x = 5;
    let y = x; // Copy x into y

    // Prints out "x = 5, y = 5"
    println!("x = {}, y = {}", x, y);
```

**Important note (3)**: Variable can be copied only if it has a `copy` trait. This information can be found in documentation, but general rule, scalar values can implement `copy`.

**Nice thing to remember. Whenever a variable is out of scope it will be dropped. Rust automatically calls drop for the variable, which returns resources to the system.**

**So what ownership means in action?** To be honest this is kinda a nothing burger, it's good to know and gives better understanding how Rust functions. In short what ownership is all about is who has ownership of certain values. For example in scope of a function the function has ownership over the variables that are inside of it. When the function is run and completed everything that the function has ownership over is droped. Ownership can be given away by returning a value. *(I know. This is kinda messy example.)*


# 4.2. References and Borrowing