// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
use crate::config::{Style, StyleConversion, StyleType};
use crate::lib::types::ChronoDate;
use crate::lib::DateExtensions;
use crate::Context;
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DateProperty {
    FirstDayOfMonth,
    BeforeFirstDayOfMonth,
    BeforeCurrentDate,
    CurrentDate,
    AfterCurrentDate,
    AfterLastDayOfMonth,
    LastDayOfMonth,
    IsEvent,
}

pub struct Date<'a> {
    pub date: ChronoDate,
    pub ctx: &'a Context,
    pub firstdayofdisplayedmonth: ChronoDate,
}

impl Date<'_> {
    pub fn satisfy_all(&self, properties: &[DateProperty]) -> bool {
        properties.iter().all(|prop| match prop {
            DateProperty::FirstDayOfMonth => self.date == self.firstdayofdisplayedmonth,
            DateProperty::BeforeFirstDayOfMonth => self.date < self.firstdayofdisplayedmonth,
            DateProperty::BeforeCurrentDate => self.date < self.ctx.usersetdate,
            DateProperty::CurrentDate => self.date == self.ctx.usersetdate,
            DateProperty::AfterCurrentDate => self.date > self.ctx.usersetdate,
            DateProperty::AfterLastDayOfMonth => {
                self.date > self.firstdayofdisplayedmonth.last_day_of_month()
            }
            DateProperty::LastDayOfMonth => {
                self.date == self.firstdayofdisplayedmonth.last_day_of_month()
            }
            DateProperty::IsEvent => self.is_event(),
        })
    }

    fn is_event(&self) -> bool {
        for (event, _) in &self.ctx.eventstuple {
            if event.is_day(&self.date) {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Date<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dateformat: String;

        if self.ctx.opts.julian {
            dateformat = format!("{:>3}", self.date.format("%j"));
        } else {
            dateformat = format!("{:>2}", self.date.format("%e"));
        }

        let mut styles: Vec<Style> = self
            .ctx
            .theme
            .date
            .to_vec()
            .into_iter()
            .filter(|datestyle| self.satisfy_all(&datestyle.properties))
            .map(|datestyle| datestyle.style)
            .collect();

        for (event, style) in &self.ctx.eventstuple {
            if event.is_day(&self.date) {
                styles.push(style.clone());
            }
        }

        styles = styles
            .into_iter()
            .filter(|style| {
                style.styletype == self.ctx.styletype || style.styletype == StyleType::None
            })
            .collect();

        styles.sort_by(|a, b| a.weight.cmp(&b.weight));
        let mut stylenames = vec![];
        for mut style in styles {
            stylenames.append(&mut style.stylenames);
        }

        write!(
            f,
            "{}",
            stylenames.to_style().paint(&dateformat).to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use chrono::{Local, TimeZone};

    #[test]
    fn test_fmt_0() {
        let date = Local.ymd(1970, 1, 1);
        let ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let s = String::from("\u{1b}[1;4m 1\u{1b}[0m");
        assert_eq!(format!("{}", d), s);
    }

    #[test]
    fn test_fmt_1() {
        let date = Local.ymd(1970, 1, 15);
        let ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let s = String::from("15");
        assert_eq!(format!("{}", d), s);
    }

    #[test]
    fn test_fmt_2() {
        let date = Local.ymd(1970, 2, 1);
        let ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let s = String::from("\u{1b}[8m 1\u{1b}[0m");
        assert_eq!(format!("{}", d), s);
    }
    #[test]
    fn test_fmt_is_julian() {
        let date = Local.ymd(1970, 1, 15);
        let mut ctx = Context::default();
        ctx.opts.julian = true;
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let s = String::from("015");
        assert_eq!(format!("{}", d), s);
    }
    #[test]
    fn test_fmt_is_event() {
        let date = Local.ymd(1970, 1, 1);
        let mut ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);

        let event = Event::default();
        let style = Style::default();
        ctx.eventstuple = vec![(event, style)];
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };

        let s = String::from("\u{1b}[1;4m 1\u{1b}[0m");
        assert_eq!(format!("{}", d), s);
    }

    #[test]
    fn test_satisfy_firstdayofdisplayedmonth() {
        let date = Local.ymd(1970, 2, 1);
        let ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 2, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let properties = [DateProperty::FirstDayOfMonth];
        assert!(d.satisfy_all(&properties));
    }
    #[test]
    fn test_satisfy_aftercurrentdate() {
        let date = Local.ymd(1970, 2, 1);
        let ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 2, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let properties = [DateProperty::AfterCurrentDate];
        assert!(d.satisfy_all(&properties));
    }
    #[test]
    fn test_satisfy_afterlastdayofmonth() {
        let date = Local.ymd(1970, 1, 31);
        let ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let properties = [DateProperty::LastDayOfMonth];
        assert!(d.satisfy_all(&properties));
    }
    #[test]
    fn test_is_event() {
        let date = Local.ymd(1970, 1, 31);
        let ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        assert!(!d.is_event());
    }
    #[test]
    fn test_satisfy_is_event() {
        let date = Local.ymd(1970, 1, 1);
        let mut ctx = Context::default();
        let firstdayofdisplayedmonth = Local.ymd(1970, 1, 1);

        let event = Event::default();
        let style = Style::default();
        ctx.eventstuple = vec![(event, style)];
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };

        let properties = [DateProperty::IsEvent];
        assert!(d.satisfy_all(&properties));
    }
}
