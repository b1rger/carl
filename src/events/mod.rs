// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
extern crate ical;

mod ics;
pub use ics::ReadFromIcsFile;

use crate::lib::types::{ChronoDate, ChronoDateTime};
use chrono::prelude::*;
use chrono::Days;
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
    DateTime(ChronoDateTime),
    Date(ChronoDate),
}

#[derive(Debug, Clone)]
pub struct Event {
    pub start: EventDateTime,
    pub end: Option<EventDateTime>,
    pub frequency: EventFrequency,
    pub summary: String,
}

pub type Events = Vec<Event>;

impl Default for Event {
    fn default() -> Event {
        Event {
            start: EventDateTime::Date(NaiveDate::default()),
            end: None,
            frequency: EventFrequency::None,
            summary: String::from("Default Event"),
        }
    }
}

impl Event {
    pub fn is_day(&self, date: &ChronoDate) -> bool {
        self.in_range(*date, *date)
    }

    pub fn in_range(&self, daterangebegin: ChronoDate, daterangeend: ChronoDate) -> bool {
        let start = self.get_start_date();
        let end = self.get_end_date();

        match self.frequency {
            EventFrequency::Yearly => {
                daterangebegin.day() <= start.day()
                    && daterangebegin.month() <= start.month()
                    && end.month() <= daterangeend.month()
                    && end.day() <= daterangeend.day()
            }
            EventFrequency::Monthly => {
                daterangebegin.day() <= start.day() && end.day() <= daterangeend.day()
            }
            EventFrequency::Daily => true,
            EventFrequency::Weekly => todo!(),
            _ => daterangebegin <= start && end <= daterangeend,
        }
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
            self.summary.replace("\\", "")
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
        let mut event = Event::default();
        event.frequency = EventFrequency::Yearly;
        let date = NaiveDate::default();
        event.end = Some(EventDateTime::Date(date));
        assert!(event.is_day(&date));
    }
    #[test]
    fn test_event_is_monthy_day() {
        let mut event = Event::default();
        event.frequency = EventFrequency::Monthly;
        let date = NaiveDate::default();
        assert!(event.is_day(&date));
    }
    #[test]
    fn test_event_is_daily_day() {
        let mut event = Event::default();
        event.frequency = EventFrequency::Daily;
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
        let mut event = Event::default();
        let date = NaiveDate::default();
        event.start = EventDateTime::Date(date);
        event.end = Some(EventDateTime::Date(date));
        assert_eq!(event.get_end_date(), date);
    }
    #[test]
    fn test_event_get_end_date_case3() {
        let mut event = Event::default();
        let date = NaiveDate::default();
        event.start = EventDateTime::Date(date);
        event.end = Some(EventDateTime::Date(date + Days::new(1)));
        assert_eq!(event.get_end_date(), date);
    }
    #[test]
    fn test_event_fmt_date() {
        let mut event = Event::default();
        let date = NaiveDate::default();
        event.start = EventDateTime::Date(date);
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
