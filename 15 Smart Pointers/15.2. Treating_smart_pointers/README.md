# Treating Smart Pointers Like Regular References with the `Deref` Trait

Implementing the Deref trait allows you to customize the behavior of the dereference operator, which is this character "*", which is different than multiplication or glob operator, so context matters! So, what benefits does normal reference have over smart pointer? Don't know but let's see how this chapter will explain it. 

## Following the Pointer to the Value

References are their own type (sort of), which is why in some cases the reference needs to be dereferenced in order to compare reference to a "real" value. 

```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

For example, in the code example we need to dereference the `y` with `*` operator to get the real value, which is a number. Without the derefence the reference would be a reference and the compiler would throw an error about how `assert_eq` macro can't compare number to a reference. In a way `*` operator is a fetcher.

## Using `Box<T>` Like a Reference

This is probably going to include some "witchcraft".

```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Basically it's functioning the same way, but without the usage of references. Value of `x` is passed normally as an argument when new `Box` instance is created. Doesn't mean that `Box` doesn't need to be dereferenced, `Box` itself is a pointer just like normal reference.

## Defining Our Own Smart Pointer

This part is just for excerise. Defining own smart pointer can be useful, but most of the time build in functionalities are more than enough. Creating new types in Rust is quite easy thanks to the `struct` type, which kinda functions like a class.

```rs
// Creating new tuple struct named box with a generic parameter T.
struct MyBox<T>(T);

// Let's add some functionality to it.
// New method creates new struct instance of MyBox.
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

Now we have basic "pointer", which lacks some functionality. At its current form it's unable to dereference itself. We probably need to introduce a trait to the struct that allows `MyBox` to be dereferencable. And seems like my guess was right.

**To enable dereferencing with the * operator, we implement the `Deref` trait.**

## Treating a Type Like a Reference by Implementing the Deref Trait

So, implementing a trait to a custom type. This topic has already been discussed in chapter 10, but if I remember correctly it was quite narrow discussion. So to the example.

```rs
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

At first this may seem a little bit weird, but let's go through some points so the code would make some sense. **The type Target = T; syntax defines an associated type for the Deref trait to use. Associated types are a slightly different way of declaring a generic parameter...** Because we want the origin of the referenced value we need to get the value of first tuple value with `&self.0` *(Yes, the method returns a reference on dereference, this is explained later on the book)*. `&self` refences to the value of the MyBox instance and `.0` references to the first value in a tuple (remember MyBox is tuple struct and has a tuple structure). And with that additional code the compiler shouldn't throw any errors... If you didn't make any typos that is.

**Without the Deref trait, the compiler can only dereference & references.** Makes sense because our custom struct type doesn't include the trait that includes functionality of dereference. That said there is probably no type that is dereferencable without the correct trait implementation. Additional info: **The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.** In other words (if I understood correctly) compiler calls `deref` method on `*` operator or it first checks if the type has the correct trait and then calls the method.

More additional information that may not be relevent, but it's interesting. **When we entered `*y` in Listing 15-9, behind the scenes Rust actually ran this code** `*(y.deref())`.

*The book includes great explanation about why references work the way they do, so read that if your mind needs some more explanation. Starts after the example code of `*(y.deref())`.*

## Implicit Deref Coercions with Functions and Methods

*Read the section. I don't want to try simplify something that interesting.*

## How Deref Coercion Interacts with Mutability

Besides `Deref` method there is also a `DerefMut` method that overrides the `*` operator on mutable references. Also **Rust does deref coercion when it finds types and trait implementations in three cases:**

- **From &T to &U when T: Deref<Target=U>**
- **From &mut T to &mut U when T: DerefMut<Target=U>**
- **From &mut T to &U when T: Deref<Target=U>**

The first two make sense, but the third one is trickier where I need to refer to the book.

**The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile)... Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that.**

In other words there are cases where specific functionality isn't guaranteed, which means that experience is gold to understanding why code isn't going to work and where.