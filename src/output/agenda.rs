// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::Context;

use std::fmt;

pub struct Agenda<'a> {
    pub ctx: &'a Context,
}

impl fmt::Display for Agenda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret: String = String::new();
        for pe in self.ctx.eventinstances.iter() {
            ret += format!("{pe}\n").as_str();
        }
        write!(f, "{}", ret)
    }
}
