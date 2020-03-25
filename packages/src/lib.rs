#[allow(dead_code)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {};

    // When making a struct public, all fields are private
    // by default and must be identified as public.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}

mod back_of_house {
    // Making an enum public all variants are public.
    // We only need the `pub` before the `enum` keyword.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Bringing Paths into Scope with `use` keyword

// absolute
use crate::front_of_house::hosting;
// relative
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// use crate::front_of_house::hosting::add_to_waitlist;
// ^-- this would work, but
// the idiomatic way of using `use` is to bring the
// functions parent module into scope to make it
// clear that the function is not locally defined.

// However, when bringing in structs, enums, and other
// items with `use`. it's odomatic to specify the full
// path:

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// The exception to this idiot is if we're bringing in
// two items with the same name into scope:

use std::fmt;
use std::io;


fn func1() -> fmt::Result {
    // --snip--
}

fn func2() -> io::Result {
    // --snip--
}

// You can also solve the last problem using the `as`
// keyword:

use std::fmt::Result;
use std::io::Result as IoResult;


fn func1() -> Result {
    // --snip--
}

fn func2() -> IoResult<()> {
    // --snip--
}

// Re-exporting Names with `pub use`
// When bringing a name into scope with `use`, the
// name is private. To open that code up for others
// to bring into their scope, use `pub use`. This is
// called re-exporting.

pub use crate::front_of_house::hosting;

// We can also use nested paths to bring items into
// scope. Instead of:
use std::io;
use std::cmp::Ordering;

// you can do:
use std::{cmp::Ordering, io};

// instead of:
use std::io;
use std::io::Write;

// do:
use std::io::{self, Write};

// Separating modules into different files

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}


// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Using a semicolon after mod front_of_house rather
// than using a block tells Rust to load the contents
// of the module from another file with the same name
// as the module.

// To continue with our example and extract the hosting
// module to its own file as well, we change
// src/front_of_house.rs to contain only the
// declaration of the hosting module:

// src/front_of_house.rs
pub mod hosting;

// Then we create a src/front_of_house directory
// and a file src/front_of_house/hosting.rs to
// contain the definitions made in the hosting module:

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
