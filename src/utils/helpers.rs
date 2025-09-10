// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crate::config::StyleName;
use anstyle::Ansi256Color;
use anstyle::AnsiColor::*;
use anstyle::RgbColor;
use anstyle::Style;

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
