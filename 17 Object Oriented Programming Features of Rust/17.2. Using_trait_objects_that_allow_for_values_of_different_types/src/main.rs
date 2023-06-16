use gui::{Button, Screen, Draw};

// example how the gui library could be used
// make your own compnent
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// use gui library to implement Draw
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // Finally execute run, which would trigger draw() on every component.
    // Because the draw is implemented in the selectbox the application is able to draw it.
    // If the screen compnents didn't implement draw rust wouldn't compile the application.
    // Check lib.rs if the lib implementation confuses.
    screen.run();
}
