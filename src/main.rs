// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

mod cli;
mod config;
mod context;
mod events;
mod utils;

extern crate clap;
extern crate serde;
extern crate toml;
extern crate xdg;
use std::process;

use crate::utils::jinja_functions::reset_style;
use crate::utils::jinja_filters::dates_to_columns;
use context::Context;
use minijinja::{path_loader, Environment, Value};
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

    let mut env = Environment::new();
    env.set_syntax(SyntaxConfig::builder()
        .line_statement_prefix("#")
        .line_comment_prefix("##")
        .build()
        .unwrap()
    );
    env.set_loader(path_loader("templates"));
    minijinja_contrib::add_to_environment(&mut env);

    env.add_function("reset_style", reset_style);
    env.add_filter("dates_to_columns", dates_to_columns);

    let tmpl = env.get_template("carl.tmpl").unwrap();

    print!("{}", tmpl.render(Value::from_object(ctx)).unwrap());
}
