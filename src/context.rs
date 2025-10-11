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
    pub specified_date: Option<chrono::NaiveDate>,
    pub usersetdate: chrono::NaiveDate,
    pub opts: Cli,
    pub config: Config,
    pub theme: Theme,
    pub styletype: StyleType,
    pub begin: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
    pub columns: usize,
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

        let usersetdate: chrono::NaiveDate = opts.validate_date()?;
        let specified_date = if opts.date.len() == 3 { Some(usersetdate) } else { None };

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
            specified_date,
            usersetdate,
            opts,
            config,
            theme,
            styletype,
            begin,
            end,
            columns,
        })
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            specified_date: None,
            usersetdate: NaiveDate::default(),
            opts: Cli::parse(),
            config: Config::default(),
            theme: Theme::default(),
            styletype: StyleType::Light,
            begin: NaiveDate::default(),
            end: NaiveDate::default(),
            columns: 1,
        }
    }
}
