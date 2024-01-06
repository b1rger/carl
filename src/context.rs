// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::cli::Cli;
use crate::config::{Config, Theme};
use crate::config::{Style, StyleType};
use crate::events::Event;
use anyhow::Result;
use chrono::prelude::*;
use clap::Parser;

// A struct storing the combined settings of config file, theme, options, ...
pub struct Context {
    pub usersetdate: chrono::NaiveDate,
    pub opts: Cli,
    pub config: Config,
    pub eventstuple: Vec<(Event, Style)>,
    pub theme: Theme,
    pub styletype: StyleType,
}

impl Context {
    pub fn new() -> Result<Context> {
        let opts: Cli = Cli::parse();
        let config: Config = Config::read();
        let theme: Theme = if opts.theme.is_some() {
            Theme::read(&opts.theme)
        } else {
            Theme::read(&config.theme)
        };

        let styletype: StyleType = if opts.themestyletype == Some(String::from("dark")) {
            StyleType::Dark
        } else {
            StyleType::Light
        };

        let usersetdate: chrono::NaiveDate = match opts.validate_date() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        Ok(Context {
            usersetdate,
            opts,
            config,
            eventstuple: vec![],
            theme,
            styletype,
        })
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            usersetdate: NaiveDate::default(),
            opts: Cli::default(),
            config: Config::default(),
            eventstuple: vec![],
            theme: Theme::default(),
            styletype: StyleType::Light,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_default() {
        let ctx = Context::default();
        assert!(ctx.eventstuple.is_empty());
    }
    #[test]
    fn test_context_new() {
        let ctx = Context::new().unwrap();
        assert!(ctx.eventstuple.is_empty());
    }
}
