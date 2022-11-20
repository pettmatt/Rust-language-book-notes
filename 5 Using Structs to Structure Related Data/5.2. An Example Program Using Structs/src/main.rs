// This is outer attribute which in this case enables debug "mode".
// In this case it means that println macro can use debug trait to display the contents.
// There's more traits to be exploited https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // This might not be the most readable code, in the sense that
    // There is no way to tell how width1 and height1 is going to be used
    // It would be better to group them like you would group coordinates
    // let width1 = 30;
    // let height1 = 50;

    // Refactoring with tuples
    // Refactoring and using the right variable types is up to the person who
    // writes and upkeeps the program. For some tuples would be more confusing in the long run.
    // let rect1 = (30, 50);

    // Refactoring with Structs
    // Easier to read, but this may not be the best approach in every situation,
    // because this approach needs more setting up and doing this for something 
    // small would probably be too much work for something that would not be used, 
    // but a few times.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Structs are handled differently in println macro, which is why it's unable
    // to print out the contents (too many variables to choose from how it should be displayed).
    // By adding ":?" inside the brackets the macro will use debug trait 
    // (which means the macro will print the variable in a way that makes sense in development).
    // Using ":#?" formats the output a little bit prettier, which comes handy with larger variables.
    // println!("Original values: {:#?}", rect1);

    // Another way of printing out values for more complicated variables
    // can be done using another macro called "dbg!". Note that this
    // macro can take over the ownership which is why the value should be
    // passed on as reference (there's probably cases where you would want to give the ownership to it).
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        // area_normal(width1, height1)
        // area_tuples(rect1)
        area_structs(&rect1) // <-- reference used. otherwise the ownership would be moved
    );
}

fn area_normal(width: u32, height: u32) -> u32 {
    width * height
}

// Function takes an argument "dimensions" which 
// is tuple group which contains two u32 values.
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    // Refering to numbers instread of width and height may not be the best
    // approach for some people.
    dimensions.0 * dimensions.1
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    // Easier to track which value is which
    rectangle.width * rectangle.height
}