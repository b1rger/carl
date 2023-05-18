// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod ansi_term_style;
mod themecomponents;

pub use ansi_term_style::{AnsiTermStyle, StyleConversion};
pub use themecomponents::{DateStyle, Style, StyleType};

use crate::date::DateProperty::*;
use AnsiTermStyle::*;

extern crate xdg;
use clap::crate_name;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Config {
    pub theme: Option<String>,
    pub ical: Vec<IcalStyle>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: None,
            ical: vec![],
        }
    }
}

impl Config {
    #[cfg(not(tarpaulin_include))]
    pub fn read() -> Config {
        match xdg::BaseDirectories::with_prefix(crate_name!()) {
            Ok(xdg_dirs) => {
                if let Some(config_path) = xdg_dirs.find_config_file("config.toml") {
                    let config_content = fs::read_to_string(&config_path).unwrap_or_default();
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Theme {
    pub date: Vec<DateStyle>,
}

impl Theme {
    #[cfg(not(tarpaulin_include))]
    pub fn read(theme: &Option<String>) -> Theme {
        if let Some(themename) = theme {
            match xdg::BaseDirectories::with_prefix(crate_name!()) {
                Ok(xdg_dirs) => {
                    if let Some(theme_path) =
                        xdg_dirs.find_config_file(format!("{}.theme", themename))
                    {
                        let theme_content = fs::read_to_string(&theme_path).unwrap_or_default();
                        match toml::from_str(&theme_content) {
                            Ok(theme) => return theme,
                            Err(e) => eprintln!("Could not parse theme file: {}", e),
                        }
                    } else {
                        eprintln!(
                            "Could not load theme file {}.toml, using builtin theme.",
                            themename
                        );
                    }
                }
                Err(e) => eprintln!("Cannot determine XDG base directories: {}", e),
            }
        }
        Theme::default()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            date: default_datestyle_vector(),
        }
    }
}

fn default_datestyle_vector() -> Vec<DateStyle> {
    vec![
        DateStyle {
            properties: vec![CurrentDate],
            style: Style {
                stylenames: vec![Bold, Underline],
                ..Default::default()
            },
        },
        DateStyle {
            properties: vec![BeforeCurrentDate],
            style: Style {
                stylenames: vec![Dimmed],
                ..Default::default()
            },
        },
        DateStyle {
            properties: vec![BeforeFirstDayOfMonth],
            style: Style {
                stylenames: vec![Hidden],
                weight: 1,
                ..Default::default()
            },
        },
        DateStyle {
            properties: vec![AfterLastDayOfMonth],
            style: Style {
                stylenames: vec![Hidden],
                weight: 1,
                ..Default::default()
            },
        },
    ]
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
    fn test_default_theme() {
        let a = Theme::default();
        assert![!a.date.is_empty()];
    }

    #[test]
    fn test_theme_read_none() {
        let a = Theme::read(&None);
        assert![!a.date.is_empty()];
    }

    #[test]
    fn test_theme_read_filename() {
        env::set_var("XDG_CONFIG_DIRS", "/nonexistent");
        env::set_var("XDG_CONFIG_HOME", "/nonexistent");
        let a = Theme::read(&Some(String::from("nonexistent")));
        assert![!a.date.is_empty()];
    }

    #[test]
    fn test_default_icalstyle() {
        let a = IcalStyle::default();
        assert_eq![a.file, String::new()];
    }
}
