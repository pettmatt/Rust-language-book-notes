# Advanced Types

## Using the Newtype Pattern for Type Safety and Abstraction

## Creating Type Synonyms with Type Aliases

Creating synonyms with type aliases can make the code more readable and easier to understand by refering to certain types in certain context with a readable type such as `type Kilometers = i32;`. Everyone would understand from the `type` synonym that it's supposed to contain kilometers.

```rs
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

But remember that the alias is not new type. It's a new way of refering to a type, in this case to `i32`, which also explains why the kilometer would be used like we would use `i32` types. Because the `Kilometer` will act as a `i32` (because it's just an alias) we can't type check it, which means if we mix a `Kilometer` with `i32` Rust won't be able to prompt about the possible problem with the code.

To be honest I don't really see the lack of type checking as a problem, but I can see why it's portraited as such.

Seems like I was somewhat off with the explanation why the type alias would be useful. With longer types the use of an alias is pretty clear and with the bonus of making the code more readable and dry, it's kinda no brainer why we would want to use aliases in case of `Box<dyn Fn() + Send + 'static>` types.

```rs
// thunk is a word for code to be evaluated at a later time
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

And the above example shows more detailed example how the alias can be used and how it can make the code cleaner.

```rs
// Without Alias
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

```rs
// With Alias
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

Another, probably more useful example would be to make an alias for `Result<T, E>` type, which is returned quite often by the standard library. The alias `Result<T>` is able to take one type, but if it's not specified the `std::io::Error` is returned.

## The Never Type that Never Returns

And now to the new type named `!`, which also known as *empty type* because it has no values.

```rs
fn bar() -> ! {
    // --snip--
}
```

**This code is read as “the function bar returns never.” Functions that return never are called *diverging functions*. We can’t create values of the type `!` so bar can never possibly return.** Yeah... but why would we declare return value as `!`? Wouldn't it be better to not allow it to return `fn bar() {}`? Or is it used when specified what a method should return or something similar?

```rs
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

**The formal way of describing this behavior is that expressions of type ! can be coerced into any other type. We’re allowed to end this match arm with continue because continue doesn’t return a value; instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.**

Interesting, not a surprise that keywords can hold that kind of logic behind them.

Another example where `!` is utilized is `panic!` macro where it holds value of `!`. It has no value, but it will end the program, which allows it to be used in `match` arms, which need to hold same value or `!`.

**One final expression that has the type `!` is a loop. The loop never ends, so `!` is the value of the expression. However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.**

## Dynamically Sized Types and the `Sized` Trait

Dynamically sized type is what it sounds like, a type which will change it's size based on the value it holds. Example of this kind of type is `str`, which whatever reason is pointed out that we mean `str` by its own, not `&str`, which would be a reference and hold some value inside it meaning it's not dynamic type anymore.

```rs
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

Because **Rust needs to know how much memory to allocate for any value of a particular type** the above example won't run. The book points out that the example above will result in two variables that hold the same type, but will require different amount of memory, which is the main reason why in Rust we're unable to create a variable holding a dynamically sized type.

Normal `&T` contains the address to the value, but `&str` holds the address and the value. Because the `&str` knows its size (twive the size of a `usize`) we can without a problem create a string using it and it will hold the same size no matter how long the string is. **In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information. The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.**

In practice we don't interact directly with dynamically sized types, but with Rust features that use them. Such as traits. **We mentioned that to use traits as trait objects, we must put them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>`.**

**By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction**

```rs
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

**A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default that generic types must have a known size at compile time. The ?Trait syntax with this meaning is only available for Sized, not any other traits.**