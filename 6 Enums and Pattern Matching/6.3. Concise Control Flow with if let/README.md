# Concise Control Flow with if let

So, concise control flow with if let. What does this mean in action? It's probably a continuation of the `6.2.` chapter, but this time focused how `match arms` are not always the perfect option, which is why programmer should understand when to use different syntaxes, for example ``if-statements``.

`Match arms` are clean and easy way to include multiple options, but it can be bothersome when there is only one option.

Example with `match arms`:
```
let config_max = Some(3u8);
match config_max {
  Some(max) => println!("The maximum is configured to be {}", max),
  _ => (),
}
```

Example with `let if`:
```
let config_max = Some(3u8);
if let Some(max) = config_max {
  println!("The maximum is configured to be {}", max);
}
```

That said if you come from frontend development environment this kind of syntax within if-statement can be a little confusing which can be cleared by **experience** with Rust.

So let's continue step by step. If-statements in Rust don't use `()` which is why the bare if-syntax looks like this `if statement {}`. "But why do we have variable inside the statement?", because it's called `let if`-syntax. The statement itself needs a pattern and expression, which looks like this ``if pattern = expression { do the thing }``. The if-statement works the same way as the `match arms`, which is why we can also bind a value and use it within the if-statement body. After this the code looks like this 
`if let Some(max) = Some(3u8) { println!("{}", max) }`

**Book: Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.**

## If let else vs. match arms

This example is from previous chapter, which showcasese how the coin example could have done using if-statement. In my opinion `match arms` are some what easier to understasnd visually, but I would say it depends who is writing the code and what they are used to. Neither is wrong, but it depends what should be used.

**Match arms:**

```
let mut count = 0;
match coin {
  Coin::Quarter(state) => println!("State quarter from {:?}!", state),
  _ => count += 1,
}
```

**If let else:**

```
let mut count = 0;
if let Coin::Quarter(state) = coin {
  println!("State quarter from {:?}!", state);
} else {
  count += 1;
}
```
