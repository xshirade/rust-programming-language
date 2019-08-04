mod front_of_house;

// use crate::front_of_house::hosting;
// re-exporting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    {
        // Absolute path
        crate::front_of_house::hosting::add_to_watilist();

        // Relative path
        front_of_house::hosting::add_to_watilist();
    }

    {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        // meal.seasonal_fruit = String::from("blueberries");
    }

    {
        let o1 = back_of_house::Appetizer::Soup;
        let o2 = back_of_house::Appetizer::Salad;
    }

    {
        hosting::add_to_watilist();
        hosting::add_to_watilist();
        hosting::add_to_watilist();
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}
