# All the Places Patterns Can Be Used

Patterns are useful to identify different points when to identify data and how to handle it without using a lot of hard coded code blocks. There are possibilities that a person has used patterns without identifying them as patterns, which would imply that the word pattern holds wider meaning than I'm aware of, but this chapter will probably clear things for me.

**`match` Arms**

```rs
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    ...
}

match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Match arm should be the obvious one out of the many patterns, which has been used pretty often in the book excercises. One downside or upside, depending on when the `match` arms is used is that it should be *exhaustive*, meaning the arms should include every possible pattern. One way of covering every pattern is to use a *catchall* pattern, which will cover other cases that don't have their own pattern. The catchall pattern uses `_` symbol, which in a way can be viewed as `else` clause of a if-stsatement.

**Conditional `if let` Expressions**

```rs
if let VALUE_TYPE = VALUE {
    ...
} else if VALUE {
    ...
} else {
    ...
}
```

Even if `if let` expression is stated as `if let`, the expression functions as normal if-statement and can include `else if` and `else` clauses. Notice how `if let` is only part of the expression that needs to hold the `let` keyword, else if can also hold the `let` keyword, but isn't needed if it compares the same value type.

If you remember when the `if let` was introduced you know that `if let` can offer a simpler way of writing `match` arms, so it's important to identify what traits of a specific expression are needed at the moment.

One downside of `if let` expression is that it will not check if every condition is checked. This means the expression is **not** *exhaustive*. This can create a logic error that the compiler is unable to catch and prompt, but if you don't need to cover every possible out come if-statement can come handy.

**`while let` Conditional Loops**

```rs
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

`While loop`, the bane of my existence. In some environments it works well, like in PHP, but in some environments it can break the logic, just like in JavaScript environments. So how does it fare in Rust environments?

The example above will function as expected. Using the `pop` method will return the last element, which will return values in this order `3, 2, 1`. And after the `pop` method has returned `1`, the first element, the `pop` will return `none`, because the loop has gone out of scope of the vector element and will not return any value after the 1.

**`for` Loops**

```rs
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

Aaah... the lovely, gentle `for` loop that is the most used loop of them all, at least in other langauges. Compared to `for` loop of a JavaScript the Rust for loop is simpler, in a way. The example above *demonstrates how to use a pattern in a for loop to destructure, or break apart, a tuple as part of the for loop*, compared to JavaScript this would have needed a statement like this `for (let i = 0; i < v.length; i++)`, which is not that readable, but you get used to it through experience.

Back to Rust, the book explains the code pretty well, at least for me it is somewhat confusing way of iterating an array by destructuring it. *We adapt an iterator using the enumerate method so it produces a value and the index for that value, placed into a tuple. The first value produced is the tuple (0, 'a'). When this value is matched to the pattern (index, value), index will be 0 and value will be 'a', printing the first line of the output.*

Rust is interesting language or I'm starting to see things differently through my experience in Rust. For example `for` loop uses tuple to get index and value out of the destructured array. Earlier I would have noted the tuple as an unique syntax piece of a `for` loop.

**`let` Statements**

```rs
let PATTERN = EXPRESSION;
let x = 1;
```

It seems like `let` statement itself can be considered to be a pattern. The book offers a detailed explonation how `let` statement in variable creation can be considered to be a pattern. 

*In statements like `let x = 5;` with a variable name in the `PATTERN` slot, the variable name is just a particularly simple form of a pattern. Rust compares the expression against the pattern and assigns any names it finds. So in the `let x = 5;` example, `x` is a pattern that means “bind what matches here to the variable x.” Because the name x is the whole pattern, this pattern effectively means “bind everything to the variable x, whatever the value is.”*

But this can be seen clearer in a tuple and when it's binded to a value. But I let it be in the book.

**Function Parameters**

Of course parameters would also count as patterns if variable declaration can be counted as a pattern. Kinda makes sense, but does it matter in action when variables and parameters are being used.

