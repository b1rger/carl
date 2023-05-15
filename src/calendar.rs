// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
use crate::lib::types::ChronoDate;
use crate::lib::{DateExtensions, MonthFullWeeksIter};
use crate::Context;
use crate::Date;
use chrono::{Duration, NaiveDate};

use std::fmt;

pub struct Calendar<'a> {
    pub dates: Vec<ChronoDate>,
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
            for _ in 0..self.columns {
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
        let mut week: Vec<ChronoDate> = vec![];
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
        let res = String::from("    January 1970      \nMo Tu We Th Fr Sa Su  \n\u{1b}[2;8m29\u{1b}[0m \u{1b}[2;8m30\u{1b}[0m \u{1b}[2;8m31\u{1b}[0m \u{1b}[1;4m 1\u{1b}[0m  2  3  4  \n 5  6  7  8  9 10 11  \n12 13 14 15 16 17 18  \n19 20 21 22 23 24 25  \n26 27 28 29 30 31 \u{1b}[8m 1\u{1b}[0m  \n\n");
        assert_eq!(res, format!("{}", cal));
    }

    #[test]
    fn test_calendar_year_fmt() {
        let mut ctx = Context::default();
        ctx.usersetdate = NaiveDate::from_ymd_opt(2021, 12, 10).unwrap();
        ctx.opts.year = true;

        let mut dates: Vec<ChronoDate> = vec![];
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
[2;8m28[0m [2;8m29[0m [2;8m30[0m [2;8m31[0m [2m 1[0m [2m 2[0m [2m 3[0m  [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m [2m 5[0m [2m 6[0m [2m 7[0m  [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m [2m 5[0m [2m 6[0m [2m 7[0m  
[2m 4[0m [2m 5[0m [2m 6[0m [2m 7[0m [2m 8[0m [2m 9[0m [2m10[0m  [2m 8[0m [2m 9[0m [2m10[0m [2m11[0m [2m12[0m [2m13[0m [2m14[0m  [2m 8[0m [2m 9[0m [2m10[0m [2m11[0m [2m12[0m [2m13[0m [2m14[0m  
[2m11[0m [2m12[0m [2m13[0m [2m14[0m [2m15[0m [2m16[0m [2m17[0m  [2m15[0m [2m16[0m [2m17[0m [2m18[0m [2m19[0m [2m20[0m [2m21[0m  [2m15[0m [2m16[0m [2m17[0m [2m18[0m [2m19[0m [2m20[0m [2m21[0m  
[2m18[0m [2m19[0m [2m20[0m [2m21[0m [2m22[0m [2m23[0m [2m24[0m  [2m22[0m [2m23[0m [2m24[0m [2m25[0m [2m26[0m [2m27[0m [2m28[0m  [2m22[0m [2m23[0m [2m24[0m [2m25[0m [2m26[0m [2m27[0m [2m28[0m  
[2m25[0m [2m26[0m [2m27[0m [2m28[0m [2m29[0m [2m30[0m [2m31[0m                        [2m29[0m [2m30[0m [2m31[0m [2;8m 1[0m [2;8m 2[0m [2;8m 3[0m [2;8m 4[0m  

        April                  May                  June          
Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  
[2;8m29[0m [2;8m30[0m [2;8m31[0m [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m  [2;8m26[0m [2;8m27[0m [2;8m28[0m [2;8m29[0m [2;8m30[0m [2m 1[0m [2m 2[0m  [2;8m31[0m [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m [2m 5[0m [2m 6[0m  
[2m 5[0m [2m 6[0m [2m 7[0m [2m 8[0m [2m 9[0m [2m10[0m [2m11[0m  [2m 3[0m [2m 4[0m [2m 5[0m [2m 6[0m [2m 7[0m [2m 8[0m [2m 9[0m  [2m 7[0m [2m 8[0m [2m 9[0m [2m10[0m [2m11[0m [2m12[0m [2m13[0m  
[2m12[0m [2m13[0m [2m14[0m [2m15[0m [2m16[0m [2m17[0m [2m18[0m  [2m10[0m [2m11[0m [2m12[0m [2m13[0m [2m14[0m [2m15[0m [2m16[0m  [2m14[0m [2m15[0m [2m16[0m [2m17[0m [2m18[0m [2m19[0m [2m20[0m  
[2m19[0m [2m20[0m [2m21[0m [2m22[0m [2m23[0m [2m24[0m [2m25[0m  [2m17[0m [2m18[0m [2m19[0m [2m20[0m [2m21[0m [2m22[0m [2m23[0m  [2m21[0m [2m22[0m [2m23[0m [2m24[0m [2m25[0m [2m26[0m [2m27[0m  
[2m26[0m [2m27[0m [2m28[0m [2m29[0m [2m30[0m [2;8m 1[0m [2;8m 2[0m  [2m24[0m [2m25[0m [2m26[0m [2m27[0m [2m28[0m [2m29[0m [2m30[0m  [2m28[0m [2m29[0m [2m30[0m [2;8m 1[0m [2;8m 2[0m [2;8m 3[0m [2;8m 4[0m  
                      [2m31[0m [2;8m 1[0m [2;8m 2[0m [2;8m 3[0m [2;8m 4[0m [2;8m 5[0m [2;8m 6[0m                        

        July                 August               September       
Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  
[2;8m28[0m [2;8m29[0m [2;8m30[0m [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m  [2;8m26[0m [2;8m27[0m [2;8m28[0m [2;8m29[0m [2;8m30[0m [2;8m31[0m [2m 1[0m  [2;8m30[0m [2;8m31[0m [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m [2m 5[0m  
[2m 5[0m [2m 6[0m [2m 7[0m [2m 8[0m [2m 9[0m [2m10[0m [2m11[0m  [2m 2[0m [2m 3[0m [2m 4[0m [2m 5[0m [2m 6[0m [2m 7[0m [2m 8[0m  [2m 6[0m [2m 7[0m [2m 8[0m [2m 9[0m [2m10[0m [2m11[0m [2m12[0m  
[2m12[0m [2m13[0m [2m14[0m [2m15[0m [2m16[0m [2m17[0m [2m18[0m  [2m 9[0m [2m10[0m [2m11[0m [2m12[0m [2m13[0m [2m14[0m [2m15[0m  [2m13[0m [2m14[0m [2m15[0m [2m16[0m [2m17[0m [2m18[0m [2m19[0m  
[2m19[0m [2m20[0m [2m21[0m [2m22[0m [2m23[0m [2m24[0m [2m25[0m  [2m16[0m [2m17[0m [2m18[0m [2m19[0m [2m20[0m [2m21[0m [2m22[0m  [2m20[0m [2m21[0m [2m22[0m [2m23[0m [2m24[0m [2m25[0m [2m26[0m  
[2m26[0m [2m27[0m [2m28[0m [2m29[0m [2m30[0m [2m31[0m [2;8m 1[0m  [2m23[0m [2m24[0m [2m25[0m [2m26[0m [2m27[0m [2m28[0m [2m29[0m  [2m27[0m [2m28[0m [2m29[0m [2m30[0m [2;8m 1[0m [2;8m 2[0m [2;8m 3[0m  
                      [2m30[0m [2m31[0m [2;8m 1[0m [2;8m 2[0m [2;8m 3[0m [2;8m 4[0m [2;8m 5[0m                        

       October              November              December        
Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  Mo Tu We Th Fr Sa Su  
[2;8m27[0m [2;8m28[0m [2;8m29[0m [2;8m30[0m [2m 1[0m [2m 2[0m [2m 3[0m  [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m [2m 5[0m [2m 6[0m [2m 7[0m  [2;8m29[0m [2;8m30[0m [2m 1[0m [2m 2[0m [2m 3[0m [2m 4[0m [2m 5[0m  
[2m 4[0m [2m 5[0m [2m 6[0m [2m 7[0m [2m 8[0m [2m 9[0m [2m10[0m  [2m 8[0m [2m 9[0m [2m10[0m [2m11[0m [2m12[0m [2m13[0m [2m14[0m  [2m 6[0m [2m 7[0m [2m 8[0m [2m 9[0m [1;4m10[0m 11 12  
[2m11[0m [2m12[0m [2m13[0m [2m14[0m [2m15[0m [2m16[0m [2m17[0m  [2m15[0m [2m16[0m [2m17[0m [2m18[0m [2m19[0m [2m20[0m [2m21[0m  13 14 15 16 17 18 19  
[2m18[0m [2m19[0m [2m20[0m [2m21[0m [2m22[0m [2m23[0m [2m24[0m  [2m22[0m [2m23[0m [2m24[0m [2m25[0m [2m26[0m [2m27[0m [2m28[0m  20 21 22 23 24 25 26  
[2m25[0m [2m26[0m [2m27[0m [2m28[0m [2m29[0m [2m30[0m [2m31[0m  [2m29[0m [2m30[0m [2;8m 1[0m [2;8m 2[0m [2;8m 3[0m [2;8m 4[0m [2;8m 5[0m  27 28 29 30 31 [8m 1[0m [8m 2[0m  

");
        assert_eq!(res, format!("{}", cal));
    }
}
