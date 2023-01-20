fn main() {
    // Declaring a hash map
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    // Insert values to a hash map using the "insert" method
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get("Yellow").copied().unwrap_or(0);
    println!("The score of yellow team is {}.", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Cases when an error is prompted
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Not the cleanest code, but if str is used it needs to be converted into a String.
    map.insert("Favorite color".to_string(), "yellow".to_string());
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //println!("Error test {field_name}"); // Ofcourse the use of the variables is impossible if their ownerships were also moved.
    println!("Overwrite test: {:?}", map);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        println!("{}", count);
        *count += 1;
    }

    println!("{:?}", map);
}
