// SPDX-FileCopyrightText: 2025 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT
use std::sync::Arc;
use crate::events::EventInstance;
use crate::config::{Theme, Style, StyleType};
use crate::utils::tostyle;
use crate::utils::DateExtensions;
use chrono::NaiveDate;
use minijinja::value::{Object, Value, from_args};


#[derive(Debug)]
pub(crate) struct DateStyler {
    event_instances: Vec<EventInstance>,
    main_date: chrono::NaiveDate,
    theme: Theme,
    styletype: StyleType,
}

impl DateStyler {
    pub(crate) fn new(event_instances: Vec<EventInstance>, main_date: chrono::NaiveDate, theme: Theme, styletype: StyleType) -> Self {
        Self { event_instances, main_date, theme, styletype }
    }
}

impl Object for DateStyler {
    fn call(self: &Arc<DateStyler>, _state: &minijinja::State, args: &[Value]) -> Result<Value, minijinja::Error> {
        let (date, month, ): (&str, Option<&str>, ) = from_args(args)?;
        if let Ok(pdate) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            let month = NaiveDate::parse_from_str(month.unwrap_or(date), "%Y-%m-%d").unwrap_or(pdate);
            let mut matching_styles: Vec<Style> = self.theme
                .date
                .iter()
                .filter(|datestyle| {
                    pdate.satisfy_all(
                        month.first_day_of_month(),
                        self.main_date,
                        &self.event_instances,
                        &datestyle.properties,
                    )
                })
                .cloned()
                .map(|datestyle| datestyle.style)
                .collect();

            for instance in &self.event_instances {
                if instance.date == pdate {
                    matching_styles.push(instance.style.clone());
                }
            }

            matching_styles.retain(|style| {
                style.styletype == self.styletype || style.styletype == StyleType::None
            });

            matching_styles.sort_by(|a, b| a.weight.cmp(&b.weight));
            let mut stylenames = vec![];
            for mut style in matching_styles {
                stylenames.append(&mut style.stylenames);
            }

            return Ok(tostyle(stylenames).render().to_string().into());
        }
        Ok(date.into())
    }
}
