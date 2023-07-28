<!--
SPDX-FileCopyrightText: 2021-2023 Birger Schacht <birger@rantanplan.org>
SPDX-License-Identifier: MIT
-->
<div align="center">

# carl

![carl logo](https://codeberg.org/birger/carl/media/branch/main/data/logo.svg)

`carl` is a calendar for the commandline. It tries to mimic the various
`cal(1)` implementations out there, but also adds enhanced features like colors
and ical support.

**If you find any bugs or have ideas for additional features, please don't hesitate to create a bug report or a feature request [on codeberg](https://codeberg.org/birger/carl/issues/new) or [github](https://github.com/b1rger/carl/issues/new).**

---

<table style="table-layout: fixed; width: 100%;">
<tr><th>

Default output of `carl`

</th><th>

`carl` output with custom colors

</th><th>

`carl` output with events<br/>from ical highlighted

</th></tr>
<tr><td>

![Screenshots of default carl](https://codeberg.org/birger/carl/media/branch/main/data/screenshot-default.png)

</td><td>

![Screenshot of carl with custom colors](https://codeberg.org/birger/carl/media/branch/main/data/screenshot-custom.png)

</td><td>

![Screenshot of carl with ical events highlighted](https://codeberg.org/birger/carl/media/branch/main/data/screenshot-ical.png)

</td></tr>
</table>

</div>

## Installation

```
cargo install carl
```

## Commandline flags

- **-1**, **--one**: Display single month output.  (This is the default.)
- **-3**, **--three**: Display prev/current/next month output.
- **-s**, **--sunday**: Display Sunday as the first day of the week.
- **-m**, **--monday**: Display Monday as the first day of the week.
- **-j**, **--julian**: Display Julian dates (days one-based, numbered from January 1).
- **-y**, **--year**: Display a calendar for the current year.
- **-V**, **--version**: Display version information and exit.
- **-h**, **--help**: Display help screen and exit.
- **--theme**: Set the theme that should be used
- **--themestyletype**: "dark" or "light", use the theme styles marked for "dark" or for "light" backgrounds. Defaults to "light"

## Commandline options

- **YYYY**: Display the current date from the given year
- **MM YYYYY**: Display the given month in the given year
- **DD MM YYYY**: Display the given date

## Configuration file

The configuration file is located `XDG_CONFIG_DIRS/carl/config.toml` or `XDG_CONFIG_HOME/.carl/config.toml` (the latter has precedence).

The configuration file can define the name of a themefile and multiple icalfiles.
Themefiles contain listings of date properties together with styledefintions.
Icalfile listings contain paths to icalfiles together with styledefinitions.

The name of a them is simple specified using the `theme = ` setting:
```
theme = "default"
```
The themefile is read from `XDG_CONFIG_DIRS/carl/<themename>.toml` or `XDG_CONFIG_HOME/.carl/<themename>.toml` (the latter has precedence).

Icalfiles can be specified using the `[[ical]]` setting:
```
[[ical]]
file = "/home/user/birthdays.ics"
stylenames = ['FGPurple']
```

If the `file` setting points to a directory, `carl` uses all the files in that directory it can parse.

A sample configuration file is located in [data/config.toml](data/config.toml).

### Styles

Themefiles and Icalfile listings can contain custom style settings. A style changes how a specific date in the calendar is displayed.
A style consists of a list of stylenames and optionally a weight and a styletype (`'Dark'` or `'Light'`). If no styletype is set, the style is effective in either case.
The various possible stylenames are listed at the bottom.

Example:
```
stylenames = ['Dimmed']
weight = 10
styletype = 'Dark'
```

## Themefile

A themefile consists of a collection of datestyles:

### Datestyles

A datestyle consists of a list of properties of a date and a style. The date has to fullfill *all* of the properties for the style to be applied.

Example:
```
[[date]]
properties = ['CurrentDate']
stylenames = ['FGRed']
weight = 3
styletype = 'Light'
```

#### Possible properties

- `FirstDayOfMonth`:
- `BeforeFirstDayOfMonth`:
- `BeforeCurrentDate`:
- `CurrentDate`:
- `AfterCurrentDate`:
- `AfterLastDayOfMonth`:
- `LastDayOfMonth`:
- `IsEvent`:

A sample theme file is located in [data/default.theme](data/default.theme).

## Stylenames

### Styles
- `Bold`
- `Dimmed`
- `Italic`
- `Underline`
- `Blink`
- `Reverse`
- `Hidden`
- `Strikethrough`

### Foreground color names
- `FGBlack`
- `FGRed`
- `FGGreen`
- `FGYellow`
- `FGBlue`
- `FGPurple`
- `FGCyan`
- `FGWhite`
- `{ FGRGB = {r = x, g = y, b = z }}` where x, y and z are the RGB values
- `{ FGFixed = x }` a color number from 0 to 255, see the [color chart](https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg)

### Background color names
- `BGBlack`
- `BGRed`
- `BGGreen`
- `BGYellow`
- `BGBlue`
- `BGPurple`
- `BGCyan`
- `BGWhite`
- `{BGRGB = { r = x, g = y, b = z }}` where x, y and z are the RGB values
- `{BGFixed = x }` a color number from 0 to 255, see the [color chart](https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg)

Multiple styles and colors can be combined using lists: `["Bold", "FGRed", "Underline"]`
