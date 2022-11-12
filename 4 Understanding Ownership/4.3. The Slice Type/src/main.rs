fn main() {
    let mut s = String::from("hello world");
    let word = first_word_without_slice(&s);
    let word = first_word(&s);
    let hello = &s[0..5]; // With slice

    // Hello prints what we need and word contains just the 
    // data where the information used to be, which means
    // the function should do more to display the word itself
    println!("{} {}", hello, word);

    s.clear();

    // If print would be here Rust would prompt an error
    // because s was cleared and the reference is unable to
    // point to the relevant bytes.

    println!("===");
    // I just want to see this part in action.
    second_main();
}

fn second_main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("{}", word);
}

fn first_word_without_slice(s: &String) -> usize {
    // Creating an array of bytes out of the s variable
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
