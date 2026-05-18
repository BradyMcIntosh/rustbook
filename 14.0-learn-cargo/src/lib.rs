//! # Learn Cargo
//!
//! I am learning cargo, so let's `learn-cargo` together!

/// Adds two numbers together.
///
/// ### Examples
///
/// ```
/// let x = 1;
/// let y = 1;
/// let answer = learn_cargo::sum(x, y);
///
/// assert_eq!(answer, 2);
/// ```
pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub mod kinds {
    /// The classic numbers everyone should know.
    pub enum EasyNumber {
        One,
        Two,
        Three,
    }

    /// Rare numbers known only to the erudite or insane.
    pub enum HardNumber {
        Nine,
        Ten,
        Eleven,
    }
}

pub mod utils {
    use std::fmt::Error;

    use crate::kinds::*;

    /// Attempts to add a hard number to an easy number. Dangerous business.
    ///
    /// ### Panics
    ///
    /// This function will panic if it can't count high enough.
    pub fn add(num1: EasyNumber, num2: HardNumber) -> Result<Error, HardNumber> {
        unimplemented!();
    }
}
