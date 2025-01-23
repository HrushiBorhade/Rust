mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("you're added to the waitlist, Thank you for you patience");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path 
    front_of_house::hosting::add_to_waitlist();
}
