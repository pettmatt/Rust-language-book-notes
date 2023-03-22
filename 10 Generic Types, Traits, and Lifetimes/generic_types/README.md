# Generic Data Types

## In Function Definitions

Creating multiple functions that basically function the same way with some differences throws us to the same problem we had without the function. We write specific logic twice with some differences, which can be written better by using `generic types`. Check the code example.

Note that the application won't compile at this point, because not every type can be ordered, `: std::cmp::PartialOrd` narrows the types to only those that have `PartialOrd` implementation.

## In Struct Definitions

Using generic types in structs is possible, but these is certain things that should be noted.

```
struct Point<T> {
    x: T,
    y: T,
}
```

Example above supports only one type, which makes following instance of struct is impossible `let wont_work = Point { x: 5, y: 4.0 };`. The instance is trying to define a new `Point` by using two different types. If that kind of behavior is needed the struct should be changed as follows.

```
struct Point<T, U> {
    x: T,
    y: U,
}

// Now this kind of Point instances are possible
fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## In Enum Definitions

Using generic types is nothing too crazy after handling them in struct. It follows basically the same syntax, which can include more than one type by adding another definition for the second type, like this `<X, Y>`.

```
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

In short generic types are useful tool in Rust when there needs to be some kind of way to alter the types, for example in functions, structs and enums.

## In Method Definitions

Implementing generic type in a method is little different, but only a little. In order for `Point<T>` to be considered as generic type the `impl<T>` is needed. This just tells Rust that the `Point<T>` doesn't contain a concrete type. Oh, also `impl<T>` could have been defined using different name, but using the same `<T>` is conventional.

```
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

And if we only wanted to implement the method to structs with the type of `f32` we could define the `impl Point<T>` with `f32`, just like below.

```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

And ofcourse if we wanted to support more than one type then the implementation would look like below. Nothing too crazy going on.

```
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```