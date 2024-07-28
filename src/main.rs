// same level, but if you want to use the lib in the main.rs, you need to use the crate name
// Because the main.rs is not in the same module as the lib.rs

use backyard::garden::vegetable::Asparagus;
use backyard::fountain::water::Music;
use backyard::eat;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing a {:?}! There is some {:?}.", plant, Music {});
    eat();
}
