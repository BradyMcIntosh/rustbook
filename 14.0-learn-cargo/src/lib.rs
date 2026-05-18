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
