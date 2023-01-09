// "Root" functions of a module don't need public status, because they are siblings.
mod front_of_house {
    // Rust is unable to find a module if the module is NOT "public".
    // Fixed with pub keyword.
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Wow! It's in italic that's super...
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        // If we wanted to make seasonal_fruit public we could change it here.
        // But it would 
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // REMEMBER: Crate keyword is usable if the function is defined within the file. 
    // (It could probably be read as "this", in context of paths)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // In this case the path uses the same logic, but it doesn't need the keyword "crate",
    // because it uses this file as the root of its "relative source"
    // (because this file is the file where the function was defined).
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}