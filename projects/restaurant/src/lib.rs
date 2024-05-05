#[allow(dead_code)]

mod front_of_house {
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

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // // Abs path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Rel path
    // front_of_house::hosting::add_to_waitlist();

    // With use
    hosting::add_to_waitlist();
}
