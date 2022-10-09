fn main() {
    let number = 3;

    example_one(number);
    example_three();
}

fn example_one(number: i32) {
    println!("If statement");

    if number > 5 {
        println!("number is bigger than 5");
    } else if number != 0 {
        println!("number was something other than zero");
    } else {
        println!("condition was false");
    }

    example_two(number);
}

fn example_two(parameter: i32) {
    // Arm values should return the same type.
    // Variables should have ONE declared type. Here, it is integer.
    let number = if parameter != 0 { 1 } else { 10 };
    println!("Value of number is: {number}");
}

fn example_three() {
    println!("Loops");

    let mut count = 0;

    loop {
        println!("Looping around around around and around. {count}");

        if count >= 2 {
            println!("Until the fun stops after 3 loops");
            break;
        }

        count += 1;
    }

    while count != 0 {
        println!("{count}");
        println!("Short while loop. Loop count {count}");
        count -= 1;
    }

    println!("and now we're out of that loop");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("and now to the for loop");

    for element in a {
        println!("the value is: {element}");
    }

    println!("and final test subject is... countdown example");

    // rev = reverse the range. not necessary, but looks nice.
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("and we are done here!");
}