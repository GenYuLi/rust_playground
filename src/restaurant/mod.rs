
mod meat {
    pub mod steak {
        pub struct Steak {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            // static t: Breakfast = Breakfast {
            let t = Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
            return t;
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    /*let mut meal = back_of_house::Breakfast{
        toast: String::from("Wheat"),
        seasonal_fruit: String::from("Apple"),
    };*/
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
}

use super::Asparagus;

pub fn eat() {
    let plant;
    let steak = meat::steak::Steak {};
    let meat = crate::restaurant::meat::steak::Steak {};
    plant = Asparagus {};
}