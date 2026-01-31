use std::num::ParseIntError;
use std::str::FromStr;

// Supported missions
// Support Clipper, etc, in the future?
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mission {
    PSYCHE, // Psyche
}

// Supported instruments
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Instrument {
    PsycheImagerA,
    PsycheImagerB,
    #[default]
    None,
}

impl FromStr for Instrument {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Instrument, ParseIntError> {
        Ok(match s.to_uppercase().as_str() {
            "A" => Instrument::PsycheImagerA,
            "B" => Instrument::PsycheImagerB,
            _ => Instrument::None,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CalFileType {
    FlatField,
    InpaintMask,
    Mask,
    Lut,
}
