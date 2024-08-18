// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod cli;
mod config;
mod context;
mod events;
mod filters;
mod output;
mod utils;

extern crate clap;
extern crate serde;
extern crate toml;
extern crate xdg;
use chrono::Datelike;
use chrono::Duration;
use std::process;

use crate::filters::months_into_columns;
use context::Context;
use events::{Events, ReadFromIcsFile};
use minijinja::{render, Environment};
use output::agenda::Agenda;
use output::yearprogress::Yearprogress;
use utils::DateExtensions;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut ctx: Context;
    match Context::new() {
        Ok(x) => ctx = x,
        Err(x) => {
            eprintln!("{}", x);
            process::exit(1);
        }
    }

    let mut env = Environment::new();
    env.add_filter("months_into_columns", months_into_columns);
    minijinja_contrib::add_to_environment(&mut env);

    ctx.daterangebegin = ctx
        .usersetdate
        .first_day_of_month()
        .first_day_of_week_before_first_day_of_month(ctx.opts.sunday);
    ctx.daterangeend = ctx
        .usersetdate
        .last_day_of_month()
        .last_day_of_week_after_last_day_of_month(ctx.opts.sunday);
    let mut daterange: Vec<String> = vec![ctx.usersetdate.format("%Y-%m").to_string()];
    let mut columns = 1;

    if ctx.opts.three {
        ctx.daterangebegin = (ctx.usersetdate - Duration::weeks(4)).first_day_of_month();
        ctx.daterangeend = (ctx.usersetdate + Duration::weeks(4)).last_day_of_month();
        daterange.push(
            (ctx.usersetdate.first_day_of_month() - Duration::days(1))
                .format("%Y-%m")
                .to_string(),
        );
        daterange.push(
            (ctx.usersetdate.last_day_of_month() + Duration::days(1))
                .format("%Y-%m")
                .to_string(),
        );
        columns = 3;
    }
    if ctx.opts.year {
        ctx.daterangebegin = ctx.usersetdate.first_day_of_year();
        ctx.daterangeend = ctx.usersetdate.last_day_of_year();
        for month in 1..=12 {
            if month != ctx.usersetdate.month() {
                daterange.push(format!("{}-{}", ctx.usersetdate.year(), month));
            }
        }
        columns = 3;
    }
    if ctx.opts.action.calendar {
        let template = include_str!("templates/default.tmpl").to_string();
        print!(
            "{}",
            render!(in env, &template, months=>daterange, columns=>columns)
        );
    }

    for icalstyle in &ctx.config.ical {
        for event in Events::read_from_ics_file(&icalstyle.file) {
            ctx.eventinstances.append(&mut event.instances(
                &ctx.daterangebegin,
                &ctx.daterangeend,
                &icalstyle.style,
            ));
        }
    }
    ctx.eventinstances.sort_by(|a, b| a.date.cmp(&b.date));

    /*if ctx.opts.action.calendar {
        let calendar = Calendar { ctx: &ctx };
        print!("{}", calendar);
    }*/

    if ctx.opts.action.agenda {
        let agenda = Agenda { ctx: &ctx };
        print!("{}", agenda);
    }

    if ctx.opts.action.yearprogress {
        let yp = Yearprogress { ctx: &ctx };
        print!("{}", yp);
    }
}
