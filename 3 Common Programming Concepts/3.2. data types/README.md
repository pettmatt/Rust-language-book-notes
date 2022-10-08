# Chapter 3.2. - Data Types

Note 1: Let's start comparing Rust to JavaScript again! Because Rust is **a statically typed language** it needs to know every type of variables. My guess is this `let hello;` kind of declaration is not allowed and would prompt an error or a warning.

Answer: **Correct**. That kind of declaration prompts following error.

```
error[E0282]: type annotations needed
 --> src/main.rs:6:9
  |
6 |     let guess;
  |         ^^^^^
  |
help: consider giving `guess` an explicit type
  |
6 |     let guess: _;
  |              +++
```

The error points out what's wrong and how it can be fixed. How handy!

Note 2: Difference between `Singed` and `Unsigned`. When ever a number is positive it doesn't need a sign to implicate that, but when it goes to negative it needs some kind of sign to show it. In the world of the future we use `-` also known as minus sign to show that a number is a negative one. `-1` singed and `2` unsigned. We clear?

Book: *Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.*

Note 3: Interesting...

Book: *When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics...*

Note 4: Interesting indeed...

Book: *Rust supports the basic mathematical operations you’d expect for all of the number types...*

Note 5: As expected.

Book: *Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.*

Note 6: Wouldn't have even noticed that.

Book: *However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.*

Book: *f you’re unsure whether to use an array or a vector, chances are you should use a vector. ... However, arrays are more useful when you know the number of elements will not need to change.*

Note 7: Hmm... doesn't that mean that vectors and arrays function with the same idea in Rust that `const` and `let` does? In relation to each other. Vectors can change in length, but arrays need to maintain it, but its values can be changed.