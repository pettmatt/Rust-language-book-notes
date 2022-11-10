# 4.2. References and Borrowing

References are pretty easy to understand. They are "shortcuts" or "pointers" that tell the program where the value is using already existing value, which is better for the memory. 

## Normal references

*There's a lot of examples how you could visualize it, but for me it's easier to think that reference is just a rabbit hole to a variable value.*

When it comes to ownership, reference is one way to maintain the original ownership while using the value somewhere else. 

**What does this mean in action?!** Well, this is a fancy way of saying Rust will not drop the value after the reference has been used. For example when reference is passed as an argument the reference will only be available while the function is in scope, after the scope, the reference is unavailable, but the origin is still available.

*Book has really good example how references should be handled using the logic behind borrowing something in real life. You don't usually want to tweak something you're borrowing and references follow the same logic.*

## Mutable references

Syntax wise mutable is easy to grasp. `&s1` becomes `&mut s1`. One big drawback about mutable references is the fact that there can be only one instance of the same mutable reference.

```
    // Creates an error.
    // Mutation should be controlled which is why this example fails.
    // By deleting 'r1' or 'r2' the problem will be fixed. Other fix would be to control the mutation, which would prevent 'data races'.
    let r1 = &mut s;
    let r2 = &mut s;
```

```
    // Fix example 2
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

Also the origin should be mutatable. Example below would create an error.
```
    // Adding 'mut' before variable name would fix the error.
    // Like this -> let mut s1 = String::from("hello");
    let s1 = String::from("hello");
    let mut_len = add_to_length(&mut s1);
```

This next example is just one of the interesting examples of how Rust works. Without the `println` macro Rust would prompt an error.

```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

## Dangling References

**This one is unique to languages with pointers!** 

What is dangling reference? An reference that have been preserved even if it should have been dropped. In Rust this would mean an reference that has somehow survived even if it's out of scope, which is luckily impossible in Rust. In Rust references will be used after which the origin will be dropped when it's out of scope.

Example how Rust prevents dangling with error prompts.
```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // Return s as a reference, which would create a dangling reference
}
```

The origin is dropped so the reference is pointing into 'the void', which is something that no software would want, so the reference should be just returned as normal value with `s` (dropping the reference prefix `&`).