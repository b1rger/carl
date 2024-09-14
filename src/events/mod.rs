// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod ics;
pub use ics::ReadFromIcsFile;

use crate::config::Style;
use crate::utils::convertstyle;
use chrono::prelude::*;
use chrono::Duration;
use rrule::{RRuleSet, Tz};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventDateTime {
    DateTime {
        date_time: chrono::NaiveDateTime,
        offset: Option<i32>,
    },
    Date(chrono::NaiveDate),
}

impl EventDateTime {
    pub fn date(self) -> NaiveDate {
        match self {
            EventDateTime::Date(x) => x,
            EventDateTime::DateTime { date_time, .. } => date_time.date(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstance {
    pub date: chrono::NaiveDate,
    pub event: Event,
    pub style: Style,
}

impl fmt::Display for EventInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style = convertstyle(self.style.stylenames.to_vec(), "·");
        write!(f, "{} {}: {}", style, self.date, self.event.summary)
    }
}

pub type EventInstances = Vec<EventInstance>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub start: EventDateTime,
    pub end: EventDateTime,
    pub rrulesets: Vec<RRuleSet>,
    pub summary: String,
}

impl Event {
    pub fn instances(&self, start: &NaiveDate, end: &NaiveDate, style: &Style) -> EventInstances {
        let timezone: Tz = Local::now().timezone().into();
        let before = timezone
            .with_ymd_and_hms(end.year(), end.month(), end.day(), 23, 59, 59)
            .unwrap();
        let after = timezone
            .with_ymd_and_hms(start.year(), start.month(), start.day(), 0, 0, 0)
            .unwrap();
        let duration = *end - *start;
        let mut eventinstances: EventInstances = vec![];
        if self.rrulesets.is_empty() {
            let mut date = self.start.date();
            if date == self.end.date() {
                if start <= &date && &date <= end {
                    eventinstances.push(EventInstance {
                        date,
                        event: self.clone(),
                        style: style.clone(),
                    });
                }
            } else {
                while date < self.end.date() {
                    if start <= &date && &date <= end {
                        eventinstances.push(EventInstance {
                            date,
                            event: self.clone(),
                            style: style.clone(),
                        });
                    }
                    date += Duration::days(1);
                }
            }
        } else {
            for rruleset in &self.rrulesets {
                let ruleset = rruleset
                    .clone()
                    .before(before)
                    .after(after)
                    .all(duration.num_days() as u16);
                eventinstances.append(
                    &mut ruleset
                        .dates
                        .iter()
                        .map(|date| EventInstance {
                            date: date.date_naive(),
                            event: self.clone(),
                            style: style.clone(),
                        })
                        .collect::<EventInstances>(),
                );
            }
        }
        eventinstances
    }
}

pub type Events = Vec<Event>;

impl Default for Event {
    fn default() -> Event {
        Event {
            start: EventDateTime::Date(NaiveDate::default()),
            end: EventDateTime::Date(NaiveDate::default()),
            rrulesets: vec![],
            summary: String::from("Default Event"),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let startformatstring = match self.start {
            EventDateTime::DateTime { date_time, offset } => {
                if let Some(x) = offset {
                    format!("{} ({})", date_time.format("%a, %b, %e (%H:%M)"), x)
                } else {
                    date_time.format("%a, %b, %e (%H:%M)").to_string()
                }
            }
            EventDateTime::Date(x) => x.format("%a, %b, %e").to_string(),
        };
        write!(
            f,
            "{}: {}",
            startformatstring,
            self.summary.replace('\\', "")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_default() {
        let event = Event::default();
        let date = NaiveDate::default();
        assert_eq!(event.start, EventDateTime::Date(date));
    }
    #[test]
    fn test_event_get_end_date_case2() {
        let date = NaiveDate::default();
        let event = Event {
            start: EventDateTime::Date(date),
            end: EventDateTime::Date(date),
            ..Default::default()
        };
        assert_eq!(event.end.date(), date);
    }
    #[test]
    fn test_event_get_end_date_case3() {
        let date = NaiveDate::default();
        let event = Event {
            start: EventDateTime::Date(date),
            end: EventDateTime::Date(date),
            ..Default::default()
        };
        assert_eq!(event.end.date(), date);
    }
    #[test]
    fn test_event_fmt_date() {
        let date = NaiveDate::default();
        let event = Event {
            start: EventDateTime::Date(date),
            ..Default::default()
        };
        assert_eq!(
            format!("{}", event),
            String::from("Thu, Jan,  1: Default Event")
        );
    }
    #[test]
    fn test_event_fmt_datetime() {
        let event = Event::default();
        assert_eq!(
            format!("{}", event),
            String::from("Thu, Jan,  1: Default Event")
        );
    }
}
