// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use chrono::prelude::*;
use clap::{crate_authors, crate_name, crate_version, Args, Parser};

#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!(","), about = "Display a calendar")]
pub struct Cli {
    #[clap(short = '1', long = "one",
           help = "show only current month (default)",
           conflicts_with_all = &["three", "year", "months"])]
    pub one: bool,

    #[clap(short = '3',
           long = "three",
           help = "show previous, current and next month",
           conflicts_with_all = &["one", "year", "months"]
    )]
    pub three: bool,

    #[clap(short = 'n', long = "months",
           value_name="NUMBER", value_parser = clap::value_parser!(u8).range(1..),
           help = "show current and following months, in total NUMBER months",
           conflicts_with_all = &["one", "three", "year"])]
    pub months: Option<u8>,

    #[clap(short = 'y', long = "year",
           help = "show whole current year",
           conflicts_with_all = &["one", "three", "months"])]
    pub year: bool,

    #[clap(short = 's', long = "sunday", help = "Sunday as first day of week")]
    pub sunday: bool,
    #[clap(short = 'm', long = "monday", help = "Monday as first day of week")]
    pub monday: bool,

    #[clap(short = 'j', long = "julian", help = "output Julian dates")]
    pub julian: bool,

    #[clap(long = "themestyletype",
           help = "select dark or light theme styles",
           value_parser=["dark", "light"])]
    pub themestyletype: Option<String>,
    #[clap(long = "theme", help = "select theme by name", num_args(1))]
    pub theme: Option<String>,

    #[clap(num_args(0..=3))]
    pub date: Vec<String>,

    #[command(flatten)]
    pub action: Action,
}

#[derive(Args, Clone, Default, PartialEq)]
#[group(required = false, multiple = true)]
pub struct Action {
    #[clap(short = 'c', long = "calendar", help = "show calendar (default)")]
    pub calendar: bool,
    #[clap(short = 'a', long = "agenda", help = "show agenda")]
    pub agenda: bool,
    #[clap(long = "year-progress", help = "show year progress")]
    pub yearprogress: bool,
}

