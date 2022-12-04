// Import as module from another file. Module name to match file name.
mod other_module;

mod front_of_house {

    fn test_parent_function() {}

    pub mod hosting {
        pub fn add_to_waitlist() {
            super::test_parent_function();
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }

    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serving::serve_order();
        }

        fn cook_order() {}
    }
}

// Abs use path - Idiomatic for functions
use crate::front_of_house::hosting;
// Relative use path -- Idiomatic for structs and enums
use front_of_house::hosting::add_to_waitlist;

fn eat_at_restaurant() {
    // Abs Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    // From 'use' abs path  in scope
    hosting::add_to_waitlist();
    // From 'use' rel path  in scope
    add_to_waitlist();

    other_module::test();
}
