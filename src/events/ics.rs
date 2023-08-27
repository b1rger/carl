// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

extern crate ical;

use crate::events::{Event, EventDateTime, Events};
use crate::utils::DateRange;
use chrono::prelude::*;
use ical::parser::ical::component::IcalEvent;
use rrule::RRuleSet;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

impl TryFrom<ical::property::Property> for EventDateTime {
    type Error = &'static str;

    fn try_from(property: ical::property::Property) -> Result<Self, Self::Error> {
        if let Some(x) = property.value {
            if let Some(y) = property.params {
                if y.iter().any(|(paramname, params)| {
                    paramname == "VALUE" && params.contains(&"DATE".to_string())
                }) {
                    if let Ok(naive_date) = NaiveDate::parse_from_str(&x, "%Y%m%d") {
                        return Ok(EventDateTime::Date(naive_date));
                    }
                } else if let Ok(naive_datetime) =
                    NaiveDateTime::parse_from_str(&x, "%Y%m%dT%H%M%S")
                {
                    if let Some(x) = Local.from_local_datetime(&naive_datetime).single() {
                        return Ok(EventDateTime::DateTime(x));
                    }
                }
            }
        }
        Err("Could not parse ical property.")
    }
}

impl TryFrom<IcalEvent> for Event {
    type Error = &'static str;

    fn try_from(event: IcalEvent) -> Result<Self, Self::Error> {
        let mut start: Option<EventDateTime> = None;
        let mut end: Option<EventDateTime> = None;
        let mut summary: String = String::new();
        let mut rrulesets: Vec<RRuleSet> = vec![];
        let mut rrule: Option<String> = None;
        for property in event.properties {
            match property.name.as_ref() {
                "DTSTART" => {
                    start = EventDateTime::try_from(property).ok();
                }
                "DTEND" => {
                    end = EventDateTime::try_from(property).ok();
                }
                "SUMMARY" => {
                    if let Some(x) = property.value {
                        summary = x;
                    }
                }
                "RRULE" => {
                    if let Some(value) = property.value {
                        rrule = Some(format!("RRULE:{}", value));
                    }
                }
                _ => {}
            }
        }
        if let Some(x) = start {
            if let Some(rrule) = rrule {
                let end = match end {
                    Some(y) => y,
                    _ => x,
                };
                for date in DateRange(
                    x.date() - chrono::Duration::days(1),
                    end.date() - chrono::Duration::days(1),
                ) {
                    let rrulestr =
                        format!("DTSTART;VALUE=DATE:{}\n{}", date.format("%Y%m%d"), rrule);
                    rrulesets.push(rrulestr.parse().unwrap());
                }
            }
            Ok(Event {
                start: x,
                end,
                summary,
                rrulesets,
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
            if let Ok(f) = File::open(filepath) {
                let buf = BufReader::new(f);
                let mut reader = ical::IcalParser::new(buf);

                while let Some(Ok(cal)) = reader.next() {
                    for event in cal.events {
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
    use chrono::{Local, TimeZone};

    #[test]
    fn test_property_to_eventdatetime_1() {
        let property: ical::property::Property = ical::property::Property {
            name: String::from("DTSTART"),
            params: Some(vec![(String::from("VALUE"), vec![String::from("DATE")])]),
            value: Some(String::from("19700101")),
        };
        let date = NaiveDate::default();
        assert_eq!(
            EventDateTime::try_from(property),
            Ok(EventDateTime::Date(date))
        );
    }
    #[test]
    fn test_property_to_eventdatetime_2() {
        let property: ical::property::Property = ical::property::Property {
            name: String::from("DTSTART"),
            params: Some(vec![]),
            value: Some(String::from("19700101T010130")),
        };
        let date = Local.timestamp_opt(90, 0).unwrap();
        assert_eq!(
            EventDateTime::try_from(property),
            Ok(EventDateTime::DateTime(date))
        );
    }
    #[test]
    fn test_property_to_eventdatetime_err() {
        let property: ical::property::Property = ical::property::Property {
            name: String::from("DTSTART"),
            params: Some(vec![]),
            value: Some(String::from("19700101010130")),
        };
        assert!(EventDateTime::try_from(property).is_err());
    }
    #[test]
    fn test_icalevent_to_event() {
        let dtstart: ical::property::Property = ical::property::Property {
            name: String::from("DTSTART"),
            params: Some(vec![]),
            value: Some(String::from("19700101T010130")),
        };
        let dtend: ical::property::Property = ical::property::Property {
            name: String::from("DTEND"),
            params: Some(vec![]),
            value: Some(String::from("19700101T010130")),
        };
        let summary: ical::property::Property = ical::property::Property {
            name: String::from("SUMMARY"),
            params: None,
            value: Some(String::from("Some summary")),
        };
        let icalevent: IcalEvent = IcalEvent {
            alarms: vec![],
            properties: vec![dtstart, dtend, summary],
        };
        assert!(Event::try_from(icalevent).is_ok());
    }
    #[test]
    fn test_icalevent_to_event_err() {
        let dtstart: ical::property::Property = ical::property::Property {
            name: String::from("DTSTART"),
            params: Some(vec![]),
            value: Some(String::from("19700101010130")),
        };
        let icalevent: IcalEvent = IcalEvent {
            alarms: vec![],
            properties: vec![dtstart],
        };
        assert!(Event::try_from(icalevent).is_err());
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
