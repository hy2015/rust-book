mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    add_to_waitlist();
}