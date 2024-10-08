// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::utils::{DateExtensions, MonthFullWeeksIter};
use crate::Context;
use crate::Date;
use chrono::{Duration, NaiveDate};
use std::cmp::min;

use std::fmt;

pub struct Calendar<'a> {
    pub dates: Vec<chrono::NaiveDate>,
    pub columns: usize,
    pub ctx: &'a Context,
}

impl fmt::Display for Calendar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret: String = String::new();

        let width = 7 * self.julian_or_gregorian_width() + 7;

        // If we display the whole year, lets display a year header
        if self.ctx.opts.year {
            ret = format!(
                "{:^width$}\n",
                self.ctx.usersetdate.format("%Y"),
                width = width * self.columns
            );
        }

        let monthformat = if self.ctx.opts.year { "%B" } else { "%B %Y" };
        // Iterate through the list of dates and show a chart
        // for the month of every date
        // First split the list of dates into chunks for every
        // column we want to display
        let date_chunks = self.dates.chunks(self.columns);
        // Go through every date in the chunk...
        for chunk in date_chunks {
            // ... and display a header
            for date in chunk {
                ret = format!(
                    "{}{:^width$} ",
                    ret,
                    date.format(monthformat),
                    width = width
                );
            }
            ret = format!("{}\n", ret);

            // .. then display the weekdays
            let weekdays = self.weekdays();
            for _ in 0..min(chunk.len(), self.columns) {
                ret = format!("{}{:^width$} ", ret, weekdays, width = width);
            }
            ret = format!("{}\n", ret);

            // .. now display the days of the month of the date
            let mut mfwi: Vec<MonthFullWeeksIter> = chunk
                .iter()
                .map(|date| date.month_full_weeks_iter(self.ctx.opts.sunday))
                .collect();
            while mfwi
                .iter()
                .any(|month_full_week_iter| !month_full_week_iter.empty())
            {
                for month_full_week_iter in mfwi.iter_mut() {
                    let firstdayofdisplayedmonth =
                        month_full_week_iter.base_date.first_day_of_month();
                    if month_full_week_iter.empty() {
                        ret = format!("{}{:width$}", ret, " ", width = width);
                    } else {
                        let week = month_full_week_iter.take(7);
                        for day in week {
                            let date = Date {
                                date: day,
                                ctx: self.ctx,
                                firstdayofdisplayedmonth,
                            };
                            ret = format!("{}{} ", ret, date);
                        }
                    }
                    ret = format!("{} ", ret);
                }
                ret = format!("{}\n", ret);
            }
            ret = format!("{}\n", ret);
        }
        write!(f, "{}", ret)
    }
}

