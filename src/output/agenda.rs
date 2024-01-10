// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::utils::convertstyle;
use crate::Context;
use anstyle::Style;

use std::fmt;

pub struct Agenda<'a> {
    pub ctx: &'a Context,
}

impl fmt::Display for Agenda<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style = Style::default().bold();
        let mut ret: String = format!("{}Agenda:{}\n", style.render(), style.render_reset());
        for ei in self.ctx.eventinstances.iter() {
            let datestr = if self.ctx.opts.julian {
                format!("{}", ei.date.format("%j"))
            } else {
                format!("{}", ei.date)
            };
            let style = &convertstyle(ei.style.stylenames.to_vec(), "·");
            ret += format!("{style} {datestr}: {}\n", ei.event.summary).as_str();
        }
        write!(f, "{}", ret)
    }
}
