fn main() {
    // This example uses only one "real" variable value, which is then
    // used as a refereence in creating new variables.
    let s1 = String::from("hello");

    let mut len = calculate_length(&s1); // <-- Ref

    let _mut_len = add_to_length(&mut len);

    println!("The length of '{}' is {}.", s1, len);
    println!("New length: {}", _mut_len);
    println!("Old length: {}", len);
}

fn calculate_length(s: &String) -> usize { // <-- value is passed as a ref
    s.len()
}

fn add_to_length(s: &mut usize) -> usize {
    // By dereferencing the reference the calculation can be executed
    *s + 3
}