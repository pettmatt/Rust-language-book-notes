# Pattern Syntax

Here the book goes through possible pattern syntaxes and how and where to use them.

## Matching Literals

```rs
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

In its simplicity, `match` needs a variable to match its arms with, in the example we use literals to match. Additionally we can match any value that isn't specified with `_`, think of it as `else` clause.

## Matching Named Variables

```rs
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {y}"),
    _ => println!("Default case, x = {:?}", x),
}
```

So, what is named variable? *Named variables are irrefutable patterns that match any value*. In this case the example contains one value that will be printed with any value, which is the second arm `Some(y)`. To be honest I'm not quite clear why `Some(y)` ends up getting value of 5. The book talks about how the new scope will shadow earlier values that are not being matched, which would probably mean that the second arm has no value, which allows it to match with any value?

My explanation was quite alright, but it was missing some details that I want to add for clarification. Let's get back to math, the basics. When solving x in math the value can be anything, but the surrounding numbers and symbols give the x a context of what it is (` 1 + x = 3, x = 2 `). The second `match` arm works kinda the same way, it has no access to what the `y` was before, so inside the scope Rust creates a new `y` variable, which has no value, meaning it can be anything, anything as long as it's an instance of `Some()`. Finally, after the first arm doesn't match, the second arm will get the inner value of `Some(5)`, which will match. Remember the original `y` has nothing to do with the `y` inside of the `match` scope.

If we wanted to compare outer values together, in this case x and y, we would need to use `match guards`.

## Multiple Patterns

```rs
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

Just like in other languages in Rust we can have multiple patterns. Example above shows how we can have multiple patterns in a single `match` expression.

## Matching Ranges of Values with `..=`

```rs
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

We can also have more "dynamic" patterns, for example with the following syntax `1..=5`, which will match with everything between 1 and 5. Rust can also handle this syntax with `char` values, pretty neat isn't it?

## Destructuring to Break Apart Values

### Destructuring Structs

```rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

Example above shows how a `let` statement can be used to break a `struct` into seperate variables, which can create more simplistic code and variable structure in certain situations. Anyway the structure of this operation can be broken like this `let TYPE { NEW_VARIABLES } = ORIGINAL_VARIABLE`. Additionally we could simplify the original breaking process like this `let Point { x, y } = p;` if we didn't want to rename the properties, but it's also good to know the original longer way of doing things.

```rs
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

And ofcourse we can use destructuring in earlier examples (that's the beauty of programming). And the above example will match, again with the second arm, because it has to contain `x: 0` and `y` matches with any. Finally we have the `_` or else clause, which matches for anything, which uses `Point { x, y }` pattern, which won't trigger in this case because `match` can only match with one arm.

### Destructuring Enums

```rs
// Won't compile, just an example
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```

Destructuring can be done in multiple ways, depending on where the destructuring is happening and to what kind of data. And the example above includes destructuring in parameters and properties (structs, strings, tuples). In the first arm the destructuring is impossible which is why we match on the literal `Message::Quit`.

### Destructuring Nested Structs and Enums

```rs
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
```

And if needed we can even destructure nested values which means we can match values that are one level deeper or even deeper, but in that case we need to ask how many nested values are acceptable in order for the code to stay readable. Destructuring this far can be beneficial when it's useful to `match` with specific type of value.

### Destructuring Structs and Tuples

```rs
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

More destructuring, but in this case we have more advanced example. We want to destructure a tuple `((number, number), Point)` that holds a tuple and a struct to a tuple and a Point struct.

## Ignoring Values in a Pattern

### Ignoring an Entire Value with `_`

```rs
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

The underscore `_` also known as a wildcard pattern can be used to ignore a value, in this case a parameter. This can be useful when a trait is implemented into a function that doesn't require one or more parameters of a trait. This would prevent compiler from prompting a warning about unused parameter values.

### Ignoring Parts of a Value with a Nested `_`

```rs
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
```

Because in the example we haven't specified what value is going to get caught in the first `match` arm it's going to match any `Some()` value and will only pass to the next arm when the value is anything else than instance of `Some()`.

```rs
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {first}, {third}, {fifth}")
    }
}
```

But if we wanted to ignore a value completely we could use method shown above. There we just want to use first, third and fifth values.

### Ignoring an Unused Variable by Starting Its Name with `_`

```rs
fn main() {
    let _x = 5;
    let y = 10;
}
```

In Rust when we have unused variables the compiler will warn about it and suggests adding `_` as the prefix to ignore the variable. 

```rs
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

*Note that there is a subtle difference between using only _ and using a name that starts with an underscore. The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.*

Another note about the example. Even if `_` is used the binded value will still offer the ownership as normally. But if the `_` is used alone without variable (`if let Some(_) = s `) it won't bind any value and ownerships are not transfered, which is pretty obvious to me.

### Ignoring Remaining Parts of a Value with `..`

```rs
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

If I needed to guess how `..` functions in the example I would say that it will take the first value and ignore the rest. **Checking through Rust playground** And it does work like that. The `..` operator can be used to replace `_` operator when only a few values are relevant.


```rs
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {first}, {last}");
    }
}
```

*The syntax `..` will expand to as many values as it needs to be*

As seen above we can use `..` to get first and last or if we wanted `(first, .., middle, .., last) =>` would probably be valid to get three values out of the tuple. Note that `..` needs to be unambiguous, meaning it needs to be clear what values we want to extract from the variable or Rust will prompt an error. Below you can see an example that won't compile.

```rs
match numbers {
    (.., second, ..) => {
        println!("Some numbers: {}", second)
    },
}
```

## Extra Conditionals with Match Guards

*A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen. Match guards are useful for expressing more complex ideas than a pattern alone allows.*

```rs
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {} is even", x),
    Some(x) => println!("The number {} is odd", x),
    None => (),
}
```

So the match guards give us more control over what kind of values go through, but the compiler doesn't check if the guard expressions are exhaustive, meaning that in some cases these guard conditions may create issues that cannot be detected through compiler, but through debugging.

```rs
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {n}"),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {y}", x);
```

We can also check if a value is equal to a variable that is outside of the scope through conditional guards. But why? Well that's because the guard `if n == y` isn't a pattern and is unable to create new variables, which means the original `y` isn't shadowed out. The guard can be implemented to more complex patterns, such as `4 | 5 | 6 if y`, which will apply the guard to each condition.

## `@` Bindings

```rs
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {}", id),
}
```

The at (`@`) operator will allow us to create a variable that holds a value at the same time as we’re testing that value for a pattern match. 

*In the second arm, where we only have a range specified in the pattern, the code associated with the arm doesn’t have a variable that contains the actual value of the id field. The id field’s value could have been 10, 11, or 12, but the code that goes with that pattern doesn’t know which it is. The pattern code isn’t able to use the value from the id field, because we haven’t saved the id value in a variable.*