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
use std::path::Path;

use crate::utils::helpers::generate_dates_from_to;
use crate::template::{objects, functions, filters};
use crate::events::{Events, ReadFromIcsFile};
use context::Context;
use minijinja::{path_loader, Environment, context};
use minijinja::syntax::SyntaxConfig;

#[cfg(not(tarpaulin_include))]
fn main() {
    let ctx: Context = match Context::new() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{}", x);
            process::exit(1);
        }
    };

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
    let mut default_template: String = "carl.tmpl".to_string();

    if let Some(template_file) = ctx.config.template {
        let path = Path::new(&template_file);
        if path.is_file() {
            if let Some(parent) = path.parent() {
                if let Some(filename) = path.file_name() {
                    if let Some(filename) = filename.to_str() {
                        env.set_loader(path_loader(parent));
                        default_template = filename.to_string();
                    }
                }
            }
        }
    } else {
        minijinja_embed::load_templates!(&mut env);
    }

    env.add_filter("days_in_year_left", filters::days_in_year_left);
    env.add_filter("percentage_of_year", filters::percentage_of_year);
    env.add_function("dates_to_columns", functions::dates_to_columns);
    env.add_function("reset_style", functions::reset_style);
    minijinja_contrib::add_to_environment(&mut env);

    let date_styler = objects::DateStyler::new(event_instances.clone(), ctx.usersetdate, ctx.theme.clone());
    let template_context = context! { 
        cli => ctx.opts,
        columns => ctx.columns,
        dates_per_month => dates_per_month,
        event_instances => event_instances,
        main_date => ctx.usersetdate,
        style_date => minijinja::Value::from_object(date_styler),
    };

    let tmpl = env.get_template(default_template.as_str()).unwrap();

    print!("{}", tmpl.render(template_context).unwrap());
}
