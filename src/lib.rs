pub mod garden;
pub mod fountain;

mod meat {
    mod steak {
        pub struct Steak {}
    }
}

// Unlike main.rs, lib.rs does not need to use the crate name to access the vegetable module.
// It's because that the lib.rs is in the same module as the garden.rs and fountain.rs.
use garden::vegetable::Asparagus;
use fountain::water::Music;

pub fn eat() {
    let steak = meat::steak::Steak {};
    let meat = crate::meat::Steak {};
}