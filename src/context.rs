// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::cli::{Action, Cli};
use crate::config::StyleType;
use crate::config::{Config, Theme};
use crate::utils::DateExtensions;
use chrono::prelude::*;
use clap::Parser;
use serde::Deserialize;

// A struct storing the combined settings of config file, theme, options, ...
#[derive(Deserialize, Debug)]
pub struct Context {
    pub usersetdate: chrono::NaiveDate,
    pub opts: Cli,
    pub config: Config,
    pub begin: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
    pub columns: usize,
    pub theme: Theme,
}

impl Context {
    pub fn new() -> Result<Context, String> {
        let mut opts: Cli = Cli::parse();
        let config: Config = Config::read();
        let mut theme: Theme = if opts.theme.is_some() {
            Theme::read(&opts.theme)
        } else {
            Theme::read(&config.theme)
        };

        // Get styletype and filter theme styles accordingly
        let styletype: StyleType = if opts.themestyletype == Some(String::from("dark")) {
            StyleType::Dark
        } else {
            StyleType::Light
        };
        theme.date.retain(|date| {
            date.style.styletype == styletype || date.style.styletype == StyleType::None
        });

        let usersetdate: chrono::NaiveDate = match opts.validate_date() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if opts.action == Action::default() {
            opts.action.calendar = true;
        }

        let mut columns = 1;
        if opts.three || opts.year || opts.months.is_some() {
            columns = 3;
        }

        let mut begin = usersetdate.first_day_of_month();
        let mut end = usersetdate.last_day_of_month();

        if opts.three {
            begin = usersetdate
                .first_day_of_month()
                .pred_opt()
                .unwrap()
                .first_day_of_month();
            end = usersetdate.first_day_of_next_month().last_day_of_month();
        }

        if let Some(num) = opts.months {
            begin = usersetdate.first_day_of_month();
            let mut tmpdate = begin;
            for _ in 1..=(num - 1) {
                tmpdate = tmpdate.first_day_of_next_month();
            }
            end = tmpdate.last_day_of_month();
        }

        if opts.year {
            begin = usersetdate.first_day_of_year();
            end = usersetdate.last_day_of_year();
        }


        Ok(Context {
            usersetdate,
            opts,
            config,
            begin,
            end,
            columns,
            theme,
        })
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            usersetdate: NaiveDate::default(),
            opts: Cli::parse(),
            config: Config::default(),
            begin: NaiveDate::default(),
            end: NaiveDate::default(),
            columns: 1,
            theme: Theme::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_default() {
        let ctx = Context::default();
        assert!(ctx.event_instances.is_empty());
    }
    #[test]
    fn test_context_new() {
        let ctx = Context::new().unwrap();
        assert!(ctx.event_instances.is_empty());
    }
}
