#![no_std]

extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

mod board;
mod moves;
mod piece;
mod position;


pub const RED: Color = Color::Red;
pub const BLACK: Color = Color::Black;



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color{
    Red,
    Black,
}


impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Red => "Red",
                Self::Black => "Black",
            }
        )
    }
}

/// A color can be inverted using the `!` operator.
/// `!Color::White` becomes `Color::Black` and vice versa.
impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::Red => Self::Black,
            Self::Black => Self::Red,
        }
    }
}

