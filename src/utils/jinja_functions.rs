use anstyle::Reset;

pub fn reset_style() -> String {
    format!("{}", Reset)
}
