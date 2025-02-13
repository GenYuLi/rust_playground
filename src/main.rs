// same level, but if you want to use the lib in the main.rs, you need to use the crate name
// Because the main.rs is not in the same module as the lib.rs

use std::collections::BTreeMap;

use rust_playground::garden::vegetable::Asparagus;
use rust_playground::fountain::water::Music;
use rust_playground::restaurant::eat;
use anyhow::{Context, Error, Result};

fn test() -> Result<()> {
    match Err::<(), Error>(Error::msg("warn")) {
        Ok(_) => (),
        Err(t) => println!("lol[{:?}]", t),
    }
    Ok(())
}

/// fn main() {
///     // The keys in this map are vectors, not strings.
///     let mut map = BTreeMap::new();
///     map.insert(vec![32, 64], "x86");
///
///     println!("{}", serde_json::to_value(map).unwrap_err());
/// }

fn main() {
    let mut map = BTreeMap::new();
    map.insert("32", "x86");

    println!("[{}]", serde_json::to_value(map).unwrap());
    // you can declare a variable without assigning a value
    let plant: Asparagus;
    // and then assign a value later
    plant = Asparagus {};
    let tests = test();
    if let Err(err) = tests {
        println!("{:?}", err);
    } 
    println!("I'm growing a {:?}! There is some {:?}.", plant, Music {});
    eat();
}
