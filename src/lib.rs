mod front_of_house;
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // utilize use keyword
    pub use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
}
