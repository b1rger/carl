// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::config::DateProperty;
use crate::config::Style as MyStyle;
use crate::config::StyleName;
use crate::config::Theme;
use crate::cli::Cli;
use crate::events::EventInstance;
use crate::utils::DateExtensions;
use anstyle::Ansi256Color;
use anstyle::AnsiColor::*;
use anstyle::RgbColor;
use anstyle::Style;
use chrono::Datelike;
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyDate {
    pub date: chrono::NaiveDate,
    pub style: String,
}

impl MyDate {
    pub fn new(
        date: chrono::NaiveDate,
        month: chrono::NaiveDate,
        maindate: chrono::NaiveDate,
        theme: &Theme,
        eventinstances: &Vec<EventInstance>,
    ) -> MyDate {
        let mut matching_styles: Vec<MyStyle> = theme
            .date
            .iter()
            .filter(|datestyle| {
                satisfy_all(
                    date,
                    month.first_day_of_month(),
                    maindate,
                    &eventinstances,
                    &datestyle.properties,
                )
            })
            .cloned()
            .map(|datestyle| datestyle.style)
            .collect();

        for eventinstance in eventinstances {
            if eventinstance.date == date {
                matching_styles.push(eventinstance.style.clone());
            }
        }

        matching_styles.sort_by(|a, b| a.weight.cmp(&b.weight));
        let mut stylenames = vec![];
        for mut style in matching_styles {
            stylenames.append(&mut style.stylenames);
        }

        let style = tostyle(stylenames).render().to_string();

        MyDate { date, style }
    }
}

pub fn satisfy_all(
    date: chrono::NaiveDate,
    firstdayofmonth: chrono::NaiveDate,
    maindate: chrono::NaiveDate,
    events: &[EventInstance],
    properties: &[DateProperty],
) -> bool {
    properties.iter().all(|prop| match prop {
        DateProperty::FirstDayOfMonth => date == firstdayofmonth,
        DateProperty::BeforeFirstDayOfMonth => date < firstdayofmonth,
        DateProperty::BeforeCurrentDate => date < maindate,
        DateProperty::CurrentDate => date == maindate,
        DateProperty::AfterCurrentDate => date > maindate,
        DateProperty::AfterLastDayOfMonth => date > firstdayofmonth.last_day_of_month(),
        DateProperty::LastDayOfMonth => date == firstdayofmonth.last_day_of_month(),
        DateProperty::IsEvent => events
            .iter()
            .any(|eventinstance| eventinstance.date == date),
        DateProperty::Monday => date.weekday() == chrono::Weekday::Mon,
        DateProperty::Tuesday => date.weekday() == chrono::Weekday::Tue,
        DateProperty::Wednesday => date.weekday() == chrono::Weekday::Wed,
        DateProperty::Thursday => date.weekday() == chrono::Weekday::Thu,
        DateProperty::Friday => date.weekday() == chrono::Weekday::Fri,
        DateProperty::Saturday => date.weekday() == chrono::Weekday::Sat,
        DateProperty::Sunday => date.weekday() == chrono::Weekday::Sun,
        DateProperty::Odd => date.day() % 2 == 1,
        DateProperty::Even => date.day() % 2 == 0,
    })
}

pub fn tostyle(styles: Vec<StyleName>) -> Style {
    let mut style = Style::default();
    for ansistyle in styles {
        style = match ansistyle {
            StyleName::Bold => style.bold(),
            StyleName::Dimmed => style.dimmed(),
            StyleName::Italic => style.italic(),
            StyleName::Underline => style.underline(),
            StyleName::Blink => style.blink(),
            StyleName::Reverse => style.effects(anstyle::Effects::INVERT),
            StyleName::Hidden => style.hidden().bg_color(None),
            StyleName::Strikethrough => style.strikethrough(),

            StyleName::FGBlack => style.fg_color(Some(Black.into())),
            StyleName::FGRed => style.fg_color(Some(Red.into())),
            StyleName::FGGreen => style.fg_color(Some(Green.into())),
            StyleName::FGYellow => style.fg_color(Some(Yellow.into())),
            StyleName::FGBlue => style.fg_color(Some(Blue.into())),
            StyleName::FGPurple => style.fg_color(Some(Magenta.into())),
            StyleName::FGCyan => style.fg_color(Some(Cyan.into())),
            StyleName::FGWhite => style.fg_color(Some(White.into())),
            StyleName::FGrgb { r, g, b } => style.fg_color(Some(RgbColor(r, g, b).into())),
            StyleName::FGFixed(x) => style.fg_color(Some(Ansi256Color(x).into())),

            StyleName::BGBlack => style.bg_color(Some(Black.into())),
            StyleName::BGRed => style.bg_color(Some(Red.into())),
            StyleName::BGGreen => style.bg_color(Some(Green.into())),
            StyleName::BGYellow => style.bg_color(Some(Yellow.into())),
            StyleName::BGBlue => style.bg_color(Some(Blue.into())),
            StyleName::BGPurple => style.bg_color(Some(Magenta.into())),
            StyleName::BGCyan => style.bg_color(Some(Cyan.into())),
            StyleName::BGWhite => style.bg_color(Some(White.into())),
            StyleName::BGrgb { r, g, b } => style.bg_color(Some(RgbColor(r, g, b).into())),
            StyleName::BGFixed(x) => style.bg_color(Some(Ansi256Color(x).into())),
        }
    }
    style
}

