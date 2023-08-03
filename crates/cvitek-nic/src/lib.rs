#![no_std]
#![allow(dead_code)]

extern crate alloc;
#[macro_use]
extern crate log;

mod cvitek_defs;
mod cvitek_main;

pub use cvitek_main::CvitekNicDevice;