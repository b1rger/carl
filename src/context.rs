// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::cli::{Action, Cli};
use crate::config::StyleType;
use crate::config::{Config, Theme};
use crate::events::EventInstances;
use chrono::prelude::*;
use clap::Parser;

// A struct storing the combined settings of config file, theme, options, ...
pub struct Context {
    pub usersetdate: chrono::NaiveDate,
    pub opts: Cli,
    pub config: Config,
    pub theme: Theme,
    pub styletype: StyleType,
    pub eventinstances: EventInstances,
}

impl Context {
    pub fn new() -> Result<Context, String> {
        let mut opts: Cli = Cli::parse();
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

        if opts.action == Action::default() {
            opts.action.calendar = true;
        }

        Ok(Context {
            usersetdate,
            opts,
            config,
            theme,
            styletype,
            eventinstances: vec![],
        })
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            usersetdate: NaiveDate::default(),
            opts: Cli::parse(),
            config: Config::default(),
            theme: Theme::default(),
            styletype: StyleType::Light,
            eventinstances: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_default() {
        let ctx = Context::default();
        assert!(ctx.eventinstances.is_empty());
    }
    #[test]
    fn test_context_new() {
        let ctx = Context::new().unwrap();
        assert!(ctx.eventinstances.is_empty());
    }
}
