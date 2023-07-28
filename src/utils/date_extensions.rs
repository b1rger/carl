// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use chrono::prelude::*;
use chrono::Days;
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
    fn month_full_weeks_iter(&self, from_sunday: bool) -> MonthFullWeeksIter;
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

    fn month_full_weeks_iter(&self, from_sunday: bool) -> MonthFullWeeksIter {
        MonthFullWeeksIter {
            base_date: *self,
            current_date: self.first_day_of_week_before_first_day_of_month(from_sunday),
            from_sunday,
        }
    }
}

pub struct MonthFullWeeksIter {
    pub base_date: chrono::NaiveDate,
    current_date: chrono::NaiveDate,
    from_sunday: bool,
}

impl MonthFullWeeksIter {
    pub fn empty(&self) -> bool {
        self.current_date
            >= self
                .base_date
                .last_day_of_week_after_last_day_of_month(self.from_sunday)
    }
}

impl Iterator for MonthFullWeeksIter {
    type Item = chrono::NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.current_date;
        self.current_date = old + Days::new(1);

        if old
            <= self
                .base_date
                .last_day_of_week_after_last_day_of_month(self.from_sunday)
        {
            Some(old)
        } else {
            None
        }
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
    #[test]
    fn test_iterator_end() {
        let i = NaiveDate::default();
        let mut it = i.month_full_weeks_iter(true);
        for _ in 1..=42 {
            it.next();
        }
        assert_eq!(it.next(), None);
    }
}
