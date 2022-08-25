use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// CHIP-8 displays are black and white, so each pixel can be in only one of two states.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Pixel {
    #[default]
    Black = 0,
    White = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct PixelConversionError;

impl Display for PixelConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tried to convert a number that wasn't a 0 or 1 into a Pixel."
        )
    }
}

use Pixel::*;

impl From<Pixel> for u8 {
    fn from(px: Pixel) -> Self {
        match px {
            Black => 0,
            White => 1,
        }
    }
}

impl From<Pixel> for bool {
    fn from(px: Pixel) -> Self {
        match px {
            Black => false,
            White => true,
        }
    }
}

impl TryFrom<u8> for Pixel {
    type Error = PixelConversionError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Black),
            1 => Ok(White),
            _ => Err(PixelConversionError),
        }
    }
}

impl BitAnd for Pixel {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Black, Black) => Black,
            (Black, White) => Black,
            (White, Black) => Black,
            (White, White) => White,
        }
    }
}

impl BitAndAssign for Pixel {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Pixel {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Black, Black) => Black,
            (Black, White) => White,
            (White, Black) => White,
            (White, White) => White,
        }
    }
}

impl BitOrAssign for Pixel {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Pixel {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Black, Black) => Black,
            (Black, White) => White,
            (White, Black) => White,
            (White, White) => Black,
        }
    }
}

impl BitXorAssign for Pixel {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for Pixel {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Black => White,
            White => Black,
        }
    }
}
