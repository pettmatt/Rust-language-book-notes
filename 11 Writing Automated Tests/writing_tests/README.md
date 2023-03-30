# How to Write Tests

Tests in Rust are functions that test that the non-test code is functioning as expected. The structure of tests are as follows.

- Set up any needed data or state.
- Run the code you want to test.
- Assert the results are what you expect.

## The Anatomy of a Test Function

Tests are pretty easy to define and identify thanks to `#[test]` before the declaration of the function (`fn`). Tests can be run with `cargo test` which creates binaries for the tests.

```rs
#[cfg(test)]
mod tests {
    // Generic test example
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

If the test panics during the run time it's considered as failure. The error/failure prompt will give more information where the error occured and what triggered it, which in this case was the panic macro. 

```rs
#[test]
fn another() {
    panic!("Make this test fail");
}
```

## Checking Results with the assert! Macro

This chapter focuses on giving context how `assert!` macro can be used to test if value is boolean.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

So, as long as the assert receives a boolean and the logic is true the test passes.

```rs
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}
```

If test found an error it would look like this.

```
failures:

---- tests::larger_can_hold_smaller stdout ----
thread 'main' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:28:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Testing Equality with the assert_eq! and assert_ne! Macros

Because testing is in most cases testing if the end product is using correct type or is equal to what is expected, tests can utilize macros or passing the result to expression using the `==` operator. `assert_eq!` and `assert_ne!` are macros that can be used to replace the more tradition expression operators such as `==`. These two are part of the standard library, which are often used to perform tests more conveniently.

```rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

But because this tests are supposed to catch bugs here's an example what the prompt will be if the values passed to `assert_eq!` are not equal.

```
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

If we wanted the test to pass only when the two values are not equal we could use `asset_ne!` macro. *The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal.*

## Adding Custom Failure Messages

*You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros. Any arguments specified after the required arguments are passed along to the format! macro.*

```rs
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing is usually done with dummy data so even if this 
    // test seems kinda stupid it's some times better to add multiple of these smaller tests
    // to catch when/if the basic functionality breaks when new features are being developed.
    // (I'm not a tester, so this statement may be somewhat wrong).
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

Then if we wanted to add cutomized error message the test function would look like this. Just like previously explained.

```rs
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

## Checking for Panics with should_panic

Some times developers need to test if the function will panic in certain conditions and ofcourse there is an attribute for that called `should_panic`. With this attribute panic is expected and will not break the test or declare it as a failure.

```rs
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

*Tests that use should_panic can be imprecise. A should_panic test would pass even if the test panics for a different reason from the one we were expecting. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the failure message contains the provided text.*

Modified example looks like this.

```rs
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

With the changes the expected panic case has a context in a way of customized message what is expected.

## Using Result<T, E> in Tests

Test managment can be done through result, which moves the failure state from panicing to controlled failure state.

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

This method has some conditions that should be addressed before commiting in using either way of testing.

*The it_works function now has the Result<(), String> return type. In the body of the function, rather than calling the assert_eq! macro, we return Ok(()) when the test passes and an Err with a String inside when the test fails.*

*Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.*

*You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).*