use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Cycles,
    Instructions,
    Branches,
    BranchMisses,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Event::Cycles => write!(f, "Cycles"),
            Event::Instructions => write!(f, "Instructions"),
            Event::Branches => write!(f, "Branches"),
            Event::BranchMisses => write!(f, "BranchMisses"),
        }
    }
}
