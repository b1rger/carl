// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::Context;

use anstyle::Style;
use chrono::Datelike;
use std::fmt;

pub struct Yearprogress<'a> {
    pub ctx: &'a Context,
}

impl fmt::Display for Yearprogress<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style = Style::default().bold();
        let year = self.ctx.usersetdate.year();
        let mut ret: String = format!(
            "{}Yearprogress ({year}):{}\n",
            style.render(),
            style.render_reset()
        );
        let day = self.ctx.usersetdate.ordinal();
        let days_in_year: u32 = if self.ctx.usersetdate.leap_year() {
            366
        } else {
            365
        };
        let days_in_year_left: u32 = days_in_year - day;
        let percentage: f32 = (day * 100) as f32 / days_in_year as f32;
        ret += format!("{percentage:.3}% of {year}\n").as_str();
        ret += format!("Day number {day}, {days_in_year_left} left\n").as_str();
        write!(f, "{}", ret)
    }
}
