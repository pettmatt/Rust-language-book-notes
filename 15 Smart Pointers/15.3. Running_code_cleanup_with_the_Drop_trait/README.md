# Running Code on Cleanup with the `Drop` Trait

The header is pretty streight forward, so my guess for this topic is that the `drop` trait allows tweaking of the behavior of smart points on cleanup or when the operation is about to expire. Would make sense considering how `drop` keyword is used in SQL (I know, it's a little far fetched).

**The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.**

Eh.. close enough.

Some other languages would require the programmer to free memory when the memory is no longer needed (if they do not the program can become memory leaker), and on those kinds of situations something like a `drop` trait, that is called when a code goes out of scope would be useful. Thankfully in Rust the compiler does a lot of heavy lifting and we don't really need to think about freeing space, so this chapter is probably going to give a new perspective why something would be needed to be cleaned up right before going out of scope.

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

Example above shows how the `drop` trait can be implemented. Basically it's done the same way as the `defer` trait implementation, by creating a method `drop`, which will for now print line and inform what the program is losing access to. And if you run the code you can see when the `drop` method is being executed, right after the last `println` macro that is still in scope. Oh and **the Drop trait is included in the prelude, so we don’t need to bring it into scope** kinda neat, isn't it.

```
// Still in scope.
CustomSmartPointers created.
// Executed when the code is going to be unaccessable.
// Notice how d is dropped before c, drop order order is reversed compared to creation order.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

But this "clean up" trait is supposed to be used to clean up and free resources so let's see what the book has in store under the next sub header.

## Dropping a Value Early with std::mem::drop

As the header says the values can be dropped early, which is where the `drop` trait comes in to picture. Unfortunately, the drop method cannot be executed manually, which could release the smart pointer that manages the locks early, so some other part of the code (in the same scope) can acquire the lock. And this is where `std::mem::drop` comes in, which allows forcing early execution of the `drop` method.

```rs
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");

    c.drop();

    println!("CustomSmartPointer dropped before the end of main.");
}
```

Modifying the earlier code, which would drop the `c` early. But as expected the code will not compile, because we're trying to execute a method that shouldn't be executed manually. The error prompt gives us pretty good hint how we can drop the `c` smart pointer earlier by using the `drop` function, which probably means that using standard library's drop function would allow us to drop the smart pointer.

```
16 |     c.drop();
   |     --^^^^--
   |     | |
   |     | explicit destructor calls not allowed
   |     help: consider using `drop` function: `drop(c)`
```

```rs
// Note that the function doesn't need to be imported with `use`.
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");

    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}
```

And it works like toilet on a train. And to the topic of how this functionality can be used in Rust when Rust doesn't require manually handling the memory. **You can use code specified in a Drop trait implementation in many ways to make cleanup convenient and safe: for instance, you could use it to create your own memory allocator! With the Drop trait and Rust’s ownership system, you don’t have to remember to clean up because Rust does it automatically.** Pretty neat, that's one topic that I could read more about, sounds fun.

Note that the drop function/method is executed when the compiler recognices that the value that is about to be dropped is no longer used anywhere in the code.