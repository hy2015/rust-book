
//! # My Crate
//! `my_crate` is a collection of utilities to make performing certain 
//! calculations more convenient.

/// Adds one to the number given
/// # Examples
/// ```
/// let arg = 5;
/// let answer = chapter_14_02::add_one(arg); 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;
}

/// # Art
///
/// A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


pub mod kinds {
    /// The primary colors according to the RYB color model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }

}

pub mod utils {
    use crate::kinds::{PrimaryColor, SecondaryColor};

    /// Mixes two primary colors to create a secondary color.
    /// # Examples
    /// ```
    /// use chapter_14_02::kinds::PrimaryColor;
    /// use chapter_14_02::utils::mix;
    ///
    /// let color1 = PrimaryColor::Red;
    /// let color2 = PrimaryColor::Yellow;
    /// let result = mix(color1, color2);
    /// assert_eq!(result, Some(chapter_14_02::kinds::SecondaryColor::Orange));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        use PrimaryColor::*;
        use SecondaryColor::*;

        match (c1, c2) {
            (Red, Yellow) | (Yellow, Red) => Some(Orange),
            (Red, Blue) | (Blue, Red) => Some(Purple),
            (Yellow, Blue) | (Blue, Yellow) => Some(Green),
            _ => None,
        }
    }
}