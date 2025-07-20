use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(
    Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TableType {
    #[default]
    A,
    B,
    C,
}

impl Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableType::A => write!(f, "A"),
            TableType::B => write!(f, "B"),
            TableType::C => write!(f, "C"),
        }
    }
}

impl FromStr for TableType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(TableType::A),
            "B" => Ok(TableType::B),
            "C" => Ok(TableType::C),
            _ => Err(format!("Invalid table type: {}", s)),
        }
    }
}
