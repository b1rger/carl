// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
use crate::config::{Config, Style, StyleType, Theme};
use crate::events::Event;
use crate::lib::types::ChronoDate;
use crate::opts::Opts;
use anyhow::Result;
use chrono::prelude::*;
use clap::Parser;

// A struct storing the combined settings of config file, theme, options, ...
pub struct Context {
    pub usersetdate: ChronoDate,
    pub opts: Opts,
    pub config: Config,
    pub eventstuple: Vec<(Event, Style)>,
    pub theme: Theme,
    pub styletype: StyleType,
}

impl Context {
    pub fn new() -> Result<Context> {
        let opts: Opts = Opts::parse();
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

        let usersetdate: ChronoDate;
        match opts.validate_date() {
            Ok(x) => usersetdate = x,
            Err(x) => return Err(x),
        }

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
            usersetdate: Local.ymd(1970, 1, 1),
            opts: Opts::default(),
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
