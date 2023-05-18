// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod agenda;
mod calendar;
mod config;
mod context;
mod date;
mod events;
mod lib;
mod opts;

extern crate clap;
extern crate serde;
extern crate toml;
extern crate xdg;
use chrono::Duration;
use std::process;

use agenda::Agenda;
use calendar::Calendar;
use context::Context;
use date::Date;
use events::Events;
use events::ReadFromIcsFile;
use lib::types::ChronoDate;
use lib::DateExtensions;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut columns = 1;
    let mut months: Vec<ChronoDate> = vec![];

    let mut ctx: Context;
    match Context::new() {
        Ok(x) => ctx = x,
        Err(x) => {
            eprintln!("{}", x);
            process::exit(1);
        }
    }

    let mut daterangebegin: ChronoDate = ctx.usersetdate.first_day_of_month();
    let mut daterangeend: ChronoDate = ctx.usersetdate.last_day_of_month();

    if ctx.opts.three {
        daterangebegin = (ctx.usersetdate - Duration::weeks(4)).first_day_of_month();
        daterangeend = (ctx.usersetdate + Duration::weeks(4)).last_day_of_month();
        months.push(daterangebegin);
        months.push(ctx.usersetdate);
        months.push(daterangeend);
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
    if !ctx.opts.three && !ctx.opts.year {
        months.push(ctx.usersetdate);
    }

    // mabye we should use a pointer instead of cloning the style?
    for icalstyle in &ctx.config.ical {
        let mut icsevents = Events::read_from_ics_file(&icalstyle.file);
        icsevents = icsevents
            .into_iter()
            .filter(|event| event.in_range(daterangebegin, daterangeend))
            .collect();
        for event in icsevents {
            ctx.eventstuple.push((event, icalstyle.style.clone()));
        }
    }

    let calendar = Calendar {
        dates: months,
        columns,
        ctx: &ctx,
    };
    print!("{}", calendar);

    let agenda = Agenda { ctx: &ctx };
    print!("{}", agenda)
}
