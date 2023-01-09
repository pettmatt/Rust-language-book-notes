# Bringing Paths into Scope with the use Keyword

In simple terms, shortcut to paths. Just like in JS `import x from library` or `const path = './images/image01.png'`. 
In Rust, we can define a path and reference to it by using the last part of the path which looks like this in action: 

```
use crate::front_of_house::hosting;

...

hosting::add_to_waitlist();
```

**Book: Paths brought into scope with `use` also check privacy, like any other paths.**

If we would want to move the function in `lib` file to its own **module** the path wouldn't work because it's not in the same scope.

```
use crate::front_of_house::hosting;

// In scope
pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}

// In different scope
mod customer {
  // Customer's scope
  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
  }
}
```

Shown error:

```
  // Doesn't specify that the path isn't in the same scope,
  // but informs that "hosting" is undeclared.

error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src\lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
```

**Book: Note that use only creates the shortcut for the particular scope in which the use occurs. Listing 7-12 moves the eat_at_restaurant function into a new child module named customer, which is then a different scope than the use statement, so the function body won’t compile.**

# Creating Idiomatic use Paths

With `use` we could have referenced to the function we were going to point to with `use crate::front_of_house::hosting::add_to_waitlist;`. In my eyes that is a little bit too cluttered which is why I would use `use crate::front_of_house::hosting;` path with `hosting::add_to_waitlist();`. But why would I do that? It looks better, we see which module we're using here and when we're executing a function we see clearly what function it is and where it's from. Book talks about `the idiomatic way to bring a function into scope with use` which means (more or less) that we're including the context in code which also makes the code more readable without the need of adding comments. 
*BONUS EXPLANATION: When we're not using `use` too specifically we can reuse it later if need be without creating a new path with `use` + it's easier to see which functions are locally defined. And our result is cleaner code!*

**Book: On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.** Fuck...

Actually with context (code) that makes a lot more sense. But example below is closer to how classes and libraries are used in other languages, but in Rust structs and items in general function like classes.

```
// This makes a lot more sense...
use std::collections::HashMap;
// ...than this
use std::collections;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

**Book: There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.** Yeah... I figured as much. With experience developers create habits how certain things should be written.

**Book: As you can see, using the parent modules distinguishes the two Result types. If instead we specified ``use std::fmt::Result`` and ``use std::io::Result``, we’d have two Result types in the same scope and Rust wouldn’t know which one we meant when we used Result.**

Makes sense and another valid reason why context is important when writing code. The text above is referencing to code snippet below:

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

# Re-exporting Names with `pub use`

**Book: When we bring a name into scope with the use keyword, the name available in the new scope is private.** OH... well that is good to know, kinda makes sense also when I think of it longer.

**Book: To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.**

*From this point onward I'm not going to write my thoughts if there's not anything **extremely** interesting/important stuff.*

**Book: Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.”** 

Makes sense, but that's going to need  some hands on experience.

# Using External Packages

Note for self; check out some packages on [crates.io](http://crates.io).

# Using Nested Paths to Clean Up Large use Lists

These snipets show how to import items with the same kind of path parts.

```
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

// VS

// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

What the heck is that syntax when importing multiple items from `std` in single line. That said it makes sense.

```
use std::io;
use std::io::Write;

// VS

use std::io::{self, Write};
```

# The Glob Operator

**Book: If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:**

```
use std::collections::*;
```

The `*` glob operator should not be abused because it will probably make the code less readable. There's no harm in using it while developing, poking and testing things.

And next topic is testing, which is probably going to be interesting (at least for me).