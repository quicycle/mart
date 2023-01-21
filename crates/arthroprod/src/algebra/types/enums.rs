use std::{convert::TryFrom, fmt, ops};

/// Simple vector directed sign (positive or negative)
#[derive(Hash, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub enum Sign {
    Pos,
    Neg,
}

impl Sign {
    /// Combine together two Signs using conventional rules of arithmetic
    pub fn combine(&self, other: Sign) -> Sign {
        use Sign::*;

        match (self, other) {
            (Pos, Pos) | (Neg, Neg) => Pos,
            _ => Neg,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sign::Pos => write!(f, "+"),
            Sign::Neg => write!(f, "-"),
        }
    }
}

impl ops::Neg for Sign {
    type Output = Sign;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Neg => Sign::Pos,
            Sign::Pos => Sign::Neg,
        }
    }
}

impl ops::Mul<Sign> for Sign {
    type Output = Sign;

    fn mul(self, other: Sign) -> Self::Output {
        use Sign::*;

        match (self, other) {
            (Pos, Pos) | (Neg, Neg) => Pos,
            _ => Neg,
        }
    }
}

/// A single Space-Time axis. One of the four basis elements for the coordinate system we work in
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Index {
    Zero,
    One,
    Two,
    Three,
}

impl Index {
    /// Allow for construction of Index values using 0-3 notation
    pub fn try_from_u8(x: u8) -> Result<Index, String> {
        match x {
            0 => Ok(Index::Zero),
            1 => Ok(Index::One),
            2 => Ok(Index::Two),
            3 => Ok(Index::Three),
            _ => Err(format!("{:?} is not a valid index", x)),
        }
    }

    pub fn try_from_char(c: char) -> Result<Index, String> {
        match c {
            '0' => Ok(Index::Zero),
            '1' => Ok(Index::One),
            '2' => Ok(Index::Two),
            '3' => Ok(Index::Three),
            _ => Err(format!("{:?} is not a valid index", c)),
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Index::Zero => write!(f, "0"),
            Index::One => write!(f, "1"),
            Index::Two => write!(f, "2"),
            Index::Three => write!(f, "3"),
        }
    }
}

/// The Grade of a given space-time form as determined by its number of indices.
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Grade {
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl TryFrom<&str> for Grade {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            _ => Err(format!("{} is not a valid grade", s)),
        }
    }
}
