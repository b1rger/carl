// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
use chrono::prelude::*;
use chrono::Duration;

pub trait DateExtensions {
    fn first_day_of_month(&self) -> chrono::Date<Local>;
    fn last_day_of_month(&self) -> chrono::Date<Local>;
    fn first_day_of_year(&self) -> chrono::Date<Local>;
    fn last_day_of_year(&self) -> chrono::Date<Local>;
    fn first_day_of_next_month(&self) -> chrono::Date<Local>;
    fn first_day_of_week_before_first_day_of_month(&self, from_sunday: bool)
        -> chrono::Date<Local>;
    fn last_day_of_week_after_last_day_of_month(&self, from_sunday: bool) -> chrono::Date<Local>;
    fn month_full_weeks_iter(&self, from_sunday: bool) -> MonthFullWeeksIter;
}

impl DateExtensions for chrono::Date<Local> {
    fn first_day_of_month(&self) -> chrono::Date<Local> {
        let mut prevdate: chrono::Date<Local> = *self;
        while let Some(x) = prevdate.pred_opt() {
            if x.month() == prevdate.month() {
                prevdate = prevdate.pred();
            } else {
                break;
            }
        }
        prevdate
    }

    fn last_day_of_month(&self) -> chrono::Date<Local> {
        let mut nextdate: chrono::Date<Local> = *self;
        while let Some(x) = nextdate.succ_opt() {
            if x.month() == nextdate.month() {
                nextdate = nextdate.succ();
            } else {
                break;
            }
        }
        nextdate
    }

    fn first_day_of_year(&self) -> chrono::Date<Local> {
        let mut prevdate: chrono::Date<Local> = *self;
        while let Some(x) = prevdate.pred_opt() {
            if x.year() == prevdate.year() {
                prevdate = prevdate.pred();
            } else {
                break;
            }
        }
        prevdate
    }

    fn last_day_of_year(&self) -> chrono::Date<Local> {
        let mut nextdate: chrono::Date<Local> = *self;
        while let Some(x) = nextdate.succ_opt() {
            if x.year() == nextdate.year() {
                nextdate = nextdate.succ();
            } else {
                break;
            }
        }
        nextdate
    }

    fn first_day_of_next_month(&self) -> chrono::Date<Local> {
        self.last_day_of_month().succ()
    }

    fn first_day_of_week_before_first_day_of_month(
        &self,
        from_sunday: bool,
    ) -> chrono::Date<Local> {
        let days_to_weekstart = if from_sunday {
            self.first_day_of_month().weekday().num_days_from_sunday()
        } else {
            self.first_day_of_month().weekday().num_days_from_monday()
        };
        self.first_day_of_month() - Duration::days(days_to_weekstart.into())
    }

    fn last_day_of_week_after_last_day_of_month(&self, from_sunday: bool) -> chrono::Date<Local> {
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
    pub base_date: chrono::Date<Local>,
    current_date: chrono::Date<Local>,
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
    type Item = chrono::Date<Local>;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.current_date;
        self.current_date = old.succ();

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

    #[test]
    fn test_first_day_of_month() {
        let i = Local.ymd(2021, 1, 1);
        let u = Local.ymd(2021, 1, 15);
        assert_eq!(i, u.first_day_of_month());
    }
    #[test]
    fn test_last_day_of_month() {
        let i = Local.ymd(2021, 1, 31);
        let u = Local.ymd(2021, 1, 15);
        assert_eq!(i, u.last_day_of_month());
    }
    #[test]
    fn test_first_day_of_year() {
        let i = Local.ymd(2021, 1, 1);
        let u = Local.ymd(2021, 2, 15);
        assert_eq!(i, u.first_day_of_year());
    }
    #[test]
    fn test_last_day_of_year() {
        let i = Local.ymd(2021, 12, 31);
        let u = Local.ymd(2021, 1, 15);
        assert_eq!(i, u.last_day_of_year());
    }
    #[test]
    fn test_first_day_of_next_month() {
        let i = Local.ymd(2021, 3, 1);
        let u = Local.ymd(2021, 2, 1);
        assert_eq!(i, u.first_day_of_next_month());
    }
    #[test]
    fn test_first_day_of_week_before_first_day_of_month() {
        let i = Local.ymd(2021, 11, 29);
        let u = Local.ymd(2021, 12, 15);
        assert_eq!(i, u.first_day_of_week_before_first_day_of_month(false));
    }
    #[test]
    fn test_last_day_of_week_after_last_day_of_month() {
        let i = Local.ymd(2022, 01, 2);
        let u = Local.ymd(2021, 12, 15);
        assert_eq!(i, u.last_day_of_week_after_last_day_of_month(false));
    }
    #[test]
    fn test_first_day_of_week_before_first_day_of_month_sunday() {
        let i = Local.ymd(2021, 11, 28);
        let u = Local.ymd(2021, 12, 15);
        assert_eq!(i, u.first_day_of_week_before_first_day_of_month(true));
    }
    #[test]
    fn test_last_day_of_week_after_last_day_of_month_sunday() {
        let i = Local.ymd(2022, 01, 1);
        let u = Local.ymd(2021, 12, 15);
        assert_eq!(i, u.last_day_of_week_after_last_day_of_month(true));
    }
    #[test]
    fn test_iterator_end() {
        let i = Local.ymd(2022, 01, 1);
        let mut it = i.month_full_weeks_iter(true);
        for _ in 1..=42 {
            it.next();
        }
        assert_eq!(it.next(), None);
    }
}
