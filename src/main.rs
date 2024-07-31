// same level, but if you want to use the lib in the main.rs, you need to use the crate name
// Because the main.rs is not in the same module as the lib.rs

use rust_playground::garden::vegetable::Asparagus;
use rust_playground::fountain::water::Music;
use rust_playground::eat;
use rust_playground::practice_syntax::loop_test;

fn main() {
    let plant = Asparagus {};
    loop_test::loop_test();
    println!("I'm growing a {:?}! There is some {:?}.", plant, Music {});
    eat();
}
