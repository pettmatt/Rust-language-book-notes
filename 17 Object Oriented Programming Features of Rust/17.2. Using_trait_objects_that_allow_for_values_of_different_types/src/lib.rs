// Trait object
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Vector of a box type, which implements Draw
    pub components: Vec<Box<dyn Draw>>,

    // Second way of implementing Draw to components
    // pub components: Vec<T>
}

// Implementing to existing struct
// Which can also be implemented another way
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implement to generic type Screen<T>, where Draw is implemented
// impl<T> Screen<T>
// where T: Draw, {
//     pub fn run(&self) {
//         for compnent in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// BOOK
// This restricts us to a Screen instance that has a list of components all of type 
// Button or all of type TextField. If you’ll only ever have homogeneous collections, 
// using generics and trait bounds is preferable because the definitions will be 
// monomorphized at compile time to use the concrete types.

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Potentially Button could have an implementation of functionalities
// impl Button {
//     fn click(&self) {
//         // Functionality would go here
//          println!("Click event triggered");
//     }
// }

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button, which is outside of the scope of this exercise
    }
}