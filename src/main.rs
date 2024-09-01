// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod cli;
mod config;
mod context;
mod events;
mod output;
mod utils;

extern crate clap;
extern crate serde;
extern crate toml;
extern crate xdg;
use std::process;

use context::Context;
use events::{Events, ReadFromIcsFile};
use output::agenda::Agenda;
use output::calendar::Calendar;
use output::date::Date;
use output::yearprogress::Yearprogress;
use utils::DateExtensions;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut columns = 1;
    let mut months: Vec<chrono::NaiveDate> = vec![];

    let mut ctx: Context;
    match Context::new() {
        Ok(x) => ctx = x,
        Err(x) => {
            eprintln!("{}", x);
            process::exit(1);
        }
    }

    let mut daterangebegin: chrono::NaiveDate = ctx.usersetdate.first_day_of_month();
    let mut daterangeend: chrono::NaiveDate = ctx.usersetdate.last_day_of_month();

    if ctx.opts.three {
        daterangebegin = ctx
            .usersetdate
            .first_day_of_month()
            .pred_opt()
            .unwrap()
            .first_day_of_month();
        daterangeend = ctx
            .usersetdate
            .first_day_of_next_month()
            .last_day_of_month();
        months.push(daterangebegin);
        months.push(ctx.usersetdate);
        months.push(daterangeend);
        columns = 3;
    }
    if let Some(num) = ctx.opts.months {
        daterangebegin = ctx.usersetdate.first_day_of_month();
        let mut tmpdate = daterangebegin;
        months.push(tmpdate);
        for _ in 1..=(num - 1) {
            tmpdate = tmpdate.first_day_of_next_month();
            months.push(tmpdate);
        }
        daterangeend = tmpdate.last_day_of_month();
        columns = 3;
    }
    if ctx.opts.year {
        daterangebegin = ctx.usersetdate.first_day_of_year();
        daterangeend = ctx.usersetdate.last_day_of_year();
        let mut tmpdate = daterangebegin;
        while tmpdate < daterangeend {
            months.push(tmpdate);
            tmpdate = tmpdate.first_day_of_next_month();
        }
        columns = 3;
    }
    if !ctx.opts.three && !ctx.opts.year && ctx.opts.months.is_none() {
        months.push(ctx.usersetdate);
    }

    for icalstyle in &ctx.config.ical {
        for event in Events::read_from_ics_file(&icalstyle.file) {
            ctx.eventinstances.append(&mut event.instances(
                &daterangebegin,
                &daterangeend,
                &icalstyle.style,
            ));
        }
    }
    ctx.eventinstances.sort_by(|a, b| a.date.cmp(&b.date));

    if ctx.opts.action.calendar {
        let calendar = Calendar {
            dates: months,
            columns,
            ctx: &ctx,
        };
        print!("{}", calendar);
    }

    if ctx.opts.action.agenda {
        let agenda = Agenda { ctx: &ctx };
        print!("{}", agenda);
    }

    if ctx.opts.action.yearprogress {
        let yp = Yearprogress { ctx: &ctx };
        print!("{}", yp);
    }
}
