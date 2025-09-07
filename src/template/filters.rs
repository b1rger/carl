// SPDX-FileCopyrightText: 2025 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT
use chrono::Datelike;
use chrono::NaiveDate;
use minijinja::{Error, ErrorKind};

pub(crate) fn days_per_year(date: chrono::NaiveDate) -> u32 {
    if date.leap_year() {
        366
    } else {
        365
    }
}

pub(crate) fn days_in_year_left(value: &str) -> Result<u32, Error> {
    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        let days_in_year = days_per_year(date);
        return Ok(days_in_year - date.ordinal());
    };
    Err(Error::new(ErrorKind::InvalidOperation, "not a date."))
}

pub(crate) fn percentage_of_year(value: &str) -> Result<f32, Error> {
    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        let days_in_year = days_per_year(date);
        let percentage: f32 = (date.ordinal() * 100) as f32 / days_in_year as f32;
        return Ok(percentage);
    }
    Err(Error::new(ErrorKind::InvalidOperation, "not a date."))
}
