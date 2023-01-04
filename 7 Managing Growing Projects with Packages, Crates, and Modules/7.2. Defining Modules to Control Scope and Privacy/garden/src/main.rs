// Declaring a shortcut to a path
use crate::garden::vegetables::Asparagus;

// Changing privacy of garden.
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}