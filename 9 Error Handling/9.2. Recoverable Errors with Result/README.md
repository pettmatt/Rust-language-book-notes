# Recoverable Errors with Result

Panicing is not always the best way of handling errors, which brings us to more productive error handling with `Result`. So what is `Result`? It's enum that has two possible variants; `Ok` and `Err`. The enum also has following **generic** type parameters; `<T, E>`. `T` represents the type of the value that is returned in a success case, and `E` represents the type of error that is returned in a failure case.

When it comes to error handling one of the basic tools is `match` expression, which is pretty easy way of going through possible options of `Result`.

*Check the rust application*

## Matching on Different Errors

*Check the rust application*

### Alternatives to using match with `Result<T, E>`

Because error handling can include a lot of nesting with `match` expressions there is more appealing ways of handling them.

*Check `advanced_error_handler` function from the rust application*

## Shortcuts for Panic on Error: unwrap and expect

This part introduces what unwrap is and how it can make the code more readable. In short, with unwrap developer can handle `Result` state and check the values with if-statements without going through `match` expressions which are somewhat "out dated", meaning they are okey to use, but there are better options.

Unwrap can be used like follows. If returned `Result` is not `Ok` it will call panic macro.
```Rust
let greeting_file = File::open("hello.txt").unwrap();
```

If we wanted to prompt our own error message we could add it by using `expect`, which has the same functionality, but it allows custom error messages.
```Rust
let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
```

**Book: In production-quality code, most Rustaceans choose `expect` rather than `unwrap` and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.**

## Propagating Errors

What does *propagating error* mean? Well, instead of the function handling an error, we could give the caller more control and **return the error to the calling code so that it can decide what to do**. This is also one way to simplify the code within a function. Why would you need to handle the error outside the function? Some times when you want to know if it fails or not you may want to handle the error differently in different cases when the function is called.

*Check the code inside of `propagating_errors` function*

**Book: This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.**

### A Shortcut for Propagating Errors: the ? Operator

Apparently error handling is so common and follows the same pattern that there is an operator that is designed to handle errors, which works almost the same way as `match` expression.

**Book: If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.**

When `?` operator is called it will call `from` function, which in turn, according to the book **"When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function"**. Pretty neat.

*Check the code inside of `propagating_errors_advanced` function*

Neat, but all of this can be done a lot easier, but without the need to handle errors, which makes this next snippet kind of useless when we want to learn error handling.

```Rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    // Yes, this next line handles everything that the previous code handled.
    fs::read_to_string("hello.txt")
}
```

## Where The ? Operator Can Be Used

**Book: The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function, in the same manner as the match expression we defined earlier.**

Meaning...

**Book: Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.**