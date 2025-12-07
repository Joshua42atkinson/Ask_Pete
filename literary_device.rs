use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiteraryDevice {
    Metaphor,
    Simile,
    Allegory,
    Irony,
    Foreshadowing,
    Flashback,
    Symbolism,
    Personification,
    Hyperbole,
    Alliteration,
    // Add more as needed
}

impl fmt::Display for LiteraryDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
