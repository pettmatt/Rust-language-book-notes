# Storing Keys with Associated Values in Hash Maps

The type `HashMap<K, V>` can be used as a list of things but every value also has a key, which can be pointed to in order to get the value of that key.

## Creating a New Hash Map

Hash map is included within the stantard library (`std`), which makes it easy to use within any project.

```rust
// Declaring a hash map
use std::collections::HashMap;
let mut scores = HashMap::new();

// Insert values to a hash map using the "insert" method
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Because hash maps are included in `collections` and also less often used, they tend not to have the same kind of support compared to other more popular types and methods.

**Book: Hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.**

It's good to note that `HashMap` follows the same kind of logic with `vectors`. The keys and values within the same hash map has to have the same types, keys need to be the same and values needs to be the same type than the other values.

## Accessing Values in a Hash Map

Just like inserting values to a hash map, using those values are as easy. `get` method can be used with a key value to **get** a value, for example.

```rust
let score = scores.get("Yellow").copied().unwrap_or(0);
```

*Note that a string of a `str` type doesn't need to be passed as a reference using `&`.*

The explonation of that code snippet is quite extensive so I'm not trying to explain it myself.

**Book: The get method returns an `Option<&V>`; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an `Option<i32>` rather than an `Option<&i32>`, then `unwrap_or` to set score to zero if scores doesn't have an entry for the key.**

Iterating can be done basically the same way as with vectors.

```rust
for (key, value) in &scores {
  println!("{key}: {value}");
}
```

## Hash Maps and Ownership

Hash map is quite logical when it comes to the ownership. Using the `Copy` trait `i32` for an example, the values are copied into the hash map without removing ownership from the original variable. When it comes to `String`, the values will be moved into the hash map and with those values the ownership switches to the hash map.

Because the ownership moves to a hash map, the use of the original values is impossible if the value was for example a `String`. Check the `main.rs` file in scoreboard directory.

## Updating a Hash Map

Updating the hash map isn't that complicated, but there is probably some weird interactions when dived deeper. Anyway, hash map values can be updated, but they cannot hold more than one value at a time, the value can be completely replaced, ignored or combined with the previous one.

## Overwriting a Value

If the same key is used to insert a new value it overwrites the previous one. That's about it. Even if the value is overwritten right away it still functions the same way, the old value is overwritten.

```rust
// This functions the same way as...
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

// this
scores.entry(String::from("Yellow")).or_insert(50);
// Do something useful with the "Yellow" value before overwriting.
scores.entry(String::from("Blue")).or_insert(50);
```

*Note that `str` is not acceptable type in hash maps. It needs to be converted to a `String`.*

## Adding a Key and Value Only If a Key Isn't Present

Well the header speaks for itself. This can be done through a special API which takes a key as a parameter and checks if the key is in use. The api is called `entry`.

**Book: The return value of the entry method is an enum called Entry that represents a value that might or might not exist. Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for the Blue team.**

```rust
// Will insert the value if key is not found
scores.entry(String::from("Yellow")).or_insert(50); 
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

As you can see the `entry` is pared with a method called `or_insert` (my hypotysis without reading the answer) which inserts the given value if the entry returned value is `true`.

According to [doc.rust-lang.org](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) the method functions as follows; **Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.**

Addition to that `Entry` either returns an `Occupied` or a `Vacant` as a response.

## Updating a Value Based on the Old Value

In some cases there are moments when the value needs to be updated based on the old value. For example I would use the `for` loop which needs a point of reference how many times it has iterated, but this example isn't that useful in this case soooo look at the code snippet.

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}

println!("{:?}", map);
```

*Wait, is `*count += 1` really enough to update the hash map value? Seems too simple of a syntax. **Testing noises**. Seems like it is all it needs, neat.*

**Book: You might see the same key/value pairs printed in a different order: recall from the “Accessing Values in a Hash Map” section that iterating over a hash map happens in an arbitrary order.**

Hmmm... interesting. So the order changes every time I run the program.

**Book: The split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the value in text. The or_insert method returns a mutable reference (&mut V) to the value for the specified key. Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.**

That explonation is extremely useful. I already forgot what the `*` operator does.

## Hashing Functions

This section is dedicated to explaining what hash maps include to offer as secure environment as possible with hashing. That's about it.

## Summary

Hash maps are another way to store data in Rust. In some cases they offer enough flexibility, but for some cases they are not valid for the use case. And that's the final chapter in **Common collections**. Next up **Error handling!**