pub fn generate_dates_from_to(
    begin: chrono::NaiveDate,
    end: chrono::NaiveDate,
    maindate: chrono::NaiveDate,
    opts: &Cli,
    theme: &Theme,
    eventinstances: &Vec<EventInstance>,
) -> Vec<Vec<MyDate>> {
    let mut dates: Vec<Vec<MyDate>> = vec![];
    let mut months = vec![begin.first_day_of_month()];
    while months.last().unwrap().first_day_of_next_month() <= end {
        months.push(months.last().unwrap().first_day_of_next_month());
    }

    for month in months {
        let mut month_v: Vec<MyDate> = vec![];
        let mut date = month
            .first_day_of_month()
            .first_day_of_week_before_first_day_of_month(opts.sunday);
        while date
            <= month
                .last_day_of_month()
                .last_day_of_week_after_last_day_of_month(opts.sunday)
        {
            month_v.push(MyDate::new(date, month, maindate, theme, eventinstances));
            date += Duration::days(1);
        }
        dates.push(month_v)
    }
    dates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_style1() {
        let a = vec![StyleName::Bold, StyleName::FGBlack, StyleName::BGBlack];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.bold()
                .fg_color(Some(Black.into()))
                .bg_color(Some(Black.into()))
        ];
    }
    #[test]
    fn test_to_style2() {
        let a = vec![StyleName::Dimmed, StyleName::FGRed, StyleName::BGRed];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.dimmed()
                .fg_color(Some(Red.into()))
                .bg_color(Some(Red.into()))
        ];
    }
    #[test]
    fn test_to_style3() {
        let a = vec![StyleName::Italic, StyleName::FGGreen, StyleName::BGGreen];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.italic()
                .fg_color(Some(Green.into()))
                .bg_color(Some(Green.into()))
        ];
    }
    #[test]
    fn test_to_style4() {
        let a = vec![
            StyleName::Underline,
            StyleName::FGYellow,
            StyleName::BGYellow,
        ];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.underline()
                .fg_color(Some(Yellow.into()))
                .bg_color(Some(Yellow.into()))
        ];
    }
    #[test]
    fn test_to_style5() {
        let a = vec![StyleName::Blink, StyleName::FGBlue, StyleName::BGBlue];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.blink()
                .fg_color(Some(Blue.into()))
                .bg_color(Some(Blue.into()))
        ];
    }
    #[test]
    fn test_to_style6() {
        let a = vec![StyleName::Reverse, StyleName::FGPurple, StyleName::BGPurple];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.effects(anstyle::Effects::INVERT)
                .fg_color(Some(Magenta.into()))
                .bg_color(Some(Magenta.into()))
        ];
    }
    #[test]
    fn test_to_style7() {
        let a = vec![StyleName::BGRed, StyleName::Hidden];
        let b = Style::default();
        assert_eq![tostyle(a), b.hidden()];
    }
    #[test]
    fn test_to_style8() {
        let a = vec![
            StyleName::Strikethrough,
            StyleName::FGCyan,
            StyleName::BGCyan,
        ];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.strikethrough()
                .fg_color(Some(Cyan.into()))
                .bg_color(Some(Cyan.into()))
        ];
    }
    #[test]
    fn test_to_style9() {
        let a = vec![StyleName::FGWhite, StyleName::BGWhite];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.fg_color(Some(White.into())).bg_color(Some(White.into()))
        ];
    }
    #[test]
    fn test_to_style10() {
        let a = vec![StyleName::FGFixed(17), StyleName::BGFixed(71)];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.bg_color(Some(Ansi256Color(71).into()))
                .fg_color(Some(Ansi256Color(17).into()))
        ];
    }
    #[test]
    fn test_to_style11() {
        let a = vec![
            StyleName::FGrgb {
                r: 17,
                g: 18,
                b: 19,
            },
            StyleName::BGrgb {
                r: 17,
                g: 18,
                b: 19,
            },
        ];
        let b = Style::default();
        assert_eq![
            tostyle(a),
            b.fg_color(Some(RgbColor(17, 18, 19).into()))
                .bg_color(Some(RgbColor(17, 18, 19).into()))
        ];
    }
}
