# Validating References with Lifetimes

## Preventing Dangling References with Lifetimes

The main functionality of lifetimes is that they get rid off dangling references. Dangling references can be harmful when the reference is referencing to a value that shouldn't exist anymore. Example below will prompt an error that is caused by a reference that has a borrowed value that doesn't live long enough (as expected), which can be easily fixed by just assigning the value directly `r = x`.

```rs
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

But that would probably not fit for the specific reason why the value should be assigned as a reference. To add to that this example demonstrates how Rust handles dangling references, if Rust didn't prompt an error and allowed dangling references to continue living on (even if they shouldn't) the reference would point to a memory that was deallocated and my hypothesis is that in some cases the reference could reference to other value that took the location of the old reference.

## The Borrow Checker

So, borrow checker. What is their deal? What are they checking? Well, this may or may not shock **you**. The borrow checker compares scopes to determine if the borrows are valid. The previous example can be visualized in the context of borrow checker like so.

```rs
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

The visualized scopes show that scope b doesn't have much of a lifetime compared to a and will be out of scope in no time.

## Generic Lifetimes in Functions

Example how generic lifetimes can some times create weird prompt errors that basically say that they need more information about the lifetimes of the parameters. `longest` function will prompt an error that tells to use a named lifetime parameters.

```rs
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Lifetime Annotation Syntax

*Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types. Most people use the name 'a for the first lifetime annotation. We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.*

*One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. Let’s examine how the lifetime annotations relate to each other in the context of the longest function.*

Working example would look like this.

```rs
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Thinking in Terms of Lifetimes

The lifetime parameter `'a` can be also applyed to just those values that have relationship. In the next example return type is specified to have some kind of relationship with the first parameter and none with the second one. This allows the function to compile.

```rs
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

Next example shows that even if the parameters have no relationship with the returned value, it still can prompt an error, just because the returned value goes out of the scope right when the function ends. Why? Because the returned value is dangling reference which is removed by Rust. Best way of fixing the function would be to return value with the ownership and not just a reference.

```rs
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// Error
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
```

*Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.*

## Lifetime Annotations in Struct Definitions

By default structs hold the owned types, but structs can be defined to hold references, which would require the lifetime annotations on every reference.

```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

## Lifetime Elision

In some cases where adding annotations would make sense the code can be compiled without an error, which is thanks to legacy feature where every reference would need an explicit lifetime. These cases were predictable and followed a few deterministic patterns. *After which Rust developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.*

```rs
// Current
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Pre 1.0 Rust
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

*This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.*

These rules are called lifetime elision rules, which aren't rules for programmers to follow. *They’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.*

## The Static Lifetime

Static life time is a way of making sure that the reference lives in the scope for the entire time the program is running. It's adviced to take a double take to make sure the variable benefits for using static and in some cases using static can create a dangling reference or a mismatch of the available lifetimes. In those cases using static is not the solution.

```rs
let s: &'static str = "I have a static lifetime.";
```

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rs
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

*This is the longest function from Listing 10-21 that returns the longer of two string slices. But now it has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause. This extra parameter will be printed using {}, which is why the Display trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.*

## Summary

The concepts of the chapter 10 are meant to prevent developers from writing repetitive code, which in turn promotes good coding practices. Here's a summary how the book puts it:

*Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in many different situations. Generic type parameters let you apply the code to different types. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime performance!*