impl Calendar<'_> {
    fn weekdays(&self) -> String {
        let mut week: Vec<chrono::NaiveDate> = vec![];
        let (s, e) = if self.ctx.opts.sunday {
            (3, 9)
        } else {
            (4, 10)
        };
        for x in s..=e {
            let d = NaiveDate::default() + Duration::days(x);
            week.push(d);
        }
        let width = self.julian_or_gregorian_width();
        format!(
            "{:.width$} {:.width$} {:.width$} {:.width$} {:.width$} {:.width$} {:.width$}",
            week[0].format("%A"),
            week[1].format("%A"),
            week[2].format("%A"),
            week[3].format("%A"),
            week[4].format("%A"),
            week[5].format("%A"),
            week[6].format("%A"),
            width = width
        )
    }

    fn julian_or_gregorian_width(&self) -> usize {
        if self.ctx.opts.julian {
            return 3;
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_calendar_julian_or_gregorian_width_1() {
        let date = NaiveDate::default();
        let cal = Calendar {
            dates: vec![date],
            columns: 1,
            ctx: &Context::default(),
        };
        assert_eq!(cal.julian_or_gregorian_width(), 2);
    }
    #[test]
    fn test_calendar_julian_or_gregorian_width_2() {
        let date = NaiveDate::default();
        let mut ctx = Context::default();
        ctx.opts.julian = true;
        let cal = Calendar {
            dates: vec![date],
            columns: 1,
            ctx: &ctx,
        };
        assert_eq!(cal.julian_or_gregorian_width(), 3);
    }
    #[test]
    fn test_calendar_fmt() {
        let date = NaiveDate::default();
        let cal = Calendar {
            dates: vec![date],
            columns: 1,
            ctx: &Context::default(),
        };
        let res = String::from("    January 1970      \nMo Tu We Th Fr Sa Su  \n\u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m\u{1b}[8m31\u{1b}[0m \u{1b}[1m\u{1b}[4m 1\u{1b}[0m  2  3  4  \n 5  6  7  8  9 10 11  \n12 13 14 15 16 17 18  \n19 20 21 22 23 24 25  \n26 27 28 29 30 31 \u{1b}[8m 1\u{1b}[0m  \n\n");
        assert_eq!(res, format!("{}", cal));
    }

    #[test]
    fn test_calendar_year_fmt() {
        let mut ctx = Context {
            usersetdate: NaiveDate::from_ymd_opt(2021, 12, 10).unwrap(),
            ..Default::default()
        };
        ctx.opts.year = true;

        let mut dates: Vec<chrono::NaiveDate> = vec![];
        let daterangebegin = ctx.usersetdate.first_day_of_year();
        let daterangeend = ctx.usersetdate.last_day_of_year();
        let mut tmpdate = daterangebegin;
        while tmpdate < daterangeend {
            dates.push(tmpdate);
            tmpdate = tmpdate.first_day_of_next_month();
        }

        let cal = Calendar {
            dates,
            columns: 3,
            ctx: &ctx,
        };
        let res = String::from("                             2021                              
       January              February                March         
Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  
\u{1b}[2m\u{1b}[8m28\u{1b}[0m \u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m\u{1b}[8m31\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m  \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m  \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m  
\u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m  \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m  \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m  
\u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m  \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m  \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m  
\u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m  \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m  \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m  
\u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m31\u{1b}[0m                        \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m31\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m \u{1b}[2m\u{1b}[8m 2\u{1b}[0m \u{1b}[2m\u{1b}[8m 3\u{1b}[0m \u{1b}[2m\u{1b}[8m 4\u{1b}[0m  

        April                  May                  June          
Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  
\u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m\u{1b}[8m31\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m  \u{1b}[2m\u{1b}[8m26\u{1b}[0m \u{1b}[2m\u{1b}[8m27\u{1b}[0m \u{1b}[2m\u{1b}[8m28\u{1b}[0m \u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m  \u{1b}[2m\u{1b}[8m31\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m  
\u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m  \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m  \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m  
\u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m  \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m  \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m  
\u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m  \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m  \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m  
\u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m \u{1b}[2m\u{1b}[8m 2\u{1b}[0m  \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m  \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m \u{1b}[2m\u{1b}[8m 2\u{1b}[0m \u{1b}[2m\u{1b}[8m 3\u{1b}[0m \u{1b}[2m\u{1b}[8m 4\u{1b}[0m  \n                      \u{1b}[2m31\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m \u{1b}[2m\u{1b}[8m 2\u{1b}[0m \u{1b}[2m\u{1b}[8m 3\u{1b}[0m \u{1b}[2m\u{1b}[8m 4\u{1b}[0m \u{1b}[2m\u{1b}[8m 5\u{1b}[0m \u{1b}[2m\u{1b}[8m 6\u{1b}[0m                        

        July                 August               September       
Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  
\u{1b}[2m\u{1b}[8m28\u{1b}[0m \u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m  \u{1b}[2m\u{1b}[8m26\u{1b}[0m \u{1b}[2m\u{1b}[8m27\u{1b}[0m \u{1b}[2m\u{1b}[8m28\u{1b}[0m \u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m\u{1b}[8m31\u{1b}[0m \u{1b}[2m 1\u{1b}[0m  \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m\u{1b}[8m31\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m  
\u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m  \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m  \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m  
\u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m  \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m  \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m  
\u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m  \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m  \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m  
\u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m31\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m  \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m  \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m \u{1b}[2m\u{1b}[8m 2\u{1b}[0m \u{1b}[2m\u{1b}[8m 3\u{1b}[0m  \n                      \u{1b}[2m30\u{1b}[0m \u{1b}[2m31\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m \u{1b}[2m\u{1b}[8m 2\u{1b}[0m \u{1b}[2m\u{1b}[8m 3\u{1b}[0m \u{1b}[2m\u{1b}[8m 4\u{1b}[0m \u{1b}[2m\u{1b}[8m 5\u{1b}[0m                        

       October              November              December        
Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  
\u{1b}[2m\u{1b}[8m27\u{1b}[0m \u{1b}[2m\u{1b}[8m28\u{1b}[0m \u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m  \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m  \u{1b}[2m\u{1b}[8m29\u{1b}[0m \u{1b}[2m\u{1b}[8m30\u{1b}[0m \u{1b}[2m 1\u{1b}[0m \u{1b}[2m 2\u{1b}[0m \u{1b}[2m 3\u{1b}[0m \u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m  
\u{1b}[2m 4\u{1b}[0m \u{1b}[2m 5\u{1b}[0m \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m  \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[2m10\u{1b}[0m \u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m  \u{1b}[2m 6\u{1b}[0m \u{1b}[2m 7\u{1b}[0m \u{1b}[2m 8\u{1b}[0m \u{1b}[2m 9\u{1b}[0m \u{1b}[1m\u{1b}[4m10\u{1b}[0m 11 12  
\u{1b}[2m11\u{1b}[0m \u{1b}[2m12\u{1b}[0m \u{1b}[2m13\u{1b}[0m \u{1b}[2m14\u{1b}[0m \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m  \u{1b}[2m15\u{1b}[0m \u{1b}[2m16\u{1b}[0m \u{1b}[2m17\u{1b}[0m \u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m  13 14 15 16 17 18 19  
\u{1b}[2m18\u{1b}[0m \u{1b}[2m19\u{1b}[0m \u{1b}[2m20\u{1b}[0m \u{1b}[2m21\u{1b}[0m \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m  \u{1b}[2m22\u{1b}[0m \u{1b}[2m23\u{1b}[0m \u{1b}[2m24\u{1b}[0m \u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m  20 21 22 23 24 25 26  
\u{1b}[2m25\u{1b}[0m \u{1b}[2m26\u{1b}[0m \u{1b}[2m27\u{1b}[0m \u{1b}[2m28\u{1b}[0m \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m31\u{1b}[0m  \u{1b}[2m29\u{1b}[0m \u{1b}[2m30\u{1b}[0m \u{1b}[2m\u{1b}[8m 1\u{1b}[0m \u{1b}[2m\u{1b}[8m 2\u{1b}[0m \u{1b}[2m\u{1b}[8m 3\u{1b}[0m \u{1b}[2m\u{1b}[8m 4\u{1b}[0m \u{1b}[2m\u{1b}[8m 5\u{1b}[0m  27 28 29 30 31 \u{1b}[8m 1\u{1b}[0m \u{1b}[8m 2\u{1b}[0m  

");
        assert_eq!(res, format!("{}", cal));
    }
}
