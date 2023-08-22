// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct DateStyle {
    pub properties: Vec<DateProperty>,
    #[serde(flatten)]
    pub style: Style,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum StyleType {
    Dark,
    Light,
    None,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Style {
    pub stylenames: Vec<StyleName>,
    pub weight: u32,
    pub styletype: StyleType,
}
impl Default for Style {
    fn default() -> Self {
        Style {
            stylenames: vec![],
            weight: 0,
            styletype: StyleType::None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum StyleName {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,

    FGBlack,
    FGRed,
    FGGreen,
    FGYellow,
    FGBlue,
    FGPurple,
    FGCyan,
    FGWhite,
    #[serde(rename = "FGRGB")]
    FGrgb {
        r: u8,
        g: u8,
        b: u8,
    },
    FGFixed(u8),

    BGBlack,
    BGRed,
    BGGreen,
    BGYellow,
    BGBlue,
    BGPurple,
    BGCyan,
    BGWhite,
    #[serde(rename = "BGRGB")]
    BGrgb {
        r: u8,
        g: u8,
        b: u8,
    },
    BGFixed(u8),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DateProperty {
    FirstDayOfMonth,
    BeforeFirstDayOfMonth,
    BeforeCurrentDate,
    CurrentDate,
    AfterCurrentDate,
    AfterLastDayOfMonth,
    LastDayOfMonth,
    IsEvent,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Odd,
    Even,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_datestyle_properties() {
        let a = DateStyle::default();
        assert![a.properties.is_empty()];
    }
    #[test]
    fn test_default_datestyle_style() {
        let a = DateStyle::default();
        assert![a.style.stylenames.is_empty()];
        assert_eq![a.style.weight, 0];
    }
    #[test]
    fn test_default_style_stylenames() {
        let a = Style::default();
        assert![a.stylenames.is_empty()];
    }
    #[test]
    fn test_default_style_weight() {
        let a = Style::default();
        assert_eq![a.weight, 0];
    }
}
