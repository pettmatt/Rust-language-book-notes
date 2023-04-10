# Test Organization

## Unit Tests

- Run in isolation
- Pin point where the code is and is not working
- Create tests module to each file to contain the test functions

### The Tests Module and #[cfg(test)]

The `#[cfg(test)]` annotation tells Rust to compile and run the test code only when the test command is run. When integration tests are put into different directory the annotation is not needed. Unit tests will always need the test annotation, because they are included in the same file as the tested code.

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

Above code includes the tests module, which is pointed out through cfg attribute which contains the test argument. The test itself has its own attribute `test` which seperates the tests from each others. Finally the `asset_eq!` macro checks if the test passes.

*cfg attribute stands for configuration, which tells Rust that the following item should only be included given a certain configuration option. In this case when rust notices that test command was run it will run the code in tests module(s).*

## Testing Private Functions

*There’s debate within the testing community about whether or not private functions should be tested directly...*

Sounds counter-intuitive to try to prevent testing on private functions. Isn't the point of testing to check if things work the way they should? How is private functions any different, sure they may be used differently in the code itself, but they still should be tested the same way. Or is this just something that "lesser" software developer wouldn't understand? 

*Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions.*

But it seems like Rust as taken the stand that private and public functions should be tested the same way, so to the next topic.

## Integration Tests

Integration tests are run separately from the library, which means the tests are only able to test the functions that are publically accessable through the public API of the library, just like the file that will use the library.

### The tests Directory

And here is an example what kind of structure could be expected in this kind of testing.

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

How the library is tested in action? Nothing special. Library is brought to the scope and test function is created which returns true of false. Note that this kind of implementation doesn't need tests module or annotation, because it operates in its own test file.

```rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

### Integration Tests for Binary Crates

*If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.*

*This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests can test the library crate with use to make the important functionality available. If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.*

Kinda handy.