# Macros

Back to macros! If you remember correctly macros can be used to add functionalities in to neat little rules called macros. Because the book haven't touched into this topic that much or in detail we're going to explore it now with the book. And here's three kinds of procedural macros:

- Custom `#[derive]` macros that specify code added with the derive attribute used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens specified as their argument

## The Difference Between Macros and Functions

So, what's the difference? My guess is a macro has some limitation what they can do, because in other context macro would mean an operator that completes multiple simple tasks in a row to ease the work of a person.

**Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming**...
**For example `println!` and `vec!` macros expand to produce more code than the code you’ve written manually.**

And what does metaprogramming mean, in detail? **Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions.**

The book also details that the macro needs more complex syntax to its definitions, which is one more reason to use macros over functions when they bring more value than a function. Hmm... it seems that macros cannot be called anywhere, you need to bring them into scope and call them then, compared to functions, which can be used anywhere.

## Declarative Macros with `macro_rules!` for General Metaprogramming

Rust contains different types of macros, but out of all of the macros the declarative macro is the most used. These macros are known with different names, such as macros by example, `macro_rules!` macros and macros, good to know when interacting with other developers who may use different term. 

**At their core, declarative macros allow you to write something similar to a Rust match expression.** So, if I understood correctly the macro name functions as the match expression and if it matches the code inside of the macro is executed or the macro gets replaced by the code, because all this expression checking happens during compilation.

So how to define a macro? As "easy" as this.

```rs
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

A lot of new syntax is introduced in this. You need to remember to specify `#[macro_export]`, macro type `macro_rules!`, name `vec`, expression `( $( $x:expr ),* )` and the logic itself. That's alot to remember if you are not familiar with creating new macros.

```rs
let v: Vec<u32> = vec![1, 2, 3];
```

Oh and that example was for `vec!` macro, which is part of the Rust language.

So let's go through the highlighted parts of a macro:

- The `#[macro_export]` annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope. Without this annotation, the macro cannot be brought into the scope.
- The `macro_rules!` defines the name of the macro, after the "keyword" we're free to add the name `vec`.
- Inside the macro scope we're specifying the `match` expression. The macro could hold more, but one is enough for the example.
  - **the `$` is used to declaring a variable in the macro system that will contain the Rust code matching the pattern**
  - **the `$` makes it clear this is a macro variable as opposed to a regular Rust variable**
  - **within `$()` is `$x:expr`, which matches any Rust expression and gives the expression the name `$x`**
  - **the comma following `$()` indicates that a literal comma separator character could optionally appear after the code that matches the code in `$()`. The \* specifies that the pattern matches zero or more of whatever precedes the \***
  - **when we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three times with the three expressions 1, 2, and 3**
  - **within the `$()*` is generated for each part that matches**
- And finally the code inside macro is generated and it will replace the macro call with the generated code.

## Procedural Macros for Generating Code from Attributes

Procedural macro acts a lot like a function, but why would a developer use it over an actual function? Compared to the declarative macro the procedural macro won't match against patterns, but will produce code as an output, otherwise they are quite similar.

**When creating procedural macros, the definitions must reside in their own crate with a special crate type. This is for complex technical reasons that we hope to eliminate in the future.** Good to know if you're going to use macros more in the future.

```rs
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    ...
}
```

Defining a procedural macro isn't as complex, but it contains some new syntax and code that needs some explanation. We import the `proc_macro` to scope, which allows us to create the macro. After that we need to place a placeholder for using specific macro variety `some_attribute`, we can specify more than one attribute. The function will take `TokenStream` as an argument and will return a `TokenStream`, but what is it? The `TokenStream` is part of the `proc_macro` crate.

## How to Write a Custom `derive` Macro

Next we will explore how to define a macro using `derive`.

```rs
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

First we're bringing into our scope crates `hello_macro` and `hello_macro_derive`. **Rather than making our users implement the HelloMacro trait for each of their types, we’ll provide a procedural macro so users can annotate their type with `#[derive(HelloMacro)]` to get a default implementation of the `hello_macro` function.** Then we create Pancakes struct, which gets traits from the `hello_macro` and finally we're calling Pancakes's `hello_macro` method.

