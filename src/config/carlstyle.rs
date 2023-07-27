// SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use anstyle::Ansi256Color;
use anstyle::AnsiColor::*;
use anstyle::RgbColor;
use anstyle::Style;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AnsiTermStyle {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,

    FGBlack,
    FGRed,
    FGGreen,
    FGYellow,
    FGBlue,
    FGPurple,
    FGCyan,
    FGWhite,
    #[serde(rename = "FGRGB")]
    FGrgb {
        r: u8,
        g: u8,
        b: u8,
    },
    FGFixed(u8),

    BGBlack,
    BGRed,
    BGGreen,
    BGYellow,
    BGBlue,
    BGPurple,
    BGCyan,
    BGWhite,
    #[serde(rename = "BGRGB")]
    BGrgb {
        r: u8,
        g: u8,
        b: u8,
    },
    BGFixed(u8),
}

type AnsiTermStyleVector = Vec<AnsiTermStyle>;

pub trait StyleConversion {
    fn to_style(&self) -> Style;
}

impl StyleConversion for AnsiTermStyleVector {
    fn to_style(&self) -> Style {
        let mut style = Style::default();
        for ansistyle in self {
            style = match ansistyle {
                AnsiTermStyle::Bold => style.bold(),
                AnsiTermStyle::Dimmed => style.dimmed(),
                AnsiTermStyle::Italic => style.italic(),
                AnsiTermStyle::Underline => style.underline(),
                AnsiTermStyle::Blink => style.blink(),
                AnsiTermStyle::Reverse => style.effects(anstyle::Effects::INVERT),
                AnsiTermStyle::Hidden => style.hidden().bg_color(None),
                AnsiTermStyle::Strikethrough => style.strikethrough(),

                AnsiTermStyle::FGBlack => style.fg_color(Some(Black.into())),
                AnsiTermStyle::FGRed => style.fg_color(Some(Red.into())),
                AnsiTermStyle::FGGreen => style.fg_color(Some(Green.into())),
                AnsiTermStyle::FGYellow => style.fg_color(Some(Yellow.into())),
                AnsiTermStyle::FGBlue => style.fg_color(Some(Blue.into())),
                AnsiTermStyle::FGPurple => style.fg_color(Some(Magenta.into())),
                AnsiTermStyle::FGCyan => style.fg_color(Some(Cyan.into())),
                AnsiTermStyle::FGWhite => style.fg_color(Some(White.into())),
                AnsiTermStyle::FGrgb { r, g, b } => {
                    style.fg_color(Some(RgbColor(*r, *g, *b).into()))
                }
                AnsiTermStyle::FGFixed(x) => style.fg_color(Some(Ansi256Color(*x).into())),

                AnsiTermStyle::BGBlack => style.bg_color(Some(Black.into())),
                AnsiTermStyle::BGRed => style.bg_color(Some(Red.into())),
                AnsiTermStyle::BGGreen => style.bg_color(Some(Green.into())),
                AnsiTermStyle::BGYellow => style.bg_color(Some(Yellow.into())),
                AnsiTermStyle::BGBlue => style.bg_color(Some(Blue.into())),
                AnsiTermStyle::BGPurple => style.bg_color(Some(Magenta.into())),
                AnsiTermStyle::BGCyan => style.bg_color(Some(Cyan.into())),
                AnsiTermStyle::BGWhite => style.bg_color(Some(White.into())),
                AnsiTermStyle::BGrgb { r, g, b } => {
                    style.bg_color(Some(RgbColor(*r, *g, *b).into()))
                }
                AnsiTermStyle::BGFixed(x) => style.bg_color(Some(Ansi256Color(*x).into())),
            }
        }
        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_style1() {
        let a = vec![
            AnsiTermStyle::Bold,
            AnsiTermStyle::FGBlack,
            AnsiTermStyle::BGBlack,
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.bold()
                .fg_color(Some(Black.into()))
                .bg_color(Some(Black.into()))
        ];
    }
    #[test]
    fn test_to_style2() {
        let a = vec![
            AnsiTermStyle::Dimmed,
            AnsiTermStyle::FGRed,
            AnsiTermStyle::BGRed,
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.dimmed()
                .fg_color(Some(Red.into()))
                .bg_color(Some(Red.into()))
        ];
    }
    #[test]
    fn test_to_style3() {
        let a = vec![
            AnsiTermStyle::Italic,
            AnsiTermStyle::FGGreen,
            AnsiTermStyle::BGGreen,
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.italic()
                .fg_color(Some(Green.into()))
                .bg_color(Some(Green.into()))
        ];
    }
    #[test]
    fn test_to_style4() {
        let a = vec![
            AnsiTermStyle::Underline,
            AnsiTermStyle::FGYellow,
            AnsiTermStyle::BGYellow,
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.underline()
                .fg_color(Some(Yellow.into()))
                .bg_color(Some(Yellow.into()))
        ];
    }
    #[test]
    fn test_to_style5() {
        let a = vec![
            AnsiTermStyle::Blink,
            AnsiTermStyle::FGBlue,
            AnsiTermStyle::BGBlue,
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.blink()
                .fg_color(Some(Blue.into()))
                .bg_color(Some(Blue.into()))
        ];
    }
    #[test]
    fn test_to_style6() {
        let a = vec![
            AnsiTermStyle::Reverse,
            AnsiTermStyle::FGPurple,
            AnsiTermStyle::BGPurple,
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.effects(anstyle::Effects::INVERT)
                .fg_color(Some(Magenta.into()))
                .bg_color(Some(Magenta.into()))
        ];
    }
    #[test]
    fn test_to_style7() {
        let a = vec![AnsiTermStyle::BGRed, AnsiTermStyle::Hidden];
        let b = Style::default();
        assert_eq![a.to_style(), b.hidden()];
    }
    #[test]
    fn test_to_style8() {
        let a = vec![
            AnsiTermStyle::Strikethrough,
            AnsiTermStyle::FGCyan,
            AnsiTermStyle::BGCyan,
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.strikethrough()
                .fg_color(Some(Cyan.into()))
                .bg_color(Some(Cyan.into()))
        ];
    }
    #[test]
    fn test_to_style9() {
        let a = vec![AnsiTermStyle::FGWhite, AnsiTermStyle::BGWhite];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.fg_color(Some(White.into())).bg_color(Some(White.into()))
        ];
    }
    #[test]
    fn test_to_style10() {
        let a = vec![AnsiTermStyle::FGFixed(17), AnsiTermStyle::BGFixed(71)];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.bg_color(Some(Ansi256Color(71).into()))
                .fg_color(Some(Ansi256Color(17).into()))
        ];
    }
    #[test]
    fn test_to_style11() {
        let a = vec![
            AnsiTermStyle::FGrgb {
                r: 17,
                g: 18,
                b: 19,
            },
            AnsiTermStyle::BGrgb {
                r: 17,
                g: 18,
                b: 19,
            },
        ];
        let b = Style::default();
        assert_eq![
            a.to_style(),
            b.fg_color(Some(RgbColor(17, 18, 19).into()))
                .bg_color(Some(RgbColor(17, 18, 19).into()))
        ];
    }
}
