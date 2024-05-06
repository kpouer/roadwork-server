use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq)]
pub(crate) enum Status {
    New,
    Later,
    Ignored,
    Finished,
    Treated,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::New => "New",
            Status::Later => "Later",
            Status::Ignored => "Ignored",
            Status::Finished => "Finished",
            Status::Treated => "Treated"
        };
        write!(f, "{}", str)
    }
}