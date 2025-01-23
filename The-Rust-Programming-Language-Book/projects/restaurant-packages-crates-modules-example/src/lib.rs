use std::collections::HashMap;

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

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad
    }
}

use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1,2);
    // relative path 
    front_of_house::hosting::add_to_waitlist();
    let mut  breakfast = back_of_house::Breakfast::summer("Wheat");
    breakfast.toast = String::from("Rye");
    println!("breakfast menu: {breakfast:?}");

    let mut appetizer_order1 = back_of_house::Appetizer::Soup;
    let mut appetizer_order2 = back_of_house::Appetizer::Salad;

    println!("appetizer order 1 : {appetizer_order1:?}");
    println!("appetizer order 2 : {appetizer_order2:?}");
}
