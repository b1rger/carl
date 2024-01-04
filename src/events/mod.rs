// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod ics;
pub use ics::ReadFromIcsFile;

use crate::utils::ChronoDate;
use chrono::prelude::*;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EventFrequency {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    None,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EventDateTime {
    DateTime {
        date_time: chrono::NaiveDateTime,
        offset: Option<chrono::offset::FixedOffset>,
    },
    Date(ChronoDate),
}

impl EventDateTime {
    pub fn date(self) -> NaiveDate {
        match self {
            EventDateTime::Date(x) => x,
            EventDateTime::DateTime { date_time, .. } => date_time.date(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub start: EventDateTime,
    pub end: EventDateTime,
    pub frequency: EventFrequency,
    pub summary: String,
}

pub type Events = Vec<Event>;

impl Default for Event {
    fn default() -> Event {
        Event {
            start: EventDateTime::Date(NaiveDate::default()),
            end: EventDateTime::Date(NaiveDate::default()),
            frequency: EventFrequency::None,
            summary: String::from("Default Event"),
        }
    }
}

impl Event {
    pub fn is_day(&self, date: &ChronoDate) -> bool {
        match self.frequency {
            EventFrequency::Weekly => {
                self.start.date() <= *date
                    && self.start.date().weekday().num_days_from_monday()
                        <= date.weekday().num_days_from_monday()
                    && date.weekday().num_days_from_monday()
                        <= self.end.date().weekday().num_days_from_monday()
            }
            EventFrequency::Daily => self.start.date() <= *date && *date <= self.end.date(),
            _ => self.in_range(*date, *date),
        }
    }

    pub fn in_range(&self, daterangebegin: ChronoDate, daterangeend: ChronoDate) -> bool {
        match self.frequency {
            EventFrequency::Yearly => {
                daterangebegin.day() <= self.start.date().day()
                    && daterangebegin.month() <= self.start.date().month()
                    && self.end.date().month() <= daterangeend.month()
                    && self.end.date().day() <= daterangeend.day()
            }
            EventFrequency::Monthly => {
                daterangebegin.day() <= self.start.date().day()
                    && self.end.date().day() <= daterangeend.day()
            }
            EventFrequency::Weekly | EventFrequency::Daily => true,
            _ => daterangebegin <= self.start.date() && self.end.date() <= daterangeend,
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
    fn test_event_is_day() {
        let event = Event::default();
        let date = NaiveDate::default();
        assert!(event.is_day(&date));
    }
    #[test]
    fn test_event_is_yearly_day() {
        let date = NaiveDate::default();
        let event = Event {
            frequency: EventFrequency::Yearly,
            end: EventDateTime::Date(date),
            ..Default::default()
        };
        assert!(event.is_day(&date));
    }
    #[test]
    fn test_event_is_monthy_day() {
        let event = Event {
            frequency: EventFrequency::Monthly,
            ..Default::default()
        };
        let date = NaiveDate::default();
        assert!(event.is_day(&date));
    }
    #[test]
    fn test_event_is_daily_day() {
        let event = Event {
            frequency: EventFrequency::Daily,
            ..Default::default()
        };
        let date = NaiveDate::default();
        assert!(event.is_day(&date));
    }
    /*#[test]
    fn test_event_get_end_date_case1() {
        let mut event = Event::default();
        let date = NaiveDateTime::default();
        event.end = Some(EventDateTime::DateTime(date));
        assert_eq!(event.get_end_date(), date);
    }*/
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
