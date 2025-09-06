use std::sync::Arc;
use crate::events::EventInstance;
use minijinja::value::{Object, Value, from_args};
use crate::config::Theme;
use crate::utils::helpers::{satisfy_all, tostyle};
use crate::utils::DateExtensions;
use chrono::NaiveDate;
use crate::config::Style as MyStyle;


#[derive(Debug)]
pub(crate) struct DateStyler {
    event_instances: Vec<EventInstance>,
    main_date: chrono::NaiveDate,
    theme: Theme,
}

impl DateStyler {
    pub(crate) fn new(event_instances: Vec<EventInstance>, main_date: chrono::NaiveDate, theme: Theme) -> Self {
        Self { event_instances, main_date, theme }
    }
}

impl Object for DateStyler {
    fn call(self: &Arc<DateStyler>, _state: &minijinja::State, args: &[Value]) -> Result<Value, minijinja::Error> {
        let (date, month, ): (&str, Option<&str>, ) = from_args(args)?;
        if let Ok(pdate) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            let month = NaiveDate::parse_from_str(month.unwrap_or(date), "%Y-%m-%d").unwrap_or(pdate);
            let mut matching_styles: Vec<MyStyle> = self.theme
                .date
                .iter()
                .filter(|datestyle| {
                    satisfy_all(
                        pdate,
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
