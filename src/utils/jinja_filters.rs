use crate::utils::helpers::MyDate;
use minijinja::value::{ViaDeserialize,Value};

pub fn dates_to_columns(dates: ViaDeserialize<Vec<Vec<MyDate>>>, columns: usize) -> Value {
    let mut months_columns: Vec<Vec<Vec<MyDate>>> = vec![];
    for chunk in dates.chunks(columns) {
        months_columns.push(chunk.to_vec());
    }
    let mut ret: Vec<Vec<Vec<Option<MyDate>>>> = vec![];
    for mut row in months_columns {
        let mut monthlines: Vec<Vec<Option<MyDate>>> = vec![];
        while row.iter().any(|x| !x.is_empty()) {
            let mut line: Vec<Option<MyDate>> = vec![];
            for month in &mut row {
                let mut foo: Vec<Option<MyDate>> = if month.len() >= 7 {
                    month.drain(..7).map(|x| Some(x)).collect()
                } else {
                    vec![None; 7]
                };
                line.append(&mut foo);
            }
            monthlines.push(line);
        }
        ret.push(monthlines)
    }
    Value::from_serialize(ret)
}
