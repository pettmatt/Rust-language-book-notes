# Unrecoverable Errors with panic!

This is detailed overview what `panic!` includes.

## Unwinding or aborting to a response of a panic

**Book: By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.**

Because unwinding is a lot of work, developers can explicitly use abort option, which can be applied by adding a new simple value in profile section of `Cargo.toml` file.

```
[profile.release]
panic = 'abort'
```

*For more, check the Rust application.*

Note that `test` command doesn't trigger panic errors, so the application should be run and not just tested with `cargo`.