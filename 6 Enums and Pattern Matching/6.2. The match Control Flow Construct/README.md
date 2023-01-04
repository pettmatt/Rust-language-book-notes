# The match Control Flow Construct

So, what is this chapter going to hold within it? I predict *(after reading a paragraph)* that the point of this chapter is to introduce the reader *why*, *when* and *how* Rust handles data comparison the way it does.

## Types of comparison operators

- `Match`, a keyword that can match patterns. **Make sure all possible options are covered.**


## Main rust file notes

The used `match` arm syntax works basically the same way as the popular `if`-statement, but why would we ever use something that could be done with `if`-statements? At this point and time, no idea. My hypothesis is that the arm syntax functions better in some scenarios, like in this one that loops through an `enum` and returns the processed data.

The book points out that the expression of if-statement needs to return a boolean value, while in the example the expression is a coin enum.

**Deep dive into match arms**

`Coin::Penny` is the expression which needs to be **MATCHED** in order to execute the code after `=>`. The code in this case returns an integer matching the value of the coin in cents.

## Patterns that Bind to Values

By including another `enum` within an `enum` the program can match **Quarter** with a **state**, using `Quarter(UsState)` syntax. **Check the main.rs file for more example about binding value to another enum.**

**In short, binding means that the value inclueds binded value within it. Example Quarter(BINDED_VALUE)**

## Matching with Option<T>

Syntax wise matching <T> is quite easy, but the syntax can seem a little complex at first. In this example `match` compares `x` value, if it holds a value add 1 into it, other wise do nothing.

**Book: Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.**

## Matches Are Exhaustive

There are some weak points when using `match` keyword. The arm syntax **must** cover every possible option in order to compile.

If every option is not covered Rust will prompt with an error. The reason why this error is prompt can be found in the book. **Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.**

```
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
note: `Option<i32>` defined here
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
4   ~             Some(i) => Some(i + 1),
5   ~             None => todo!(),
    |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```

Not every error is recognized by Rust, which may cause some grey hair.

## Catch-all Patterns and the _ Placeholder

**Note that we have to put the catch-all arm last because the patterns are evaluated in order. If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!**

To add to earlier paragraph it's also good to remember that computers and programming languages are quite stupid. They process data in order and don't think/see any other possible option after the first match.

Note: It's kinda funny how you can just name an arm with any name if it's supposed to handle other values that have not be explicitly mentioned in other arms.

If I understood the text correctly there is a special character (pattern) that is used to tell the compiler that if there is other possible values, don't use it (don't bind it), trigger this function instead. 
Example:

```
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  _ => reroll(),
}
```

This `_` pattern also fulfills the exhaustiveness requirement, because it makes sure other values are ignored. If we wanted to do nothing when explicit value wasn't rolled, we could replace ``reroll`` function with empty **tuple type** `()`.