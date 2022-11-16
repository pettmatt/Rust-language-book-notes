# Defining and Instantiating Structs

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