#![feature(link_args)]

// #[link_args = "-s EXPORTED_FUNCTIONS=['_coolrand','_makeIter','_next']"]
extern {}

mod ops;
use ops::doit;
mod indexes;

fn main() {
    doit();
    // test_lubm()
}
