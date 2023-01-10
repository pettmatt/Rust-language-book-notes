mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    // In order to work, the path should be moved inside the scope
    // use crate::front_of_house::hosting;
    // With super it would look like this
    use super::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}