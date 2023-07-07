# Advanced Traits

## Specifying Placeholder Types in Trait Definitions with Associated Types

Associated types in context of traits are somewhat regularly used by Rust developers, which is probably why it's considered to be advanced technique. Creates some necessary functionality in specific scenarios, but a program can function without them.

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

So, `type` Item is created, which makes it a placeholder. Method `next` just return the value of the `Item`. If we implemented `Iterator` the implementors of the trait would need to specify the concrete type of the  `Item`.

```rs
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        ...
    }

    ...
}
```

The book points out that the way of implementing `Iterator` trait is quite close to creating the trait with generics. So, why would we do it this way if we already have a way of implementing `Iterator` trait using generics.

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// VS

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

Using generics allows implementing a type multiple times, which has it's ups and downs. Depending on scenario maybe generics would be better. When it comes to associated types, we can annotate type once, meaning we can't implement a trait on a type multiple times. 

In practice it means we can only have `impl Iterator for Counter` once. Because we have limitation that the type can be implemented once, we don't need to specify what type the iterator is, every time we want to use the implemented `next` method. In the case of `Counter` we would need to specify that we want to interact with iterator of `u32` type.

## Default Generic Type Parameters and Operator Overloading

Second part of this header is somewhat troubling. "Operator overloading" sounds like something that can "break" existing rules of operators, but let's see what it really means.

```rs
use std::ops::Add;

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

So, the main functionality that we care about is included in the `Add` implementation for `Point` struct. `type` Output determince the type which is returned by the `add` method, which creates a new `Point` by combining two `Point` structs together.

The other thing that we care about is the trait `Add`, which includes the default type parameter `<Rhs=Self>`. The default type parameter is used as a placeholder value, which is used by default if during the implementation the value isn't overwritten during implementation.

And let's move to another example, which should clear things more.

```rs
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

This implementation works quite similarly, but in this case we use generics in implementation `impl Add<Meter> for Millimeters`, which will overwrite the placeholder which was in the previous example the `Rhs` type.

Just like any other placeholder in programming, the type parameter is used to **extending a type without breaking existing code** and to **allow customization in specific cases most users won't need**. In the examples we used type parameters as a placeholder or default value to prevent extra parameters.

## Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

**Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type. It’s also possible to implement a method directly on the type with the same name as methods from traits.**

To be honest isn't that obvious? At least it was on my mind the moment traits were introduced.

```rs
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

Shouldn't the above be already introduced and clear? We implement `Human` struct to two traits and then overwrite the default `fly` method, because we have to either overwrite or adopt the default functionality.

```rs
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

And ofcourse because the "default" functionality of a human is implemented using `struct` the syntax of calling default `fly` and implemented is going to be different. As seen above. But then there is "optional" syntax which is the following `Person::fly(&person)`, which throws my previous statement right into the trash. But if you think about it more closely we're just using the barebones version of the `fly` method using `Type::method(reference)` syntax, because we have to offer the reference. Compared to using the method of a `Human` struct, which is able to use the `self` value of the already assigned `struct` without the need of throwing reference manually as an argument.

The dog example of Rust not being able to know which instance of the trait we want is pretty good example to understand what kind of implementation could result in wrong thing being returned. In the case of `Animal` trait being implemented to overwrite the original `baby_name` and calling it directly, which won't work because Rust is unable to recognize which instance of that `trait` we want to call (or something similar. My explonation may be a little off).

Anyway, the point is that we need to use "new" syntax to tell Rust what we meant. The `<Dog as Animal::baby_name()` syntax *(fully qualified syntax)* tell Rust that we want to call the `baby_name` method of a `Animal` trait which is implemented to `Dog`. **We’re providing Rust with a type annotation within the angle brackets, which indicates we want to call the baby_name method from the Animal trait as implemented on Dog by saying that we want to treat the Dog type as an Animal for this function call.** And with that the code should print the right output.

## Using Supertraits to Require One Trait’s Functionality Within Another Trait

**Sometimes, you might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait. You would do this so that your trait definition can make use of the associated items of the second trait. The trait your trait definition is relying on is called a supertrait of your trait.**

Mmm m... makes sense.

```rs
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

So, if I understood correctly the `supertrait` of the example is the `fmt::Display` trait, which is "extended" with `OutlinePrint`, which is a custom trait with the function of `outline_print`. So, what do we gain from this? Just like extending an class we get access to functions from `Display` trait, just like the book taught us in the trait chapter. And now we can "legally" call `to_string` method through `self`.

Because we have depedency of `Display` trait we cannot implement our new trait with a struct that doesn't implement `Display` trait. For example the `struct` of Point will prompt an error, because `Point` doesn't implement `Display` trait.

```rs
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

We need to satisfy Rust by implementing `Display` to `Point`. And I presume that we also overwrite the `fmt` function with our own implementation, which returns `Result` through `write!` macro?

## Using the Newtype Pattern to Implement External Traits on External Types

**We mentioned the orphan rule that states we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate. It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct.**

At this point I shouldn't be surprised that this many advanced techniques just break the safety rules previously stated through previous chapters. At least it's not that mind breaking with these examples.

**Newtype is a term that originates from the Haskell programming language. There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.**

Kinda surprised that there isn't runtime performance penalty.

**As an example, let’s say we want to implement Display on `Vec<T>`, which the orphan rule prevents us from doing directly because the Display trait and the `Vec<T>` type are defined outside our crate.**

I thought that it worked that way, but I needed some more clarification.

```rs
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

So, in action it looks like the example. `struct Wrapper(Vec<String>);` after which we implement the trait to `Wrapper` (`impl TRAIT for Wrapper`). Then we can just call `Wrapper` and we should get desired output.

**The implementation of Display uses self.0 to access the inner `Vec<T>`, because Wrapper is a tuple struct and `Vec<T>` is the item at index 0 in the tuple. Then we can use the functionality of the Display type on Wrapper.**

Nice detailed explanation. But new types hold some down sides and the book can take it away! 

**The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding. We would have to implement all the methods of `Vec<T>` directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a `Vec<T>`. If we wanted the new type to have every method the inner type has, implementing the Deref trait on the Wrapper to return the inner type would be a solution.**

But additionally if we wanted to implement just a few we would need to implement them manually (didn't want to copy or leave that part out).

