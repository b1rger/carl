use minijinja::value::ViaDeserialize;
use minijinja::value::Value;

pub fn reset_style() -> String {
    "\x1B[0m".to_string()
}

pub(crate) fn dates_to_columns(dates: ViaDeserialize<Vec<Vec<chrono::NaiveDate>>>, columns: usize) -> Result<Value, minijinja::Error> {
    let mut months_columns: Vec<Vec<Vec<chrono::NaiveDate>>> = vec![];
    for chunk in dates.chunks(columns) {
        months_columns.push(chunk.to_vec());
    }
    let mut ret: Vec<Vec<Vec<Option<chrono::NaiveDate>>>> = vec![];
    for mut row in months_columns {
        let mut monthlines: Vec<Vec<Option<chrono::NaiveDate>>> = vec![];
        while row.iter().any(|x| !x.is_empty()) {
            let mut line: Vec<Option<chrono::NaiveDate>> = vec![];
            for month in &mut row {
                let mut week: Vec<Option<chrono::NaiveDate>> = if month.len() >= 7 {
                    month.drain(..7).map(Some).collect()
                } else {
                    vec![None; 7]
                };
                line.append(&mut week);
            }
            monthlines.push(line);
        }
        ret.push(monthlines)
    }
    Ok(Value::from_serialize(ret))
}
