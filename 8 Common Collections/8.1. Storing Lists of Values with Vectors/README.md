# Storing Lists of Values with Vectors

## Creating a New Vector

Vectors in Rust are pretty normal compared to other langauges.

```
let v: Vec<i32> = Vec::new();
```

*Some times code examples include `<T>` in cases like explaining what vector is which in short is just a short way of telling the reader that part of the code specifies what type the value should be if it is specified.*

```
let v = vec![1, 2, 3];
```

**Book: Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.**

That's neat, so in this case that simplifies the decleration process of a vector.

## Updating a Vector

Just like in any other language, you can push values to a list of values. In Rust it can be done like this:

```
// Pretty standard stuff if you ask me.
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Rust probably can read the value type of a value that is pushed, which is why the type is not specified.

**Book: The numbers we place inside are all of type `i32`, and Rust infers this from the data, so we don’t need the `Vec<i32>` annotation.**

## Reading Elements of Vectors

Seems simple enough. Is there really need to explore how vectors are read?

```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
  Some(third) => println!("The third element is {third}"),
  None => println!("There is no third element."),
}
```

I'm not  that sure about this example. Why the value is read twice? First with simply taking reference from the list and then second time creating a variable which is expecting value of `Option<&i32>` which is then pointed to take the value of third item in `v` variable.

**Book: Note a few details here. We use the index value of 2 to get the third element because vectors are indexed by number, starting at zero. Using & and [] gives us a reference to the element at the index value. When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.**

So, without `Option<T>` the match arms wouldn't work?

**Book: The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements. As an example, let’s see what happens when we have a vector of five elements and then we try to access an element at index 100 with each technique**

Answer is that Rust is going to panic, because that's an impossible task.

**Book: When the get method is passed an index that is outside the vector, it returns None without panicking.**

Oh so that's why the panicking was brought up. In that case using `match` is better idea than directly assuming that a list contains x amount of items all the time.

*Remember that if immutabtable variable borrows a value from mutatable variable the origin of the value cannot change! Compiler would catch this error.*

```
Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("The first element is: {}", first);
  |                              ----- immutable borrow later used here
```

Just a reminder to self. This is extremely simplified version of the contents of the book. Which is why I'm including the reasoning to why the error above is prompted.

**Book: The code in Listing 8-6 might look like it should work: why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.**

## Iterating over the Values in a Vector

Let's see what kind of specifics iteration contains.

```
let v = vec![100, 32, 57];
for i in &v {
  println!("{i}");
}
```

Looks pretty standard way of iterating numbers, integers or i32 values. 

```
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
```

And if we wanted to make changes it would look like that. Reference becomes `&mut`. But what `*` operator means in this case? Oh, right it's used to dereference the value.

**Book: To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.**

## Using an Enum to Store Multiple Types

So, vector type doesn't allow storing different types within it, which is why in Rust developers need to use a different type of a list to store different values. Which in this case is enum.

**Book: We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum. Then we can create a vector to hold that enum and so, ultimately, holds different types.**

```
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12),
];
```

As the book have noted, the reason why vector can include values of an enum is because they are processed as the same type, which is enum. And the reason why vector needs to know the type at compile time goes as followed:

**Book: Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled ...**

And then there's an exception...

**Book: If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object**

For more in depth look check [the API documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html).

## Dropping a Vector Drops Its Elements

It's simple to put it simply.

```
{
  let v = vec![1, 2, 3, 4];

  // do stuff with v
} // <- v goes out of scope and is freed here
```

If something goes out of scope it's **gone**. This also means that borrow check ensures that references are used while the source is valid.

Next up **String**.