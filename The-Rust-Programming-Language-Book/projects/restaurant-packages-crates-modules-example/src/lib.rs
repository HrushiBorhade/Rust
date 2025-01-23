fn random_function() {}

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

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit : String
    }

    impl Breakfast {
        pub fn summer(toast :&str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

}


pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path 
    front_of_house::hosting::add_to_waitlist();
    let mut  breakfast = back_of_house::Breakfast::summer("Wheat");
    breakfast.toast = String::from("Rye");
    println!("breakfast menu: {breakfast:?}");
}
