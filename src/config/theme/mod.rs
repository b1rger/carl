// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT
mod components;

pub use crate::config::theme::components::DateProperty::*;
pub use crate::config::theme::components::StyleName::*;
pub use crate::config::theme::components::{DateProperty, DateStyle, Style, StyleName, StyleType};
use clap::crate_name;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Theme {
    pub date: Vec<DateStyle>,
}

impl Theme {
    #[cfg(not(tarpaulin_include))]
    pub fn read(theme: &Option<String>) -> Theme {
        if let Some(themename) = theme {
            if let Some(project_dirs) = directories::ProjectDirs::from("org", "bisco", crate_name!()) {
                let theme_file = project_dirs.config_dir().join(format!("{}.theme", themename));
                if theme_file.exists() {
                    let theme_content = fs::read_to_string(theme_file).unwrap_or_default();
                    match toml::from_str(&theme_content) {
                        Ok(theme) => return theme,
                        Err(e) => eprintln!("Could not parse theme file: {}", e),
                    }
                } else {
                    eprintln!("Could not load theme file {}.theme, using builtin theme.", themename);
                }
            } else {
                eprintln!("Cannot determine config directory, using builtin theme.");
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
            properties: vec![Today],
            style: Style {
                stylenames: vec![Bold, Underline],
                ..Default::default()
            },
        },
        DateStyle {
            properties: vec![BeforeToday],
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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
}
