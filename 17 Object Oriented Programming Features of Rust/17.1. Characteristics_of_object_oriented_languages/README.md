# Characteristics of Object-Oriented Languages

This chapter handles characteristics of OOP in context of Rust language and will touch following concepts; common characteristics, namely objects, encapsulation and inheritance.

## Objects Contain Data and Behavior

*Read from the book.*

## Encapsulation that Hides Implementation Details

**Another aspect commonly associated with OOP is the idea of encapsulation, which means that the implementation details of an object aren’t accessible to code using that object.** Meaning the only way to modify the behavior of an object is through its public API. With this the most sensible way of modifying objects is to change and refactor the object's internals, which wouldn't make the object inaccessable in the code that uses the object.

*Check the rust file.*

**The option to use pub or not for different parts of code enables encapsulation of implementation details.**

## Inheritance as a Type System and as Code Sharing

**Inheritance is a mechanism whereby an object can inherit elements from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.**

In Rust it's impossible to create a struct that inherits the parent by using vanilla Rust. If we wanted to be able to create a struct that inherits fields from the parent we would need to use macros.

Cases when you would want to inherit from a parent:
- Implementing feature ones and using it multiple times (DRY code)
  - *Limited way of implementation through trait method*
- Maintaining a coherent structure between structs/objects
  - *Implementing traits allows overriting, which can be useful when structure is important, but the functionality needs some changes*
- **To enable a child type to be used in the same places as the parent type**
  - aka polymorphism