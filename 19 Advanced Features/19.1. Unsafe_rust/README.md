# Unsafe Rust

Even if unsafe rust sounds little dodgy it's quite useful in some cases. Up until now the book has used safe rust environment, which means the Rust has had its memory safety guarantees enforced at compile time. So, why would we want to expose our safe environment to the uncertainty and danger? Because the "safe" Rust makes it almost impossible to do certain things (that could be counted as being risky operations), even if the code is safe to use as is in safe mode the Rust might not have enough information to determine that the code is OK to use. 

In a way unsafe Rust can be thought as the admin mode of Rust, if you know how the tool works this mode wouldn't really affect you, but if you did stuff without deep knowledge in the topic, you might create bigger issues that could have been caught by the safety guarantees. So don't be like Linus and try to do "impressive" stuff with a software you don't completely understand *(and scream bloody murder)*, just because you thought you knew what you were doing ([Steam installation on Linux mistake](https://youtu.be/0506yDSgU7M?t=597)).

## Unsafe Superpowers

Interesting how Rust handles this mode. I would have thought that it would make the whole code in specific file unsafe, but now that I think about it this approach makes more sense. If we have code that needs to utilize unsafe code we can mark the code block as `unsafe` and the compiler probably "skips" it and "trusts" that the code is alright.

Here's some powers we gain from using unsafe mode, but remember that we have five actions to use in unsafe Rust:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

Because `unsafe` code block turns off a lot compiler checks, it's the job of a programmer to make sure the code is safe. `unsafe` keyword doesn't mean the code is unsafe in nature, but it shows a place where the code could contain unsafe code, because the compiler isn't making some checks on it. So, keep them small, check them often and development should be less stressful.

**To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API, which we’ll discuss later in the chapter when we examine unsafe functions and methods.** Quite interesting approach to implement unsafe code.

## Dereferencing a Raw Pointer

Raw pointers seem to break every possible rule of a normal pointer. Let's see if I can wrap my head around it. Anyway, it seems like raw pointers are assigned using the aterisk `*` which is also used when dereferencing, but in this case it goes before `const` or `mut` ( `*const T` and `*mut T` ).

And here's some examples how different raw pointers are to references and smart pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

```rs
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

Example above shows how raw pointers can be created, pretty streight forward. We create variable just like any other time, point it to a reference, in this case `num`, which is then specified to be certain type of raw pointer with `as *const|*mut TYPE` using `const` or `mut` depending on if we're creating an immutable or a mutable raw pointer. This example won't need `unsafe` keyword, because creating a raw pointer doesn't require it, but dereferencing a raw pointer will need it.

```rs
let address = 0x012345usize;
let r = address as *const i32;
```

Book includes pretty good example of a raw pointer which could or could not be valid. The reference example will always be valid raw pointer, because it references to it. Above code example is directly accessing to a memory address, which may or may not include data depending on what Rust have done in the background.

```rs
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

Then let's check how dereferencing works on a raw pointer. To be honest I was expecting something more complicated, but the code that includes the dereferencing is just wrapped with `unsafe` bracket. So using a value of a raw pointer may point to an invalid value, which is why we need to access it through `unsafe` code.

**We created *const i32 and *mut i32 raw pointers that both pointed to the same memory location, where num is stored. If we instead tried to create an immutable and a mutable reference to num, the code would not have compiled because Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references. With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race. Be careful!**

Hmm... interesting to say the least.

## Calling an Unsafe Function or Method

**By calling an unsafe function within an unsafe block, we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts.** So it works with the same logic as `sudo` and other "admin" modes.

```rs
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

If we want to define an unsafe function we need to add the `unsafe` keyword before `fn` and call it inside `unsafe` block. And you can guess what happens if we try to access unsafe code without the unsafe block, the compiler will promopt an error.

*Note: because unsafe function acts as an unsafe block we don't need to add unsafe block inside it or to wrap it with it.*

### Creating a Safe Abstraction over Unsafe Code

Marking whole function as `unsafe` isn't that common if we want to keep the unsafe code as controllable as possible. Based on what the book said unsafe code is more manageable if only the unsafe part is put inside `unsafe` block. That said if the function is short enough, it doesn't really matter.

```rs
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);

...

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}
```

The `split_at_mut` function is part of stantard library and uses some `unsafe` code, which makes it good subject of a study of unsafe code.

**This function first gets the total length of the slice. Then it asserts that the index given as a parameter is within the slice by checking whether it’s less than or equal to the length. The assertion means that if we pass an index that is greater than the length to split the slice at, the function will panic before it attempts to use that index.**

**Then we return two mutable slices in a tuple: one from the start of the original slice to the mid index and another from mid to the end of the slice.**

The `split_at_mut` function will panic, because Rust is unable to resolve the borrowing from different slices of a same source, which is why we need to introduce some `unsafe` code to compile the code.

```rs
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

Because we import `slice` from std library we also need to adjust our code in a way that we can use it. But reason why we need to use unsafe is this; "**The function `slice::from_raw_parts_mut` is unsafe because it takes a raw pointer and must trust that this pointer is valid.**"

And the reason why we can introduce `unsafe` code without a worry is this; **By looking at the code and by adding the assertion that mid must be less than or equal to len, we can tell that all the raw pointers used within the unsafe block will be valid pointers to data within the slice. This is an acceptable and appropriate use of unsafe.**

So, with that example we can note that we should only use `unsafe` code when we need to use a feature that requires `unsafe` code access.

```rs
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
```

Book introduces above example to point out, which case would crash. Reason being the code is trying to access **arbitrary** memory location, meaning we don't know if the location is even going to have any value in it. It's better to let Rust to handle memory and create variables that contain valid values than pointing to address that may or may not contain a value.

### Using `extern` Functions to Call External Code

**Sometimes, your Rust code might need to interact with code written in another language. For this, Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI). An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.**

```rs
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Functions that are declared within `extern` will always be `unsafe` because other languages don't enforce Rust's rules and guarantees. In the example we want to to use application binary interface of C language and inside the brackets we define the function we want to use from the language, specify what they take as a input and what comes out of the function.

```rs
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

With extern we can also export Rust functions to other languages. In the example we define a function with the name of `call_from_c`, just like earlier we need to specify which ABI we want to use and that should make the function executable in C language. The `no_mangle` annotation tells Rust not to mangle the name of the function.

**Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable.**

Oh and when "exporting" functions to another language we don't need to use `unsafe` keyword. Makes sense considering that we don't execute any unsafe code, but we make the code accessible from other languages.

## Accessing or Modifying a Mutable Static Variable

```rs
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

`static` variable is Rust's way of implementing global variables, which can be problematic because of how ownership works within Rust. By convention `static` variables should be named with "`SCREAMING_SNAKE_CASE`". Because static gets a fixed address in memory we probably could access it by directly accessing the address, but to be honest I don't really see real benefits of doing so, other than referencing to the address when there's no way of accessing the variable directly.

```rs
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

btw. reading or writing from `static` `COUNTER` requires the use of `unsafe` code block.

## Implementing an Unsafe Trait

```rs
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

As expected we can also make traits and implementations into `unsafe`, which doesn't really require much explaining. Actually there is one things that should be noted. 

**If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe. Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with unsafe.**

## Accessing Fields of a Union

**The final action that works only with unsafe is accessing fields of a union. A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.**

Makes sense, considering that with unions we're interacting with another language.

## When to Use Unsafe Code

Adding to what book pointed out. Unsafe code probably requires more testing to be sure that it doesn't create memory leaks or data races.