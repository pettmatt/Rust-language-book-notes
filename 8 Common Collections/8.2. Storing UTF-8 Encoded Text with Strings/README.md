# Storing UTF-8 Encoded Text with Strings

Deeper dive to how `string` type functions in Rust.

**Book: New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8.**

In other words, study seemingly easier topics too! Something that should be pointed out, a string is implemented as **collection of bytes** which makes it a **collection**. 

## What Is a String?

`str`, `&str` and `String` are not the same value. `str` is the only core string type, which stores string literals in the program's binary and are therefore string slices. `&str` is borrowed `str`. `String` is provided by Rust's standard library, which means it's not part of the core language, but rather an extension. When someone talks about strings they might mean both, but it's good to understand that there are some differences between the string types.

## Creating a New String

**Book: Many of the same operations available with `Vec<T>` are available with `String` as well, because `String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. An example of a function that works the same way with `Vec<T>` and String is the `new` function to create an instance.**

```
let mut s = String::new();
```

Some examples that show seemingly weird, but logical functionality how string type works.

Converting to/creating a string, `to_string` is available for any type that implements the `Display` trait:

```
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

Creating a string using `String::from`:

```
let s = String::from("initial contents");
```

*`to_string` and `String::from` do the same thing with their own way.*

**Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them**

Does that mean that Rust supports special characters out of the box?! Finally a language that supports other than Latin characters.

## Updating a String

String doesn't cause issues if its contents change and just like `Vec<T>` the value can be changed using `+` operator or the `format!` macro.

Appending to a String with push_str and push

```
let mut s = String::from("foo");
s.push_str("bar")
```

Because the `push_str` method doesn't affect the ownership of variables we can append a value and use it after it.

```
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

Additional to `push_str` we can push a single character using `push` method.

```
let mut s = String::from("string");
s.push('!');
```

Concatenation with the `+` Operator or the `format!` Macro. 

```
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

**Book: The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator. The + operator uses the add method, whose signature looks something like this:**

```
fn add(self, s: &str) -> String {
```

*So does that mean the first value has to be always concumable when `+` operator is used on a string?*

**Book: we can only add a &str to a String; we can’t add two String values together.**

Oh... well that is bothersome.

**Book: But wait—**

Aaaah. Here we go again.

**Book: the type of &s2 is &String, not &str, as specified in the second parameter to add.**

Why would it be? It's defined with `String::from` which should use `String` or am I forgetting something here? Oh, right. The `add` function is expecting `&str`, but isn't it the same as `String` at the end of the day?

**Book: The reason we’re able to use &s2 in the call to add is that the compiler can coerce the `&String` argument into a `&str`. When we call the add method, Rust uses a deref coercion, which here turns `&s2` into `&s2[..]`. We’ll discuss deref coercion in more depth in Chapter 15.**

Book dropping some spicy facts over here.

**Book: Second, we can see in the signature that add takes ownership of self, because self does not have an &. This means s1 in Listing 8-18 will be moved into the add call and will no longer be valid after that.**

I knew it! *(Self loath: Really? Proud of yourself for understanding the basics?)*

**Book: So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.**

Concatenation multiple values with the `+` Operator or the `format!` Macro. 

```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// Using a format macro is probably better here.
let s = format!("{s1}-{s2}-{s3}");
```

*So, the question is if the `+` operator takes the previous value and concatenates the next value to it.*

**Book: This code also sets s to tic-tac-toe. The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents. The version of the code using format! is much easier to read, and the code generated by the format! macro uses references so that this call doesn’t take ownership of any of its parameters.**

Neat, if I was to use macro instead the value wouldn't be consumed.

## Indexing into Strings

**Book: In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.**

And that means following

```
let s1 = String::from("hello");
let h = s1[0]; // <-- this would create an error.
```
And the error message would look like this.

```
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
  = help: the following other types implement trait `Index<Idx>`:
            <String as Index<RangeFrom<usize>>>
            <String as Index<RangeFull>>
            <String as Index<RangeInclusive<usize>>>
            <String as Index<RangeTo<usize>>>
            <String as Index<RangeToInclusive<usize>>>
            <String as Index<std::ops::Range<usize>>>
            <str as Index<I>>
```

To the explanation!

### Internal Representation

In Rust every latin character is worth of one byte which means a string of `test` would have length of four bytes. Unfortunately, some characters are not created equal in Rust. For example Cyrilics contain multiple latin characters which means `Здравствуйте` which contains 12 cyrilic characters contains 24 bytes worth of characters. This is why finding a character based on index value doesn't really work as expected in Rust (compared to other languages).

**Book: You already know that answer will not be З, the first letter. When encoded in UTF-8, the first byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, but 208 is not a valid character on its own. Returning 208 is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0. Users generally don’t want the byte value returned, even if the string contains only Latin letters: if `&"hello"[0]` were valid code that returned the byte value, it would return `104`, not `h`.**

*Interesting to say the least. So it's easier to not allow indexing in strings when the behavior isn't consistant across every characters. Is this why some languages just don't support other than latin characters?*

### Bytes and Scalar Values and Grapheme Clusters! Oh My!

There are three different (relevant) ways of looking into strings in Rust's perspective:
- Bytes
- Scalar values
- Grapheme clusters (comparable to letters)

**Book: If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:**

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

And that's how Rust stores 18 bytes. As Unicode scalar values, which are Rust's `char` types are, the bytes would look like following:

```
['न', 'म', 'स', '्', 'त', 'े']
```

18 bytes turn into 6 characters which are `char` values.

**Book: ... but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:**

```
["न", "म", "स्", "ते"]
```

And why do we need that many ways of reading the characters?

**Book: Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.**

**A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.**

Reasonable justifications.

## Slicing Strings

If you need to index the string it's recommended to give more information to Rust. What it means in action is to give a range of bytes that the variable should contain.

```
let hello = "Здравствуйте";
let s = &hello[0..4];
// s gets the value of Зд, because cyrilic characters are worth 2 bytes
```

Because characters are valued differently, if above example would be done using the range of `[0..1]` Rust would panic, but it would work with latin characters. So if slicing/indexing is needed it should be done within context and caution.

## Methods for Iterating Over Strings

**Book: The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes. For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:**

```
// The iteration will print every character in the string
for c in "Зд".chars() {
  println!("{c}");
}
```

Because strings can be viewed with three different ways the same iteration with bytes would look like this:

```
// Will print the byte values. example 208, 151 and so on.
for b in "Зд".bytes() {
  println!("{b}");
}
```

Good to point out this part from the book.
**Book: Getting grapheme clusters from strings as with the Devanagari script is complex, so this functionality is not provided by the standard library. Crates are available on crates.io if this is the functionality you need.**

## Summary

The point of this chapter was that strings are not that simple of a concept which is why developers should respect them and pay attention how they function in Rust and that Unicode scalar values (characters that people typically read) can be valued differently and `1 byte === 1 character` isn't always true. These restrictions in Rust are there to help the developers to find problematic code early which should make developing easier in the long run.

Read about following methods `contains` and `replace`.