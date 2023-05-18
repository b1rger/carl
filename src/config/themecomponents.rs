// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::config::AnsiTermStyle;
use crate::date::DateProperty;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct DateStyle {
    pub properties: Vec<DateProperty>,
    #[serde(flatten)]
    pub style: Style,
}
impl Default for DateStyle {
    fn default() -> Self {
        DateStyle {
            properties: vec![],
            style: Style::default(),
        }
    }
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
    pub stylenames: Vec<AnsiTermStyle>,
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
