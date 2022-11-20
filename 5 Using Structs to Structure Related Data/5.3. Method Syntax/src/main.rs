#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Area function has been moved inside of the Rectangle
// which makes it a method of rectangle struct.
// To be more precise "impl" makes the function associated with rectangle.
// Note that because the function is a method (of the struct) it has access to the rectangle values
// which is why the argument is referenced as "&self".
// Note also "Methods must have a parameter named self of type Self for their first parameter".
// Also also methods that take ownership of "self" is a rare occurance.
// impl = implementation (Yes! My guess was right)
// Functions that are included like this are called associated methods, but if they don't include
// self as the first parameter they are not methods. Why? They don't need instance of the type to work.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Little test. Can we implement more than one method? Answer is yes, ofcourse we can.
    // It's good to note that Rust doesn't mix up fields to methods that are named the same.
    // Here we've implemented a width method, the same name is used in a field... [1]
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle ) -> bool {
        self.area() > other.area()
    }

    // This is a method that can be used without instance
    fn square(size: u32) -> Self {
        Self { // <-- Alias of the type
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // [1] ... and here as expected Rust doesn't mix up methods with fields, because it's expected that "width()"
    // references to method where ".width" references to a field.
    if rect1.width() {
        // If we wanted to make sure that the contents of a struct have read-only access
        // we could create getters that return wanted values. Ofcourse this means that the
        // fields need to be changed to be private, only accessable through methods of the struct.
        println!("Width is: {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Function without instance: {:#?}", Rectangle::square(5))
}
