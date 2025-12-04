use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LiteraryDevice {
    HerosJourney,
    Mystery,
    DeepDive,
    Collaborative,
    Conflict,
    StressRelief,
    Intuition,
    SelfCorrection,
}

impl fmt::Display for LiteraryDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteraryDevice::HerosJourney => write!(f, "The Standard Run (Hero's Journey)"),
            LiteraryDevice::Mystery => write!(f, "The Derailment (Mystery)"),
            LiteraryDevice::DeepDive => write!(f, "The Heavy Haul (Deep Dive)"),
            LiteraryDevice::Collaborative => write!(f, "The Relay (Collaborative)"),
            LiteraryDevice::Conflict => write!(f, "The Ghost Train (Conflict)"),
            LiteraryDevice::StressRelief => write!(f, "The Blowdown Protocol (Stress Relief)"),
            LiteraryDevice::Intuition => write!(f, "Dark Territory Run (Intuition)"),
            LiteraryDevice::SelfCorrection => {
                write!(f, "The Governor Recalibration (Self-Correction)")
            }
        }
    }
}

impl LiteraryDevice {
    pub fn all() -> Vec<LiteraryDevice> {
        vec![
            LiteraryDevice::HerosJourney,
            LiteraryDevice::Mystery,
            LiteraryDevice::DeepDive,
            LiteraryDevice::Collaborative,
            LiteraryDevice::Conflict,
            LiteraryDevice::StressRelief,
            LiteraryDevice::Intuition,
            LiteraryDevice::SelfCorrection,
        ]
    }
}
