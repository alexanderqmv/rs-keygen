//! # KeyGen v0.1.2 (beta)
//! Simple, customizable and convenient Key Generator. (in development).
//!
//! By using `KeyGen` you can generate the keys of any size,enable/disable symbols, numbers, spaces, uppercase/lowercase letters, etc.
//! # Features
//!
//! * Bug fixes
//! * Code optimization
//! * New features added (`space` state, `gen` method)
//! Thank you for you help & support!
//!
//! # Example
//! ```
//! fn main() {
//!     let mut gen_range = KeyGen::new()
//!         .length(11)
//!         .numbers(true)
//!         .symbols(true)
//!         .uppercase(true)
//!         .lowercase(true)
//!         .space(true);
//!
//!    let res = gen_range
//!         .gen_one()
//!         .unwrap();
//!    println!("{res}");
//!    let res2 = gen_range
//!         .gen(5)
//!         .unwrap();
//!    println!("{res2:?}");
//!}
//! ```


mod generator;
use generator::Key::KeyGen;
mod tests;
#[cfg(feature = "crypto")]
mod hasher;
