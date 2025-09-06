// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod cli;
mod config;
mod context;
mod events;
mod utils;
mod template;

extern crate clap;
extern crate serde;
extern crate toml;
extern crate xdg;
use std::process;

use crate::utils::jinja_functions::reset_style;
use crate::utils::helpers::generate_dates_from_to;
use crate::template::{DateStyler,DateToColumnsGenerator};
use crate::events::{Events, ReadFromIcsFile};
use context::Context;
use minijinja::{path_loader, Environment, context};
use minijinja::syntax::SyntaxConfig;

#[cfg(not(tarpaulin_include))]
fn main() {
    let ctx: Context;
    match Context::new() {
        Ok(x) => ctx = x,
        Err(x) => {
            eprintln!("{}", x);
            process::exit(1);
        }
    }

    let mut event_instances = vec![];
    for icalstyle in &ctx.config.ical {
        for event in Events::read_from_ics_file(&icalstyle.file) {
            event_instances.append(&mut event.instances(&ctx.begin, &ctx.end, &icalstyle.style));
        }
    }
    event_instances.sort_by(|a, b| a.date.cmp(&b.date));
    let dates_per_month = generate_dates_from_to(ctx.begin, ctx.end, ctx.opts.sunday);


    let mut env = Environment::new();
    env.set_syntax(SyntaxConfig::builder()
        .line_statement_prefix("#")
        .line_comment_prefix("##")
        .build()
        .unwrap()
    );
    env.set_loader(path_loader("templates"));
    minijinja_contrib::add_to_environment(&mut env);

    let date_styler = DateStyler::new(event_instances.clone(), ctx.usersetdate.clone(), ctx.theme.clone());
    let template_context = context! { 
        cli => ctx.opts,
        columns => ctx.columns,
        dates_per_month => dates_per_month,
        dates_to_columns => minijinja::Value::from_object(DateToColumnsGenerator::new()),
        event_instances => event_instances,
        style_date => minijinja::Value::from_object(date_styler),
        reset_style => minijinja::Value::from_function(reset_style),
    };

    let tmpl = env.get_template("carl.tmpl").unwrap();

    print!("{}", tmpl.render(template_context).unwrap());
}
