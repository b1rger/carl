// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod theme;

pub use theme::StyleName::*;
pub use theme::{DateProperty, Style, StyleName, StyleType, Theme};

extern crate xdg;
use clap::crate_name;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub theme: Option<String>,
    pub ical: Vec<IcalStyle>,
    pub template: Option<String>,
}

impl Config {
    #[cfg(not(tarpaulin_include))]
    pub fn read() -> Config {
        match xdg::BaseDirectories::with_prefix(crate_name!()) {
            Ok(xdg_dirs) => {
                if let Some(config_path) = xdg_dirs.find_config_file("config.toml") {
                    let config_content = fs::read_to_string(config_path).unwrap_or_default();
                    match toml::from_str(&config_content) {
                        Ok(config) => return config,
                        Err(e) => eprintln!("Could not parse config file: {}", e),
                    }
                } else {
                    //for now disabled, should only be shown with some kind of --debug flag
                    //eprintln!("Could not load configuration file, using default settings.");
                }
            }
            Err(e) => eprintln!("Cannot determine XDG base directories: {}", e),
        }
        Config::default()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct IcalStyle {
    pub file: String,
    #[serde(flatten)]
    pub style: Style,
}
impl Default for IcalStyle {
    fn default() -> Self {
        IcalStyle {
            file: String::new(),
            style: default_icalstyle_style(),
        }
    }
}
fn default_icalstyle_style() -> Style {
    Style {
        stylenames: vec![Underline, FGCyan],
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let a = Config::default();
        assert![a.theme.is_none()];
    }

    #[test]
    fn test_config_read() {
        env::set_var("XDG_CONFIG_DIRS", "/nonexistent");
        env::set_var("XDG_CONFIG_HOME", "/nonexistent");
        let a = Config::read();
        assert![a.theme.is_none()];
    }

    #[test]
    fn test_default_icalstyle() {
        let a = IcalStyle::default();
        assert_eq![a.file, String::new()];
    }
}
