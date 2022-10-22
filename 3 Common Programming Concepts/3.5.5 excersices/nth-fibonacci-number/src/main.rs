use std::io;

fn main() {
    println!("Give any positive number");

    let mut fab_num = String::new();

    io::stdin()
        .read_line(&mut fab_num)
        .expect("Failed to read line!");

    let fab_num: usize = fab_num
        .trim()
        .parse()
        .expect("Given number was not a number!");

    let fab_num = fib(fab_num);

    println!("{}", fab_num);
}

fn fib(number: usize) -> usize {
    if number <= 1 {
        return number;
    }

    return fib(number - 1) + fib(number - 2);
}