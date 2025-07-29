use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
