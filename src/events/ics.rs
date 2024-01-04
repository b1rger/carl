// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::events::{Event, EventDateTime, Events};
use crate::utils::DateRange;
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event as IcalendarEvent};
use rrule::RRuleSet;
use std::path::{Path, PathBuf};

impl From<icalendar::DatePerhapsTime> for EventDateTime {
    fn from(dateperhapstime: icalendar::DatePerhapsTime) -> Self {
        match dateperhapstime {
            DatePerhapsTime::DateTime(dt) => {
                let date_time = match dt {
                    CalendarDateTime::Floating(date_time) => date_time,
                    CalendarDateTime::Utc(date_time) => date_time.naive_utc(),
                    CalendarDateTime::WithTimezone { date_time, tzid: _ } => date_time,
                };
                EventDateTime::DateTime {
                    date_time,
                    offset: None,
                }
            }
            DatePerhapsTime::Date(naive_date) => EventDateTime::Date(naive_date),
        }
    }
}

impl TryFrom<&IcalendarEvent> for Event {
    type Error = &'static str;

    fn try_from(event: &IcalendarEvent) -> Result<Self, Self::Error> {
        let mut rrulestring = String::new();
        if event.properties().contains_key("RRULE") {
            let p = event.properties()["RRULE"].clone();
            if let Ok(x) = TryInto::<String>::try_into(p) {
                rrulestring = x;
            }
        }

        if let Some(x) = event.get_start() {
            let start: EventDateTime = x.into();
            let end: EventDateTime = match event.get_end() {
                Some(y) => y.into(),
                _ => start,
            };
            let mut rrulesets: Vec<RRuleSet> = vec![];
            if !rrulestring.is_empty() {
                for date in DateRange(start.date(), end.date()) {
                    let rrule = format!(
                        "DTSTART;VALUE=DATE:{}\n{rrulestring}",
                        date.format("%Y%m%d")
                    );
                    if let Ok(x) = rrule.parse() {
                        rrulesets.push(x);
                    }
                }
            }
            Ok(Event {
                start,
                end,
                rrulesets,
                summary: event.get_summary().unwrap_or_default().to_string(),
            })
        } else {
            Err("Could not parse ical event.")
        }
    }
}

pub trait ReadFromIcsFile {
    fn read_from_ics_file(filepath: &str) -> Events;
}

impl ReadFromIcsFile for Events {
    fn read_from_ics_file(filepath: &str) -> Events {
        let mut events: Events = vec![];
        let mut filepaths: Vec<PathBuf> = vec![];

        let path = Path::new(filepath);
        if path.is_dir() {
            if let Ok(path) = path.read_dir() {
                for entry in path.flatten() {
                    if entry.path().is_file() {
                        filepaths.push(entry.path());
                    }
                }
            } else {
                eprintln!("Could not read dir {}", filepath);
            }
        } else {
            filepaths.push(path.to_path_buf())
        }

        for filepath in filepaths.iter() {
            if let Ok(contents) = std::fs::read_to_string(filepath) {
                if let Ok(calendar) = contents.parse::<Calendar>() {
                    for event in calendar
                        .components
                        .iter()
                        .filter_map(|component| component.as_event())
                        .collect::<Vec<&IcalendarEvent>>()
                    {
                        if let Ok(e) = Event::try_from(event) {
                            events.push(e);
                        }
                    }
                }
            } else {
                eprintln!("Could not read file {}", filepath.display());
            }
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icalevent_to_event() {
        let mut icalevent = IcalendarEvent::default();
        icalevent.add_property("DTSTART", "19700101T010130");
        assert!(Event::try_from(&icalevent).is_ok());
    }
    #[test]
    fn test_icalevent_to_event_err() {
        let mut icalevent = IcalendarEvent::default();
        icalevent.add_property("DTSTART", "19700101010130");
        assert!(Event::try_from(&icalevent).is_err());
    }
    #[test]
    fn test_read_from_ics_file() {
        let filename = "foobar.ics";
        assert!(Events::read_from_ics_file(filename).is_empty());
    }
    #[test]
    fn test_read_from_dir() {
        let filename = "/tmp";
        assert!(Events::read_from_ics_file(filename).is_empty());
    }
    #[test]
    fn test_read_from_dir_nor() {
        let filename = "/root";
        assert!(Events::read_from_ics_file(filename).is_empty());
    }
    #[test]
    fn test_read_from_carl_ics() {
        let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/data/carl.ics");
        assert_eq!(Events::read_from_ics_file(filename).len(), 21);
    }
}
