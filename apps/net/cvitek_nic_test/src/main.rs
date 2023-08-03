#![no_std]
#![no_main]

#[marco_use]
extern crate libax;
extern crate alloc;

#[no_mangle]
fn main() {
    libax::println!("Hello world!");
}