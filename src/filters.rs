use crate::utils::DateExtensions;
use chrono::Duration;
use std::collections::HashSet;
pub type Week = Vec<String>;
pub type Line = Vec<Week>;
pub type MonthLines = Vec<Line>;

pub type Month = Vec<String>;

pub fn months_into_columns(months: Vec<String>, columns: usize) -> Vec<MonthLines> {
    let mut months_set = HashSet::new();
    for month in months {
        let tmpmonth = format!("{}-01", month);
        if let Ok(date) = chrono::NaiveDate::parse_from_str(&tmpmonth, "%Y-%m-%d") {
            months_set.insert(date.first_day_of_month());
        }
    }
    // column -> month -> week -> date
    let mut dates: Vec<Month> = vec![];
    for month in months_set {
        let mut monthv: Month = vec![];
        let mut date = month
            .first_day_of_month()
            .first_day_of_week_before_first_day_of_month(true);
        while date
            <= month
                .last_day_of_month()
                .last_day_of_week_after_last_day_of_month(true)
        {
            monthv.push(date.format("%Y-%m-%d").to_string());
            date += Duration::days(1);
        }
        dates.push(monthv);
    }
    dates.sort();
    let mut months: Vec<Vec<Month>> = vec![];
    for chunk in dates.chunks(columns) {
        months.push(chunk.to_vec());
    }
    let mut ret: Vec<MonthLines> = vec![];
    for mut columns_month in months {
        let mut monthlines: MonthLines = vec![];
        while columns_month.iter().all(|x| !x.is_empty()) {
            let mut line: Line = vec![];
            for month in &mut columns_month {
                let foo: Vec<String> = month.drain(..7).collect();
                line.push(foo);
            }
            monthlines.push(line);
        }
        ret.push(monthlines);
    }
    ret
}
