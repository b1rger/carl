// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod cli;
mod config;
mod context;
mod events;
mod template;
mod utils;

extern crate clap;
extern crate serde;
extern crate toml;
use std::process;

use context::Context;
use events::{Events, ReadFromIcsFile};
use template::{objects, functions, filters};
use utils::DateExtensions;
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
    let dates_per_month = ctx.begin.generate_dates_from_to(ctx.end, ctx.opts.sunday);


    let mut env = Environment::new();
    env.set_syntax(SyntaxConfig::builder()
        .line_statement_prefix("#")
        .line_comment_prefix("##")
        .build()
        .unwrap()
    );
    let default_template: String = "carl.tmpl".to_string();
    minijinja_embed::load_templates!(&mut env);

    if let Some(path) = ctx.config.template() {
        // We implement template overloading for the embedded
        // templates by iterating through the embedded templates
        // and checking if in the template path a template with
        // the same filename exists. If We find such a template,
        // we remove the embedded one from the environment.
        let mut remove_templates: Vec<String> = vec![];
        for (name, _) in env.templates() {
            if path.join(name).exists() {
                remove_templates.push(name.to_string());
            }
        }
        remove_templates.iter().for_each(|name| env.remove_template(name));
        env.set_loader(path_loader(path));
    }

    env.add_filter("days_in_year_left", filters::days_in_year_left);
    env.add_filter("percentage_of_year", filters::percentage_of_year);
    env.add_function("dates_to_columns", functions::dates_to_columns);
    env.add_function("reset_style", functions::reset_style);
    env.add_function("style_event", functions::style_event);
    env.add_function("style", functions::style);
    minijinja_contrib::add_to_environment(&mut env);

    let date_styler = objects::DateStyler::new(event_instances.clone(), ctx.usersetdate, ctx.specified_date, ctx.theme.clone(), ctx.styletype);
    let template_context = context! {
        cli => ctx.opts,
        columns => ctx.columns,
        dates_per_month => dates_per_month,
        event_instances => event_instances,
        main_date => ctx.usersetdate,
        style_date => minijinja::Value::from_object(date_styler),
    };

    match env.get_template(default_template.as_str()).and_then(|x| x.render(template_context)) {
        Ok(x) => { print!("{}", x); }
        Err(x) => { eprintln!("{}", x); }
    }
}
