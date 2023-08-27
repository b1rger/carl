// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

extern crate ical;

mod ics;
pub use ics::ReadFromIcsFile;

use crate::utils::{ChronoDate, ChronoDateTime, DateRange};
use chrono::prelude::*;
use chrono::Days;
use rrule::RRuleSet;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EventDateTime {
    DateTime(ChronoDateTime),
    Date(ChronoDate),
}

impl EventDateTime {
    fn date(self) -> NaiveDate {
        match self {
            EventDateTime::DateTime(x) => x.date_naive(),
            EventDateTime::Date(x) => x,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub start: EventDateTime,
    pub end: Option<EventDateTime>,
    pub summary: String,
    pub rrulesets: Vec<RRuleSet>,
}

pub type Events = Vec<Event>;

impl Default for Event {
    fn default() -> Event {
        Event {
            start: EventDateTime::Date(NaiveDate::default()),
            end: None,
            summary: String::from("Default Event"),
            rrulesets: vec![],
        }
    }
}

impl Event {
    pub fn is_day(&self, date: &ChronoDate) -> bool {
        self.in_range(*date, *date)
    }

    pub fn in_range(&self, daterangebegin: ChronoDate, daterangeend: ChronoDate) -> bool {
        for rruleset in &self.rrulesets {
            let rresult = rruleset.clone().all(365);
            let resultdates: Vec<NaiveDate> =
                rresult.dates.iter().map(|date| date.date_naive()).collect();

            let mut rangedates: Vec<NaiveDate> = DateRange(daterangebegin, daterangeend).collect();
            rangedates.push(daterangebegin);

            if rangedates.iter().any(|&x| resultdates.contains(&x)) {
                return true;
            }
        }

        let start = self.get_start_date();
        let end = self.get_end_date();
        daterangebegin <= start && end <= daterangeend
    }

    pub fn get_start_date(&self) -> ChronoDate {
        match self.start {
            EventDateTime::DateTime(x) => x.date_naive(),
            EventDateTime::Date(x) => x,
        }
    }

    fn get_end_date(&self) -> ChronoDate {
        match self.end {
            Some(x) => match x {
                EventDateTime::DateTime(y) => y.date_naive(),
                EventDateTime::Date(y) => match self.start {
                    EventDateTime::Date(z) => {
                        if z + Days::new(1) == y {
                            z
                        } else {
                            y
                        }
                    }
                    _ => y,
                },
            },
            None => self.get_start_date(),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let startformatstring = match self.start {
            EventDateTime::DateTime(x) => x.format("%a, %b, %e (%H:%M)"),
            EventDateTime::Date(x) => x.format("%a, %b, %e"),
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
            end: Some(EventDateTime::Date(date)),
            ..Default::default()
        };
        assert!(event.is_day(&date));
    }
    #[test]
    fn test_event_is_monthy_day() {
        let event = Event::default();
        let date = NaiveDate::default();
        assert!(event.is_day(&date));
    }
    #[test]
    fn test_event_is_daily_day() {
        let event = Event::default();
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
            end: Some(EventDateTime::Date(date)),
            ..Default::default()
        };
        assert_eq!(event.get_end_date(), date);
    }
    #[test]
    fn test_event_get_end_date_case3() {
        let date = NaiveDate::default();
        let event = Event {
            start: EventDateTime::Date(date),
            end: Some(EventDateTime::Date(date + Days::new(1))),
            ..Default::default()
        };
        assert_eq!(event.get_end_date(), date);
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