impl Cli {
    pub fn validate_date(&self) -> Result<chrono::NaiveDate, String> {
        let mut today: chrono::NaiveDate = Local::now().date_naive();
        let mut year: i32 = today.year();
        let mut month: u32 = today.month();
        let mut day: u32 = today.day();

        if !self.date.is_empty() {
            year = match self.date[0].parse() {
                Ok(x) => {
                    if x > 9999 {
                        return Err(format!(
                            "{}: illegal year value: use 1-9999: {}",
                            crate_name!(),
                            x
                        ));
                    }
                    x
                }
                Err(x) => {
                    return Err(format!(
                        "{}: illegal year value: use 1-9999: {}",
                        crate_name!(),
                        x
                    ));
                }
            }
        }
        if self.date.len() >= 2 {
            month = match self.date[1].parse() {
                Ok(x) => {
                    if x > 12 {
                        return Err(format!(
                            "{}: illegal month value: use 1-12: {}",
                            crate_name!(),
                            x
                        ));
                    }
                    x
                }
                Err(x) => {
                    return Err(format!(
                        "{}: illegal month value: use 1-12: {}",
                        crate_name!(),
                        x
                    ));
                }
            }
        }
        if self.date.len() == 3 {
            // TODO: this should depend on the days a month has
            day = match self.date[2].parse() {
                Ok(x) => {
                    if x > 31 {
                        return Err(format!(
                            "{}: illegal day value: use 1-12: {}",
                            crate_name!(),
                            x
                        ));
                    }
                    x
                }
                Err(x) => {
                    return Err(format!(
                        "{}: illegal day value: use 1-12: {}",
                        crate_name!(),
                        x
                    ));
                }
            }
        }
        if self.date.len() > 3 {
            eprintln!("Could not parse date value(s) - using today.");
            return Ok(today);
        }
        //if let Some(x) = Local.ymd_opt(year, month, day).single() {
        if let Some(x) = NaiveDate::from_ymd_opt(year, month, day) {
            today = x;
        } else {
            eprintln!("Could not parse date value(s) - using today.");
        }
        Ok(today)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_date_defaults_to_now() {
        let today: chrono::NaiveDate = Local::now().date_naive();
        let o: Cli = Cli::parse();
        assert_eq!(today, o.validate_date().unwrap());
    }
    #[test]
    fn test_validate_date_default_to_now_with_custom_year() {
        let today: chrono::NaiveDate = Local::now().date_naive().with_year(2007).unwrap();
        let mut o: Cli = Cli::parse();
        o.date = vec![String::from("2007")];
        assert_eq!(today, o.validate_date().unwrap());
    }
    #[test]
    fn test_validate_date_defaults_to_now_with_custom_year_and_month() {
        let today: chrono::NaiveDate = Local::now()
            .date_naive()
            .with_year(2007)
            .unwrap()
            .with_month(1)
            .unwrap();
        let mut o: Cli = Cli::parse();
        o.date = vec![String::from("2007"), String::from("1")];
        assert_eq!(today, o.validate_date().unwrap());
    }
    #[test]
    fn test_validate_date_defaults_to_now_with_custom_year_and_month_and_day() {
        let today: chrono::NaiveDate = Local::now()
            .date_naive()
            .with_year(2007)
            .unwrap()
            .with_month(1)
            .unwrap()
            .with_day(28)
            .unwrap();
        let mut o: Cli = Cli::parse();
        o.date = vec![String::from("2007"), String::from("1"), String::from("28")];
        assert_eq!(today, o.validate_date().unwrap());
    }
    #[test]
    fn test_validate_date_defaults_to_now_with_ambiguous_arguments() {
        let today: chrono::NaiveDate = Local::now().date_naive();
        let mut o: Cli = Cli::parse();
        o.date = vec![
            String::from("2007"),
            String::from("1"),
            String::from("28"),
            String::from("28"),
        ];
        assert_eq!(today, o.validate_date().unwrap());
    }
    #[test]
    fn test_validate_date_errors_with_wrong_month() {
        let mut o: Cli = Cli::parse();
        o.date = vec![String::from("2007"), String::from("13"), String::from("28")];
        assert!(o.validate_date().is_err());
    }
    #[test]
    fn test_validate_date_errors_with_wrong_day() {
        let mut o: Cli = Cli::parse();
        o.date = vec![String::from("2007"), String::from("11"), String::from("33")];
        assert!(o.validate_date().is_err());
    }
    #[test]
    fn test_validate_date_errors_with_wrong_year() {
        let mut o: Cli = Cli::parse();
        o.date = vec![
            String::from("999999"),
            String::from("11"),
            String::from("28"),
        ];
        assert!(o.validate_date().is_err());
    }
    #[test]
    fn test_validate_date_errors_with_unparsable_year() {
        let mut o: Cli = Cli::parse();
        o.date = vec![String::from("foo"), String::from("13"), String::from("28")];
        assert!(o.validate_date().is_err());
    }
    #[test]
    fn test_validate_date_errors_with_unparsable_month() {
        let mut o: Cli = Cli::parse();
        o.date = vec![
            String::from("2007"),
            String::from("foo"),
            String::from("23"),
        ];
        assert!(o.validate_date().is_err());
    }
    #[test]
    fn test_validate_date_errors_with_unparsable_day() {
        let mut o: Cli = Cli::parse();
        o.date = vec![
            String::from("2007"),
            String::from("11"),
            String::from("foo"),
        ];
        assert!(o.validate_date().is_err());
    }
    #[test]
    fn test_validate_date_errors_with_non_existent_date() {
        let today: chrono::NaiveDate = Local::now().date_naive();
        let mut o: Cli = Cli::parse();
        o.date = vec![String::from("2007"), String::from("2"), String::from("30")];
        assert_eq!(today, o.validate_date().unwrap());
    }
}
