// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::config::DateProperty;
use crate::events::EventInstance;
use chrono::prelude::*;
use chrono::Duration;
use chrono::Months;

pub trait DateExtensions {
    fn first_day_of_month(&self) -> chrono::NaiveDate;
    fn last_day_of_month(&self) -> chrono::NaiveDate;
    fn first_day_of_year(&self) -> chrono::NaiveDate;
    fn last_day_of_year(&self) -> chrono::NaiveDate;
    fn first_day_of_next_month(&self) -> chrono::NaiveDate;
    fn first_day_of_week_before_first_day_of_month(&self, from_sunday: bool) -> chrono::NaiveDate;
    fn last_day_of_week_after_last_day_of_month(&self, from_sunday: bool) -> chrono::NaiveDate;
    fn month_full_week(&self, from_sunday: bool) -> Vec<chrono::NaiveDate>;
    fn generate_dates_from_to(&self, end: chrono::NaiveDate, from_sunday: bool) -> Vec<Vec<chrono::NaiveDate>>;
    fn satisfy_all(&self, firstdayofmonth: chrono::NaiveDate, maindate: chrono::NaiveDate, events: &[EventInstance], properties: &[DateProperty]) -> bool;
}

impl DateExtensions for chrono::NaiveDate {
    fn first_day_of_month(&self) -> chrono::NaiveDate {
        let mut prevdate: chrono::NaiveDate = *self;
        while let Some(x) = prevdate.pred_opt() {
            if x.month() == prevdate.month() {
                prevdate = x; //prevdate.pred();
            } else {
                break;
            }
        }
        prevdate
    }

    fn last_day_of_month(&self) -> chrono::NaiveDate {
        let mut date: chrono::NaiveDate = self.with_day(1).unwrap();
        date = date + Months::new(1);
        date.pred_opt().unwrap()
    }

    fn first_day_of_year(&self) -> chrono::NaiveDate {
        self.with_day(1).unwrap().with_month(1).unwrap()
    }

    fn last_day_of_year(&self) -> chrono::NaiveDate {
        let mut nextdate: chrono::NaiveDate = *self;
        while let Some(x) = nextdate.succ_opt() {
            if x.year() == nextdate.year() {
                nextdate = x;
            } else {
                break;
            }
        }
        nextdate
    }

    fn first_day_of_next_month(&self) -> chrono::NaiveDate {
        if let Some(x) = self.last_day_of_month().succ_opt() {
            return x;
        }
        *self
    }

    fn first_day_of_week_before_first_day_of_month(&self, from_sunday: bool) -> chrono::NaiveDate {
        let days_to_weekstart = if from_sunday {
            self.first_day_of_month().weekday().num_days_from_sunday()
        } else {
            self.first_day_of_month().weekday().num_days_from_monday()
        };
        self.first_day_of_month() - Duration::days(days_to_weekstart.into())
    }

    fn last_day_of_week_after_last_day_of_month(&self, from_sunday: bool) -> chrono::NaiveDate {
        let last_day_plus_one_week = self.last_day_of_month() + Duration::days(7);
        let days_to_weekstart = if from_sunday {
            last_day_plus_one_week.weekday().num_days_from_sunday()
        } else {
            last_day_plus_one_week.weekday().num_days_from_monday()
        };
        last_day_plus_one_week - Duration::days(days_to_weekstart.into()) - Duration::days(1)
    }

    /// Generate a vector that contains all the dates from one month,
    /// including the days before and after the month to have only full
    /// weeks (depending on the `from_sunday` argument.
    /// This means if the first of a month is a Wednesday and `from_sunday`
    /// is true, the days Sunday, Monday, and Tuesday before the first of
    /// the month are also part of the resulting vector.
    fn month_full_week(&self, from_sunday: bool) -> Vec<chrono::NaiveDate> {
        let mut month_v: Vec<chrono::NaiveDate> = vec![];
        let mut date = self.first_day_of_month().first_day_of_week_before_first_day_of_month(from_sunday);
        while date <= self.last_day_of_month().last_day_of_week_after_last_day_of_month(from_sunday) {
            month_v.push(date);
            date += Duration::days(1);
        }
        month_v
    }

