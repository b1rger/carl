use std::sync::Arc;
use crate::events::EventInstance;
use minijinja::value::{Object, Value, from_args};
use minijinja::value::ViaDeserialize;
use crate::config::Theme;
use crate::utils::helpers::{satisfy_all, tostyle};
use crate::utils::DateExtensions;
use chrono::NaiveDate;
use crate::config::Style as MyStyle;

pub mod filters;


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

#[derive(Debug)]
pub(crate) struct DateToColumnsGenerator {}

impl DateToColumnsGenerator {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Object for DateToColumnsGenerator {
    fn call(self: &Arc<DateToColumnsGenerator>, _state: &minijinja::State, args: &[Value]) -> Result<Value, minijinja::Error> {
        let (dates, columns, ): (ViaDeserialize<Vec<Vec<chrono::NaiveDate>>>, usize) = from_args(args)?;
        let mut months_columns: Vec<Vec<Vec<chrono::NaiveDate>>> = vec![];
        for chunk in dates.chunks(columns) {
            months_columns.push(chunk.to_vec());
        }
        let mut ret: Vec<Vec<Vec<Option<chrono::NaiveDate>>>> = vec![];
        for mut row in months_columns {
            let mut monthlines: Vec<Vec<Option<chrono::NaiveDate>>> = vec![];
            while row.iter().any(|x| !x.is_empty()) {
                let mut line: Vec<Option<chrono::NaiveDate>> = vec![];
                for month in &mut row {
                    let mut foo: Vec<Option<chrono::NaiveDate>> = if month.len() >= 7 {
                        month.drain(..7).map(|x| Some(x)).collect()
                    } else {
                        vec![None; 7]
                    };
                    line.append(&mut foo);
                }
                monthlines.push(line);
            }
            ret.push(monthlines)
        }
        Ok(Value::from_serialize(ret))
    }
}
