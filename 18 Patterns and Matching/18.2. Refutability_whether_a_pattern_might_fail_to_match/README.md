# Refutability: Whether a Pattern Might Fail to Match

The failure of a pattern matching, probably the biggest issue that could occur while trying to match and find all possible out comes. But let's see how book explains the issue of failure to match.

Patterns that can fail to match are grouped as *refutable*. Which can be highlighted in `if` statement. For example when matching `Some()`, because there is possibility that the value of `Some()` is `None`, which would need `if else` statement or `else` to match the value of `None`.

Those that cannot fail are grouped under *irrefutable* patterns, which consist of `let` statements and `for` loops, which either define or iterate values.

Let's dive deeper why these specific tools are categorised like this. Refutable group consist of tools that are used to handle failure states, while irrefutables are used to handle data (or this is atleast how I view how these tools are used).

The book has pretty great example of how refutables and irrefutables are used and how the category affects and explains why they work the way they work.


*If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: instead of using let, we can use if let. Then if the pattern doesn’t match, the code will just skip the code in the curly brackets, giving it a way to continue validly.*

First example that was given in the book. This could be read as `VALUE will be Some(x)`, which doesn't contain any failure state.

```rs
// Will produce an error because Some(x) doesn't match with None
let Some(x) = None;
```

The above code will not work as it is, which we can fix by introducing an `if` statement to the code, which is part of the refutable category meaning it can handle failure cases such as this one when pattern has a chance not to match with the value. And the code will be read like this: `IF VALUE matches with PATTERN { EXECUTE THIS CODE }`.

```rs
if let Some(x) = None {
    println!("{}", x);
}
```

Finally book gives a nice example that person implementing the code should know when to use for example `if` statement and when not to, because if there isn't possible failure state Rust compiler will whine about that the `if` statement placement doesn't make sense.