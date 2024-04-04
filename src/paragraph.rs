// Define the paragraph related structs and enums

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};
use crate::tokens::ControlWord;

#[cfg(feature="serde_support")]
#[derive(Debug, Default, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Paragraph {
    pub alignment: Alignment,
    pub spacing: Spacing,
    pub indent: Indentation,
    pub tab_width: i32,
}
#[cfg(not(feature="serde_support"))]
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Paragraph {
    pub alignment: Alignment,
    pub spacing: Spacing,
    pub indent: Indentation,
    pub tab_width: i32,
}

/// Alignement of a paragraph (left, right, center, justify)
#[cfg(feature="serde_support")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash, Deserialize, Serialize)]
pub enum Alignment {
    #[default]
    LeftAligned,    // \ql
    RightAligned,   // \qr
    Center,         // \qc
    Justify,        // \qj
}
#[cfg(not(feature="serde_support"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
pub enum Alignment {
    #[default]
    LeftAligned,    // \ql
    RightAligned,   // \qr
    Center,         // \qc
    Justify,        // \qj
}

impl From<&ControlWord<'_>> for Alignment {
    fn from(cw: &ControlWord) -> Self {
        return match cw {
            ControlWord::LeftAligned  => Alignment::LeftAligned,
            ControlWord::RightAligned => Alignment::RightAligned,
            ControlWord::Center       => Alignment::Center,
            ControlWord::Justify      => Alignment::Justify,
            _  /* default */          => Alignment::LeftAligned,
        };
    }
}

/// The vertical margin before / after a block of text
#[cfg(feature="serde_support")]
#[derive(Debug, Default, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Spacing {
    pub before: i32,
    pub after: i32,
    pub between_line: SpaceBetweenLine,
    pub line_multiplier: i32,
}
#[cfg(not(feature="serde_support"))]
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Spacing {
    pub before: i32,
    pub after: i32,
    pub between_line: SpaceBetweenLine,
    pub line_multiplier: i32,
}

#[cfg(feature="serde_support")]
#[derive(Default, Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub enum SpaceBetweenLine {
    Value(i32),
    #[default]
    Auto,
    Invalid,
}
#[cfg(not(feature="serde_support"))]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub enum SpaceBetweenLine {
    Value(i32),
    #[default]
    Auto,
    Invalid,
}

// Space between lines.
// If this control word is missing or if \sl1000 is used, the line spacing is automatically determined by the tallest character in the line;
// if N is a positive value, this size is used only if it is taller than the tallest character (otherwise, the tallest character is used);
// if N is a negative value, the absolute value of N is used, even if it is shorter than the tallest character.
impl From<i32> for SpaceBetweenLine {
    fn from(value: i32) -> Self {
        return match value {
            1000 => SpaceBetweenLine::Auto,
            val if val < 0 => SpaceBetweenLine::Value(val.abs()),
            val => SpaceBetweenLine::Value(val),
        };
    }
}

// This struct can not be an enum because left-indent and right-ident can both be defined at the same time
#[cfg(feature="serde_support")]
#[derive(Default, Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
pub struct Indentation {
    pub left: i32,
    pub right: i32,
    pub first_line: i32,
}
#[cfg(not(feature="serde_support"))]
#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct Indentation {
    pub left: i32,
    pub right: i32,
    pub first_line: i32,
}
