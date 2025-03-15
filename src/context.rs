// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::cli::{Action, Cli};
use crate::config::StyleType;
use crate::config::{Config, Theme};
use crate::events::EventInstance;
use crate::events::{Events, ReadFromIcsFile};
use crate::utils::helpers::{generate_dates_from_to, MyDate};
use crate::utils::DateExtensions;
use chrono::prelude::*;
use clap::Parser;
use minijinja::value::{Object, Value};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// A struct storing the combined settings of config file, theme, options, ...
#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    pub usersetdate: chrono::NaiveDate,
    pub opts: Cli,
    pub event_instances: Vec<EventInstance>,
    pub columns: usize,
    pub dates_per_month: Vec<Vec<MyDate>>,
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

        let mut event_instances = vec![];
        for icalstyle in &config.ical {
            for event in Events::read_from_ics_file(&icalstyle.file) {
                event_instances.append(&mut event.instances(&begin, &end, &icalstyle.style));
            }
        }
        event_instances.sort_by(|a, b| a.date.cmp(&b.date));

        let dates_per_month = generate_dates_from_to(begin, end, usersetdate, &opts, &theme, &event_instances);

        Ok(Context {
            usersetdate,
            opts,
            event_instances,
            columns,
            dates_per_month,
        })
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            usersetdate: NaiveDate::default(),
            opts: Cli::parse(),
            event_instances: vec![],
            columns: 1,
            dates_per_month: vec![],
        }
    }
}

impl Object for Context {
    fn get_value(self: &Arc<Self>, field: &Value) -> Option<Value> {
        match field.as_str()? {
            "cli" => Some(Value::from_serialize(&self.opts)),
            "columns" => Some(Value::from(self.columns)),
            "event_instances" => Some(Value::from_serialize(&self.event_instances)),
            "dates_per_month" => Some(Value::from_serialize(&self.dates_per_month)),
            _ => None,
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
