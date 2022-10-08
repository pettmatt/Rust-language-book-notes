# Chapter 3.1. - Variables and Mutability

Note 1: Variables are quite interesting in Rust. By using `let` to create new variables developer is able to over shadow it by using the same variable name without declaring it as a new variable (`let x = 0; x = 1`). In *shadowing* the original value doesn't disapear which is why the `example_two()` function is able to work how it does.

But if the variable would be created using `mut` it would indicate that the value will be changed later. But `mut` doesn't allow mutating the orignal type which is why
```
let mut spaces = "   ";
spaces = spaces.len();
```
fails, but would work using *shadowing*.

Note 2: Just incase note. `const` variables exists in Rust, but their usecase is little different compared to JavaScript. `const` variables should be named using a snake case naming schema (`this_is_snake`). `const` should also be used to make the code more readable by giving repetitive code snippets a name/explanation by binding it in a well named variable.