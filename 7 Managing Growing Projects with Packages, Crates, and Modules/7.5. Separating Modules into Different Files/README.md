# Separating Modules into Different Files

**Book: So far, all the examples in this chapter defined multiple modules in one file.**

Are we finally going to explore how to include modules from other files?! Oh my. I bet it's done with syntax similar to the one used in the past for the `use` keyword.

**Book: Note that you only need to load a file using a mod declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the `mod` statement), other files in your project should refer to the loaded file’s code using a path to where it was declared, as covered in the “Paths for Referring to an Item in the Module Tree” section. In other words, `mod` is not an “include” operation that you may have seen in other programming languages.**

Hmm... interesting, I was wrong in that part. So `mod` is a declaration operator that tells the compiler where the code can be found and if it doesn't include its own body it's going to check the file that includes the module (?). How interesting language.

**Book: For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:**

- **src/front_of_house/hosting.rs (what we covered)**
- **src/front_of_house/hosting/mod.rs (older style, still supported path)**

**If you use both styles for the same module, you’ll get a compiler error. Using a mix of both styles for different modules in the same project is allowed, but might be confusing for people navigating your project.**

Just like in any other programming language, when you stick to one style of writing code it's easier to follow (even if it takes some time to grasp what happens and where).

**Book: Note that the `pub use crate::front_of_house::hosting` statement in src/lib.rs also hasn’t changed, nor does `use` have any impact on what files are compiled as part of the crate. The `mod` keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.**

## Final thoughts

When it comes to managing a project, it seems like the people that were developing Rust had one thing in mind, how to entice developers to write clean and readable code by designing a system that uses simple syntaxes. And yes, the way that Rust does certain things are in my opinion simple, but somewhat confusing because I don't have enough experience using Rust.