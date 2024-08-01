// same level, but if you want to use the lib in the main.rs, you need to use the crate name
// Because the main.rs is not in the same module as the lib.rs

use rust_playground::garden::vegetable::Asparagus;
use rust_playground::fountain::water::Music;
use rust_playground::restaurant::eat;

fn main() {
    // you can declare a variable without assigning a value
    let plant: Asparagus;
    // and then assign a value later
    plant = Asparagus {};
    println!("I'm growing a {:?}! There is some {:?}.", plant, Music {});
    eat();
}
