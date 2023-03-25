fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let result = largest_number(&list);
    println!("The largest number is {}", result);
    
    let list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_number(&list);
    println!("The largest number is {}", result);
}

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}