# Publishing a Crate to Crates.io

Just like npm, crates.io functions as a platform to publish open source tools for others to use.

## Making Useful Documentation Comments

There is a specific way of documenting a crate, so it is easy to read and understand. For example documenting a functionality can be done using just two slashes `//`, like so. There is also a better one, which is meant to be used to documentation, which is why the name of this comment is *documentation comment*, which uses three slashes `///`. This comment will generate HTML documentation and whill support markdown notation, usually this kind of comment is used to provide comments **for public API items intended for programmers interested in knowing how to use your crate as opposed to how your crate is implemented**.

Fine example of a good comment is shown below.

```rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

To generate HTML documentation, just put following command to the terminal `cargo doc --open`. The template should be familiar if you have read any official Rust documentations.

## Commonly Used Sections

Scenarios that are usually shown in documentations besides "Examples":

- Panics
- Errors
- Safety

## Documentation Comments as Tests

If I understood correctly the commented example test code can be run through `cargo test --doc` command, kinda neat.

## Commenting Contained Items

There's also a specific way to provide generic comments that will be put in the frontpage of the generated documentation. If I understood correctly. This comment is `//!`, which is called *the doc comment*.

## Exporting a Convenient Public API with pub use

Based on what is more convinient in code is for some a touchy subject, but it's good to use some generic rules to help others to understand what your api/library is and how it should be used. For example importing a specific function like this `use my_crate::some_module::another_module::UsefulType;` can be quite a problematic way for users that would rather use shorter way of importing `use my_crate::UsefulType;`. Depending on how the tool was build, it might need some adjusting in order to be ready to be published, which might mean that the structure needs some tweaking.

**The good news is that if the structure isn’t convenient for others to use from another library, you don’t have to rearrange your internal organization: instead, you can re-export items to make a public structure that’s different from your private structure by using pub use. Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.**

The books includes great example how internal structure might not make sense in documentation (figures 14-3 to 14-5). But in a nutshell some times it makes sense to simplify the import method so the user can easily understand where the methods come from and where to find them, it should also reflect in the documentation. It's a lot easier to check through the front page what is possible than to go through sub pages that have other nested pages.

## Setting Up a Crates.io Account

In order to login to user account through terminal use `cargo login youruserapikeyhere` command. And remember the api key should not be shared.

## Adding Metadata to a New Crate