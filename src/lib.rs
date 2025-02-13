pub mod garden;
pub mod fountain;
pub mod practice_syntax;
pub mod restaurant;
pub mod learn_tokio;

// Unlike main.rs, lib.rs does not need to use the crate name to access the vegetable module.
// It's because that the lib.rs is in the same module as the garden.rs and fountain.rs.
use garden::vegetable::Asparagus;
use fountain::water::Music;


// 2 ways to create a mod in lib.rs
// 1. Create a new file in the src directory with the name of the module and add the module's code to it.
// 2. Create a new directory in the src directory with the name of the module and create a mod.rs file inside it.
// The mod.rs file is an implicit module file that contains the module's code.

// first way
// |- lib.rs
// |- house.rs
// |- house
// |  |- kitchen.rs
// |  |- bedroom.rs
// |  |- bathroom.rs

// second way
// |- lib.rs
// |- house
// |  |- mod.rs
// |  |- kitchen.rs
// |  |- bedroom.rs
// |  |- bathroom.rs

// The first way is more common because it's easier to find the module's code in a separate file.
// The second way is useful when you have multiple modules in the same directory.