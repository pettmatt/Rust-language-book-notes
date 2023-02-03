# To panic! or Not to panic!

It's important to know when you can make the program panic safely, after all `panic!` crashes the whole application which in turn affects how the user views the program. Spoiler, while the application is being developed panicing is more than enough to show how the application would function, without wasting time on how it **will** function after it's closer of being finished.

## Examples, Prototype Code, and Tests

**Book: When you’re writing an example to illustrate some concept, also including robust error-handling code can make the example less clear. In examples, it’s understood that a call to a method like unwrap that could panic is meant as a placeholder for the way you’d want your application to handle errors, which can differ based on what the rest of your code is doing.**

**Similarly, the unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.**

**If a method call fails in a test, you’d want the whole test to fail, even if that method isn’t the functionality under test. Because panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.**

## Cases in Which You Have More Information Than the Compiler

There isn't much here, but a developer can include methods in a code that is never going to trigger, but it offers context for other developers. For example in the book (code snippet below) IP address is hard coded and will never create errors, which makes `expect` method useless, but it explaines why an error isn't expected. If the IP address was given by user there would be a chance of an error and the code would need to be changed to include better error handler.

```Rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
  .parse()
  .expect("Hardcoded IP address should be valid");
```

## Guidelines for Error Handling

**It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:**

- **The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.**
- **Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.**
- **There’s not a good way to encode this information in the types you use. We’ll work through an example of what we mean in the “Encoding States and Behavior as Types” section of Chapter 17.**

For me, this requires some live examples or more experience to understand when crashing an application is better option than fixing it through custom error handling.

**If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so the user of the library can decide what they want to do in that case. However, in cases where continuing could be insecure or harmful, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development. Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.**

**However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a Result indicates that failure is an expected possibility that the calling code must decide how to handle.**

Well that explains it.

*The book includes some more good examples*

## Creating Custom Types for Validation

Structs can be used to create custom types that are used for validation. Below is some code to grasp the idea. Pretty easy to grasp, but needs some getting used and hands on experience.

```Rust
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

  pub fn value(&self) -> i32 {
    self.value
  }
}

loop {
  // --snip--

  let guess: i32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
  };

  if guess < 1 || guess > 100 {
    println!("The secret number will be between 1 and 100.");
    continue;
  }

  match guess.cmp(&secret_number) {
    // --snip--
}
```

## Summary

Panicing is okey if (for example when) passed argument is incorrect type. "Real" error handling is expected when there is a possibility of a process to fail. Of course these are not the only cases when to panic and not to, but it functions as good enough foundation to build on.