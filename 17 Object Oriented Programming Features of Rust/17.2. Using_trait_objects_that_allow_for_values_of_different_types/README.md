# Using Trait Objects That Allow for Values of Different Types

Here we explore different way of implementing different types to an object. Just in case you forgot, in chapter 8 we implemented an enum inside a vector that could hold different types. That was one way of implementing what the goal of this chapter is, in context of inheritance.

## Defining a Trait for Common Behavior

**Trait objects aren’t as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.**

*Check the lib rust file (Draw, Screen and implementation of Draw in Screen).*

## Implementing the Trait

*Check the main rust file.*

**The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway. Rust won’t compile our code if the values don’t implement the traits that the trait objects need.**

## Trait Objects Perform Dynamic Dispatch

*Read from the book.*