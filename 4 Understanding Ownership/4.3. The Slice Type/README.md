# 4.3. The Slice Type

Slicing can offer an easy solution to finding specific data/byte from variables without the need to create multiple variables just to track the original position of a word inside of an array, for an example.

*I know. Somewhat badly explained, but it will do for now. Good luck future me!*

## String Slices

**Book:** *A string slice is a reference to part of a String, and it looks like this:*

```
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];
```

So what benefits does this have? In plain language, slicing offers surprisingly easy way to reference to a specific part of an variable. Think of it as taking a note from history book, you would probably want to memorize when 100 years of war (Hundred Years' War) happend and how long it was, because those parts are more likely to be in an exam (and more relevant in that sense).

So let's go through the slice structure `let world` is slice variable that stores the value of `&s[6..11]`, which is reference of s variable (`&s`), which points to the byte at index of 6 with a length of 5 (`[6..11]`). Quite effective way of pointing to a relevant data.

Small note about `..` range syntax. `[0..2]` and `[..2]` are both acceptable ways of pointing to 0 and taking the next 2 values.

With the same logic as above the remaining values from a variable can be pointed to with `[3..len]` and `[3..]`. It's more about what the writer thinks is more informative/readable in the future.

Finally if you want to point to the entire variable, it can be done with following syntax `[0..len]` and `[..]`. Neat isn't it?

## Other Slices

There are other types of slices that reflect to what they sliced (or borrowed) their information from. End of story, gotta go. 