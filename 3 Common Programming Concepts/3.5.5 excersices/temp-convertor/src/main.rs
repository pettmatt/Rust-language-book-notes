use std::io;

fn main() {
    println!("Please enter temperature to convert");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line!");

    let temp: isize = temp
        .trim()
        .parse()
        .expect("Given temperator was not a number!");

    println!("To which do you want to convert?");
    println!("1 = Celsius");
    println!("2 = Fahrenheit");
    let mut temp_type = String::new();

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Oops. Something happend!");
    
    let temp_type: usize = temp_type
        .trim()
        .parse()
        .expect("The input was not a number!");

    // For what ever reason, when I was trying to use strings
    // to determine which type the user wants to convert to
    // the if-statement was only able to go through else
    // even when the strings contained the same characters
    // and they had the same type.

    if temp_type == 1 {
        let temp = fahrenheit_to_celsius(temp);
        println!("{} C", temp);
    } else if temp_type == 2 {
        let temp = celsius_to_fahrenheit(temp);
        println!("{} F", temp);
    } else {
        println!("Seems like someone wants to be a comedian here.");
        println!("Give me a second. I will read your mind to find out what you want.\n");
        println!("There! You want to convert fahrenheit to celsius!");

        let temp = fahrenheit_to_celsius(temp);
        println!("Here's your readable temperature \"{}\"", temp);
    }
}

fn celsius_to_fahrenheit(temp: isize) -> isize {
    let temp = temp * 2 + 30;

    return temp;
}

fn fahrenheit_to_celsius(temp: isize) -> isize {
    let temp = (temp - 30) / 2;

    return temp;
}