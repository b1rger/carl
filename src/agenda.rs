// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
use crate::config::StyleConversion;
use crate::Context;
use chrono::Datelike;

use std::fmt;

pub struct Agenda<'a> {
    pub ctx: &'a Context,
}

impl fmt::Display for Agenda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret: String = String::new();
        if self.ctx.opts.agenda {
            ret = "Agenda\n".to_string();
            let eventstuple = &mut self.ctx.eventstuple.clone();
            eventstuple.sort_by(|(aevent, _), (bevent, _)| {
                aevent
                    .get_start_date()
                    .month()
                    .cmp(&bevent.get_start_date().month())
                    .then(
                        aevent
                            .get_start_date()
                            .day()
                            .cmp(&bevent.get_start_date().day()),
                    )
            });
            for (event, style) in eventstuple {
                ret += format!("{} {}\n", style.stylenames.to_style().paint("·"), event).as_str();
            }
        }
        write!(f, "{}", ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Style;
    use crate::events::{Event, EventDateTime};
    use chrono::{Duration, NaiveDate};

    #[test]
    fn test_fmt() {
        let mut ctx = Context::default();
        ctx.opts.agenda = true;
        let e1: Event = Event {
            summary: String::from("Fake Event"),
            ..Default::default()
        };
        let e2: Event = Event {
            start: EventDateTime::Date(NaiveDate::default() + Duration::weeks(56)),
            summary: String::from("Fake Event"),
            ..Default::default()
        };
        let s1: Style = Style::default();
        let s2: Style = Style::default();
        ctx.eventstuple.push((e1, s1));
        ctx.eventstuple.push((e2, s2));
        let a = Agenda { ctx: &ctx };
        assert_eq![
            format!("{}", a),
            String::from("Agenda\n· Thu, Jan,  1: Fake Event\n· Thu, Jan, 28: Fake Event\n")
        ];
    }
}
