# Controlling How Tests Are Run

`cargo test` command works fine in smaller projects, but in the real world projects tend to be more complex and contain more tests. Here's when controlling when the tests are run comes handy. To visualize how this can be done we can use existing example. `cargo test` argument compiles the binaries, but `cargo test --help` gives options what can be done with the argument.

## Running Tests in Parallel or Consecutively

By default tests run parallel using threads, which means they more or less finish at the same time. If their result can be overwritten by other tests the output would need to be seperated in to their own files or calls or whatever the final result is. This is one case when running tests consecutively one after the other could be better if the final result needs the other test results in order to make sense or to be functionally useful.

Having more control over the tests can be done with `--test-threads` flag, which can be used to declare how many threads the test should use. the `--` in the command below is just a separator for the thread flag.

```bash
$ cargo test -- --test-threads=1
```

## Showing Function Output

*By default, if a test passes, Rust’s test library captures anything printed to standard output.* Which means any `println!` macro presented in the code is going to be ignored and only the result will be printed in the console.

```rs
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

This functionality is shown in the previous code, which contains a test that passes and one that fails. It won't produce any prints of the results, but if we wanted to overwrite this functionality we would only need to add a flag `--show-output`.

```bash
$ cargo test -- --show-output
```

## Running a Subset of Tests by Name & Running Single Tests

If you needed to run a specific test out of 50 tests, you could pass the name of the test as an argument in `cargo test` command. Let's use the previous example where one of the tests would return an error which we don't want to see.

The command:
```bash
cargo test this_test_will_pass
```

The result:
```bash
Finished test [unoptimized + debuginfo] target(s) in 0.01s
Running unittests src\lib.rs (target\debug\deps\how_tests_are_run-a56f262a73f8c16a.exe)

running 1 test
test tests::this_test_will_pass ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

This method doesn't support multiple function name arguments.

## Filtering to Run Multiple Tests

This is a little weird that this one works the way it works. But none the less let the show continue. Because the two tests share same parts of the name we can just pass one argument and trigger both tests by using `cargo test this_test_will`. The argument could be as short as `this`, but it would work the same way.

## Ignoring Some Tests Unless Specifically Requested

In case you want to exclude tests and only run them when requested, we can add additional attribute to the test.

```rs

#[test]
#[ignore] // Ignores the test as long as it is not requested.
this_test_will_fail() {
    ...
}
```

If we wanted to run the test we would need to pass a flag `--ignored`, which would run ignored tests, just like this `cargo test -- --ignored`. And if you wanted to run every test it would require `--include-ignored` flag.