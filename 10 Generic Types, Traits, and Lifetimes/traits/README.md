# Traits: Defining Shared Behavior

Traits as the word can describe defines the functionality of a type. Think of it like traits in video games, they are usually skills that are given to characters to give them specific functionality.

*In other languages traits are usually called "interfaces".*

## Defining a Trait

Defining a trait is as easy as this.

```Rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Snippet from the book:

**We declare a trait using the trait keyword and then the trait’s name, which is Summary in this case. We’ve also declared the trait as pub so that crates depending on this crate can make use of this trait too, as we’ll see in a few examples. Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait, which in this case is fn summarize(&self) -> String.**

## Implementing a Trait on a Type

Because traits are meant to be implemented next thing would be to integrate it to an existing struct. Which can look like this:

```Rust
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

And now we can just use these pre-defined structs and the trait in our `main.rs` file with `use aggregator::{Summary, Tweet};`. Even if the code doesn't include any references to `Summary` we need to include it in our import, because Tweet is dependent on it. 

*If the library file is created manually you probably need to include new configurations in the toml file. If you don't include the details for the library there is high chance of seeing following error:*

```rs
 --> src/main.rs:1:5
  |
1 | use aggregator::{Summary, Tweet};
  |     ^^^^^^^^^^ use of undeclared crate or module `aggregator`
```

*Additionally to this, if you don't include the dependencies (`Summary`) in the use `use` import, you will receive following error:*

``
error[E0599]: no method named `summarize` found for struct `Tweet` in the current scope
``

## Default Implementations

Sometimes it's useful to have some default behavior so the programmer wouldn't need to add the "basic" functionalities that would be expected the library to add. Code below makes sure to return "Read more" by default whenever summarize is called, which can be changed by overriding it in the implementations.

```rs
pub trait Summary {
    fn summarize(&self) -> String; // This line would change into...
    // ...
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Summary could contain more functions than just one. In the code snippet below we need to make sure the implementations of this trait include their own implementations of `summary_author`, which means they need to override it in order to use the Summary trait. Summarize doesn't need to be overriden because it has default value(s) / default behavior.

```rs
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

## Traits as Parameters

Implementing traits through parameters isn't much of a scretch, the function that it is implemented just needs to support any type that implements Summary.

Book snippet about the code:

*Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass in any instance of NewsArticle or Tweet. Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.*

```rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Trait Bound Syntax

If we want to dive deeper about the implementation in parameter, the `ìmpl Summary` is just... as the book puts it *"syntax sugar" for longer form known as a trait bound.* And what is trait bound? Well it looks like this...

```rs
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

It's generic type in disguise, which is expected to be Summary. To be honest I like this "bare bones" implementation more. And now let's put them side by side and check how they look when there is multiple parameters!

```rs
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// vs
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
// I think the latter is better way of implementing, personally.
```

### Specifying Multiple Trait Bounds with the + Syntax

So, what does the title really mean? In simpler terms, how to specify that the implementation must use specific traits. In example below the notify function can call `summarize` and use `{}` to format `item`.

```rs
pub fn notify(item: &(impl Summary + Display)) {}

pub fn notify<T: Summary + Display>(item: &T) {}
```

### Clearer Trait Bounds with where Clauses

The previous example of implementing multiple trait bounds is not that recommended when the function needs more than two traits. In those cases there is another syntax that can be used.

```rs
// Instead of this...
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// Try this...
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```

## Returning Types that Implement Traits

And of course we can return a value of a type that implements a trait...

```rs
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

To be honest it was sounding more complicated, but this can be used to make a function that creates the type with the trait without the need of writing the structure multiple times, which can result in errors.

*By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait without naming the concrete type. In this case, returns_summarizable returns a Tweet, but the code calling this function doesn’t need to know that.*

*However, you can only use impl Trait if you’re returning a single type. For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work:*

```rs
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

What... but why? It doesn't sound like it breaks any Rust rules...

*Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler. We’ll cover how to write a function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.*

Fine.. I guess.

## Using Trait Bounds to Conditionally Implement Methods

Read from the source: https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax