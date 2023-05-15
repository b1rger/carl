// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
extern crate ical;

use crate::events::{Event, EventDateTime, EventFrequency, Events};
use chrono::prelude::*;
use ical::parser::ical::component::IcalEvent;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

impl From<ical::property::Property> for EventFrequency {
    fn from(property: ical::property::Property) -> Self {
        let mut ret = EventFrequency::None;
        if let Some(values) = property.value {
            let values = values.split(';');
            for val in values {
                match val {
                    "FREQ=YEARLY" => ret = EventFrequency::Yearly,
                    "FREQ=MONTHLY" => ret = EventFrequency::Monthly,
                    "FREQ=WEEKLY" => ret = EventFrequency::Weekly,
                    "FREQ=DAILY" => ret = EventFrequency::Daily,
                    _ => (),
                }
            }
        }
        ret
    }
}

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
        let mut frequency: EventFrequency = EventFrequency::None;
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
                    frequency = EventFrequency::from(property);
                }
                _ => {}
            }
        }
        if let Some(x) = start {
            Ok(Event {
                start: x,
                end,
                frequency,
                summary,
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
                for entry in path {
                    if let Ok(entry) = entry {
                        if entry.path().is_file() {
                            filepaths.push(entry.path());
                        }
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
    fn test_property_to_frequency_yearly() {
        let property: ical::property::Property = ical::property::Property {
            name: String::from("RRULE"),
            params: None,
            value: Some(String::from("FREQ=YEARLY")),
        };
        assert_eq!(EventFrequency::from(property), EventFrequency::Yearly);
    }
    #[test]
    fn test_property_to_frequency_monthly() {
        let property: ical::property::Property = ical::property::Property {
            name: String::from("RRULE"),
            params: None,
            value: Some(String::from("FREQ=MONTHLY")),
        };
        assert_eq!(EventFrequency::from(property), EventFrequency::Monthly);
    }
    #[test]
    fn test_property_to_frequency_weekly() {
        let property: ical::property::Property = ical::property::Property {
            name: String::from("RRULE"),
            params: None,
            value: Some(String::from("FREQ=WEEKLY")),
        };
        assert_eq!(EventFrequency::from(property), EventFrequency::Weekly);
    }
    #[test]
    fn test_property_to_frequency_daily() {
        let property: ical::property::Property = ical::property::Property {
            name: String::from("RRULE"),
            params: None,
            value: Some(String::from("FREQ=DAILY")),
        };
        assert_eq!(EventFrequency::from(property), EventFrequency::Daily);
    }
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
        let frequency: ical::property::Property = ical::property::Property {
            name: String::from("RRULE"),
            params: None,
            value: Some(String::from("FREQ=YEARLY")),
        };
        let summary: ical::property::Property = ical::property::Property {
            name: String::from("SUMMARY"),
            params: None,
            value: Some(String::from("Some summary")),
        };
        let icalevent: IcalEvent = IcalEvent {
            alarms: vec![],
            properties: vec![dtstart, dtend, frequency, summary],
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
