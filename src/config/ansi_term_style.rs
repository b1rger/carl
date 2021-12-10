// SPDX-FileCopyrightText: 2021 Birger Schacht <birger@rantanplan.org>
// SPDX-License-Identifier: GPL-3.0-or-later
use ansi_term::Colour::*;
use ansi_term::Style;
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
                AnsiTermStyle::Reverse => style.reverse(),
                AnsiTermStyle::Hidden => {
                    let mut s = style.hidden();
                    s.background = None;
                    s
                }
                AnsiTermStyle::Strikethrough => style.strikethrough(),

                AnsiTermStyle::FGBlack => style.fg(Black),
                AnsiTermStyle::FGRed => style.fg(Red),
                AnsiTermStyle::FGGreen => style.fg(Green),
                AnsiTermStyle::FGYellow => style.fg(Yellow),
                AnsiTermStyle::FGBlue => style.fg(Blue),
                AnsiTermStyle::FGPurple => style.fg(Purple),
                AnsiTermStyle::FGCyan => style.fg(Cyan),
                AnsiTermStyle::FGWhite => style.fg(White),
                AnsiTermStyle::FGrgb { r, g, b } => style.fg(RGB(*r, *g, *b)),
                AnsiTermStyle::FGFixed(x) => style.fg(Fixed(*x)),

                AnsiTermStyle::BGBlack => style.on(Black),
                AnsiTermStyle::BGRed => style.on(Red),
                AnsiTermStyle::BGGreen => style.on(Green),
                AnsiTermStyle::BGYellow => style.on(Yellow),
                AnsiTermStyle::BGBlue => style.on(Blue),
                AnsiTermStyle::BGPurple => style.on(Purple),
                AnsiTermStyle::BGCyan => style.on(Cyan),
                AnsiTermStyle::BGWhite => style.on(White),
                AnsiTermStyle::BGrgb { r, g, b } => style.on(RGB(*r, *g, *b)),
                AnsiTermStyle::BGFixed(x) => style.on(Fixed(*x)),
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
        assert_eq![a.to_style(), b.bold().fg(Black).on(Black)];
    }
    #[test]
    fn test_to_style2() {
        let a = vec![
            AnsiTermStyle::Dimmed,
            AnsiTermStyle::FGRed,
            AnsiTermStyle::BGRed,
        ];
        let b = Style::default();
        assert_eq![a.to_style(), b.dimmed().fg(Red).on(Red)];
    }
    #[test]
    fn test_to_style3() {
        let a = vec![
            AnsiTermStyle::Italic,
            AnsiTermStyle::FGGreen,
            AnsiTermStyle::BGGreen,
        ];
        let b = Style::default();
        assert_eq![a.to_style(), b.italic().fg(Green).on(Green)];
    }
    #[test]
    fn test_to_style4() {
        let a = vec![
            AnsiTermStyle::Underline,
            AnsiTermStyle::FGYellow,
            AnsiTermStyle::BGYellow,
        ];
        let b = Style::default();
        assert_eq![a.to_style(), b.underline().fg(Yellow).on(Yellow)];
    }
    #[test]
    fn test_to_style5() {
        let a = vec![
            AnsiTermStyle::Blink,
            AnsiTermStyle::FGBlue,
            AnsiTermStyle::BGBlue,
        ];
        let b = Style::default();
        assert_eq![a.to_style(), b.blink().fg(Blue).on(Blue)];
    }
    #[test]
    fn test_to_style6() {
        let a = vec![
            AnsiTermStyle::Reverse,
            AnsiTermStyle::FGPurple,
            AnsiTermStyle::BGPurple,
        ];
        let b = Style::default();
        assert_eq![a.to_style(), b.reverse().fg(Purple).on(Purple)];
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
        assert_eq![a.to_style(), b.strikethrough().fg(Cyan).on(Cyan)];
    }
    #[test]
    fn test_to_style9() {
        let a = vec![AnsiTermStyle::FGWhite, AnsiTermStyle::BGWhite];
        let b = Style::default();
        assert_eq![a.to_style(), b.fg(White).on(White)];
    }
    #[test]
    fn test_to_style10() {
        let a = vec![AnsiTermStyle::FGFixed(17), AnsiTermStyle::BGFixed(71)];
        let b = Style::default();
        assert_eq![a.to_style(), b.on(Fixed(71)).fg(Fixed(17))];
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
        assert_eq![a.to_style(), b.fg(RGB(17, 18, 19)).on(RGB(17, 18, 19))];
    }
}
