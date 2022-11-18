## Defining and Instantiating Structs

Struct is pretty simial with tuples in a way that they both can hold a collection of values. Tuple reminds more of an array with its `let tup: (i32, f64, u8) = (500, 6.4, 1);` structure. Struct is more organized and could be more flexible and better option in some cases.

```
// Struct structure example
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
```

Because of the structure, struct (to me) reminds class constructors. The use style also reminds of classes.

```
let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  active: true,
  sign_in_count: 1,
};
```

To access a value inside of struct you need just use dot notation, like this `user1.username`. Overriting a value can be done through dot notation and ONLY IF the struct is mutable.

```
let mut user1 = User {
  ... // Some data
};

user1.email = String::from("anotheremail@example.com");
```

Construct new instance of a struct with a function.

```
fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}

```

## Using the Field Init Shorthand

Fancy terms nothing special or really complicated stuff. Probably need to memorize the term for future. Anyway what this section means in practice is "this is how you can write even cleaner code" by removing unnecessary parts such as the `email` and `username` values when the attribute and value share the same name.

```
fn build_user(email: String, username: String) -> User {
  User {
    email,        // <-- changed
    username,     // <-- changed
    active: true,
    sign_in_count: 1,
  }
}

```

*Note: It's cool that a book about deeper level programming (deeper than JavaScript) goes through even the smallest features like in this chapter.*

## Creating Instances From Other Instances With Struct Update Syntax

When you want to have a new instance of a struct, but chance some values you may want to use *struct update syntax*.

Example without struct update syntax:
```
let user2 = User {
  active: user1.active,
  username: user1.username,
  email: String::from("another@example.com"),
  sign_in_count: user1.sign_in_count,
};
```

Example with struct update syntax, which does the same thing as the example above, but it's shorter way of updating/chancing values.

```
let user2 = User {
  email: String::from("another@example.com"),
  ..user1
};
```

I know I know. I'm in the same boat if you think the same way. `..` operator is scary syntax, but it's also extremely useful, like here. Change the value you want and copy everything from existing struct instance. Kinda funny that `..user1` doesn't override the email value, but this kind of thinking just shows what kind of environment/language I'm personally used to. Remember future me, `..` syntax specifies just the values that have not been specified before the syntax comes to action.

*Note: When updating values the order doesn't matter. BUT the `..` syntax needs to come after the specified values.*

*Note: After creating `user2` the `user1` becomes unusable, because `user2` takes over the unique values from `user1` (username, email). If those values were different the `user1` would be still usable.*

## Using Tuple Structs without Named Fields to Create Different Types

*Note: Oh god. What were some people thinking. Isn't it enough that Rust has tuple and struct... why did they need to mix them together and create `tuple structs`? To be fair the explonation makes some sense, but I am unable to think of a situation when I would need it.*

The example:

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
```

*Note: With that example I have clearer image of how tuple structs could be used.*

Every struct type in itself is a different type, which is why `Color` cannot take `Point` as an argument even if their tuples are made with the same types.

## Unit-Like Structs Without Any Fields

*Note: too lazy to write an example so book can do that.*

*Book: You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section.*

```
struct AlwaysEqual;

fn main() {
  let subject = AlwaysEqual;
}
```

*Book: To define AlwaysEqual, we use the struct keyword, the name we want, then a semicolon. No need for curly brackets or parentheses! Then we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses. Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior! You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.*