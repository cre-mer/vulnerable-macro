#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use proc_macro_example::return_42;
fn my_function() -> u32 {
    {}
    42
}
fn main() {
    {
        ::std::io::_print(format_args!("My function\'s result: {0}\n", my_function()));
    };
    {
        ::std::io::_print(format_args!("Macro has been executed!\n"));
    };
}