    fn generate_dates_from_to(&self, end: chrono::NaiveDate, from_sunday: bool) -> Vec<Vec<chrono::NaiveDate>> {
        let mut dates: Vec<Vec<chrono::NaiveDate>> = vec![];
        let mut months = vec![self.first_day_of_month()];
        while months.last().unwrap().first_day_of_next_month() <= end {
            months.push(months.last().unwrap().first_day_of_next_month());
        }

        for month in months {
            let month_v = month.month_full_week(from_sunday);
            dates.push(month_v)
        }
        dates
    }
    fn satisfy_all(&self, firstdayofmonth: chrono::NaiveDate, maindate: chrono::NaiveDate, events: &[EventInstance], properties: &[DateProperty]) -> bool {
        properties.iter().all(|prop| match prop {
            DateProperty::FirstDayOfMonth => *self == firstdayofmonth,
            DateProperty::BeforeFirstDayOfMonth => *self < firstdayofmonth,
            DateProperty::BeforeCurrentDate => *self < maindate,
            DateProperty::CurrentDate => *self == maindate,
            DateProperty::AfterCurrentDate => *self > maindate,
            DateProperty::AfterLastDayOfMonth => *self > firstdayofmonth.last_day_of_month(),
            DateProperty::LastDayOfMonth => *self == firstdayofmonth.last_day_of_month(),
            DateProperty::IsEvent => events
                .iter()
                .any(|eventinstance| eventinstance.date == *self),
            DateProperty::Monday => self.weekday() == chrono::Weekday::Mon,
            DateProperty::Tuesday => self.weekday() == chrono::Weekday::Tue,
            DateProperty::Wednesday => self.weekday() == chrono::Weekday::Wed,
            DateProperty::Thursday => self.weekday() == chrono::Weekday::Thu,
            DateProperty::Friday => self.weekday() == chrono::Weekday::Fri,
            DateProperty::Saturday => self.weekday() == chrono::Weekday::Sat,
            DateProperty::Sunday => self.weekday() == chrono::Weekday::Sun,
            DateProperty::Odd => self.day() % 2 == 1,
            DateProperty::Even => self.day() % 2 == 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Days, Months};

    #[test]
    fn test_first_day_of_month() {
        let i = NaiveDate::default();
        let u = i + Days::new(15);
        assert_eq!(i, u.first_day_of_month());
    }
    #[test]
    fn test_last_day_of_month() {
        let x = NaiveDate::default();
        let i = x + Days::new(30);
        let u = x + Days::new(15);
        assert_eq!(i, u.last_day_of_month());
    }
    #[test]
    fn test_first_day_of_year() {
        let i = NaiveDate::default();
        let u = i + Days::new(15);
        assert_eq!(i, u.first_day_of_year());
    }
    #[test]
    fn test_last_day_of_year() {
        let i = NaiveDate::default() + Months::new(12) - Days::new(1);
        let u = NaiveDate::default() + Months::new(2);
        assert_eq!(i, u.last_day_of_year());
    }
    #[test]
    fn test_first_day_of_next_month() {
        let i = NaiveDate::default() + Months::new(2);
        let u = NaiveDate::default() + Months::new(1);
        assert_eq!(i, u.first_day_of_next_month());
    }
    #[test]
    fn test_first_day_of_week_before_first_day_of_month() {
        let i = NaiveDate::default() + Months::new(3) + Days::new(26);
        let u = NaiveDate::default() + Months::new(4);
        assert_eq!(i, u.first_day_of_week_before_first_day_of_month(false));
    }
    #[test]
    fn test_last_day_of_week_after_last_day_of_month() {
        let i = NaiveDate::default() + Months::new(4) + Days::new(2);
        let u = NaiveDate::default() + Months::new(3);
        assert_eq!(i, u.last_day_of_week_after_last_day_of_month(false));
    }
    #[test]
    fn test_first_day_of_week_before_first_day_of_month_sunday() {
        let i = NaiveDate::default() + Months::new(2) + Days::new(28);
        let u = NaiveDate::default() + Months::new(3);
        assert_eq!(i, u.first_day_of_week_before_first_day_of_month(true));
    }
    #[test]
    fn test_last_day_of_week_after_last_day_of_month_sunday() {
        let i = NaiveDate::default() + Months::new(4) + Days::new(1);
        let u = NaiveDate::default() + Months::new(3);
        assert_eq!(i, u.last_day_of_week_after_last_day_of_month(true));
    }
}
