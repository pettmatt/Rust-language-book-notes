fn main() {
    // Yep, panic can be called like any other macro.
    // panic!("crash and burn");

    // Classic panic example, accessing an index of a vector that doesn't exist.
    // Try `RUST_BACKTRACE=1 cargo run` command to review the backtrace list.
    let v = vec![1, 2, 3];
    v[99];
}