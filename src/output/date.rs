// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::config::{DateProperty, Style, StyleType};
use crate::utils::{convertstyle, DateExtensions};
use crate::Context;
use chrono::Datelike;

use std::fmt;

pub struct Date<'a> {
    pub date: chrono::NaiveDate,
    pub ctx: &'a Context,
    pub firstdayofdisplayedmonth: chrono::NaiveDate,
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
            DateProperty::Monday => self.date.weekday() == chrono::Weekday::Mon,
            DateProperty::Tuesday => self.date.weekday() == chrono::Weekday::Tue,
            DateProperty::Wednesday => self.date.weekday() == chrono::Weekday::Wed,
            DateProperty::Thursday => self.date.weekday() == chrono::Weekday::Thu,
            DateProperty::Friday => self.date.weekday() == chrono::Weekday::Fri,
            DateProperty::Saturday => self.date.weekday() == chrono::Weekday::Sat,
            DateProperty::Sunday => self.date.weekday() == chrono::Weekday::Sun,
            DateProperty::Odd => self.date.day() % 2 == 1,
            DateProperty::Even => self.date.day() % 2 == 0,
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
        let dateformat: String = if self.ctx.opts.julian {
            format!("{:>3}", self.date.format("%j"))
        } else {
            format!("{:>2}", self.date.format("%e"))
        };

        let mut styles: Vec<Style> = self
            .ctx
            .theme
            .date
            .iter()
            .filter(|datestyle| self.satisfy_all(&datestyle.properties))
            .cloned()
            .map(|datestyle| datestyle.style)
            .collect();

        for (event, style) in &self.ctx.eventstuple {
            if event.is_day(&self.date) {
                styles.push(style.clone());
            }
        }

        styles.retain(|style| {
            style.styletype == self.ctx.styletype || style.styletype == StyleType::None
        });

        styles.sort_by(|a, b| a.weight.cmp(&b.weight));
        let mut stylenames = vec![];
        for mut style in styles {
            stylenames.append(&mut style.stylenames);
        }

        write!(f, "{}", convertstyle(stylenames, &dateformat))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use chrono::NaiveDate;

    #[test]
    fn test_fmt_0() {
        let date = NaiveDate::default();
        let ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::default();
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        let s = String::from("\u{1b}[1m\u{1b}[4m 1\u{1b}[0m");
        assert_eq!(format!("{}", d), s);
    }

    #[test]
    fn test_fmt_1() {
        let date = NaiveDate::from_ymd_opt(1970, 1, 15).unwrap();
        let ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::default();
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
        let date = NaiveDate::from_ymd_opt(1970, 2, 1).unwrap();
        let ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::default();
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
        let date = NaiveDate::from_ymd_opt(1970, 1, 15).unwrap();
        let mut ctx = Context::default();
        ctx.opts.julian = true;
        let firstdayofdisplayedmonth = NaiveDate::default();
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
        let date = NaiveDate::default();
        let mut ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::default();

        let event = Event::default();
        let style = Style::default();
        ctx.eventstuple = vec![(event, style)];
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };

        let s = String::from("\u{1b}[1m\u{1b}[4m 1\u{1b}[0m");
        assert_eq!(format!("{}", d), s);
    }

    #[test]
    fn test_satisfy_firstdayofdisplayedmonth() {
        let date = NaiveDate::from_ymd_opt(1970, 2, 1).unwrap();
        let ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::from_ymd_opt(1970, 2, 1).unwrap();
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
        let date = NaiveDate::from_ymd_opt(1970, 2, 1).unwrap();
        let ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::from_ymd_opt(1970, 2, 1).unwrap();
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
        let date = NaiveDate::from_ymd_opt(1970, 1, 31).unwrap();
        let ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::default();
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
        let date = NaiveDate::from_ymd_opt(1970, 1, 31).unwrap();
        let ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::default();
        let d = Date {
            date,
            ctx: &ctx,
            firstdayofdisplayedmonth,
        };
        assert!(!d.is_event());
    }
    #[test]
    fn test_satisfy_is_event() {
        let date = NaiveDate::default();
        let mut ctx = Context::default();
        let firstdayofdisplayedmonth = NaiveDate::default();

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
