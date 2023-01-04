// Mod defines a module
mod front_of_house {
    // Here we have submodule
    mod hosting {
        // Here's some functionalities that are unique to this module
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    // And another submodule
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}