Now we need to create the library crate for the custom macros that we import in the previous example

```rs
pub trait HelloMacro {
    fn hello_macro();
}
```

And now our library is at a point which allows the developer to write their own implementation, but it introduces a new problem where the developer needs to make implementation for each type. Which would look like the following example.

```rs
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

Because it's more confinient to offer some basic implementation of trait of a library we need to do some more work. 

**Note before continuing. The next step is to define the procedural macro. At the time of this writing, procedural macros need to be in their own crate. Eventually, this restriction might be lifted!**

Let's continue our implementation by creating a new custom derive procedural macro crate called `hello_macro_derive`. Because our two macro crates are closely related to each other we would need to make changes to each one separately, but we also need to publish them separately.

Because we want the developer to be able to use either `hello_macro_derive` or `hello_macro`, we need to do some more work to the derive version, such as including some more dependencies (`syn` and `quote`).

```rs
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
```

Because the derive version is expected to use `HelloMacro` we need to add a definition for the `impl_hello_macro` function. Going into some more detail what `proc_macro` is the book explains it as follows: **The proc_macro crate is the compiler’s API that allows us to read and manipulate Rust code from our code.**

Because I'm little bit lazy at the moment and this text is quite dry I'm including the explanation for the other imported crates: **The syn crate parses Rust code from a string into a data structure that we can perform operations on. The quote crate turns syn data structures back into Rust code. These crates make it much simpler to parse any sort of Rust code we might want to handle: writing a full parser for Rust code is no simple task.**

And finally explanation why derive needs to be implemented this way or what kind of functionality we achieve with this approach: **The hello_macro_derive function will be called when a user of our library specifies `#[derive(HelloMacro)]` on a type. This is possible because we’ve annotated the hello_macro_derive function here with proc_macro_derive and specified the name HelloMacro, which matches our trait name; this is the convention most procedural macros follow.**

And finally we have the input, so what role does it hold? **The hello_macro_derive function first converts the input from a TokenStream to a data structure that we can then interpret and perform operations on. This is where syn comes into play. The parse function in syn takes a TokenStream and returns a DeriveInput struct representing the parsed Rust code.**

And now to finishing the `impl_hello_macro` function.

```rs
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

*Remember to check the book what was explained before this function*. 

`quote!` macro allows us to define the code block we want to execute. because the function is expected to return `TokenStream` we need to convert the result of `quote!` macro using `gen.into()` method, which consumes the intermediate representation and returns a value of the required `TokenSteam` type. The `quote!` macro also allows us to do more neat little things, so read some more documentation about it. `#name` is part of `quote!` macro and allows us to fetch value of a variable "name".

And now Rust should be able to compile the program.

## Attribute-like macros

Second last type of macro that we have is attribute-like macros, which I would guess utilizes attributes to modify their functionalities.

**Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes. They’re also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as functions.** Alright so I was quite a bit off on that one.

```rs
#[route(GET, "/")]
fn index() {
```

If you have read some Rust documentation of some APIs this should ring some bells, atleast it does for me. **This `#[route]` attribute would be defined by the framework as a procedural macro.**

And this is how the macro is defined.

```rs
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Quite straight forward if you ask me. But let's dive some more to the details. **The first is for the contents of the attribute: the GET, "/" part. The second is the body of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body.** Quite similar to derive macros with some difference in its syntax.

## Function-like macros

And the final one. As the title would imply this macro is just like a function in its syntax. Function-like macros are more flexible than function and are able to take in any number of arguments. But these macros are `macro_rules!` macros, which are only allowed to be defined using the match-like syntax.

```rs
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This is how they can be used.

```rs
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

And above example shows how we can define a new function-like macro.

## Summary

Macros are neat little fellows that can make the code more managable, but they aren't often used so beginners sholdn't bother too much thinking when to use create them, but it's good to know the basics how they work and what type of macros there are. It makes it easier to understand and work with other codebases and APIs.