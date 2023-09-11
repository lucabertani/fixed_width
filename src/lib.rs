use any_value::{AnyValue, AnyValueTrait};
use error::FixedWidthError;
use time::{format_description, macros::format_description};

pub mod any_value;
pub mod error;

extern crate fixed_width_derive;

pub fn pad(
    //bytes: &[u8],
    any_value: &dyn AnyValueTrait,
    size: usize,
    pad: u8,
    pad_left: bool,
    date_format: &str,
    time_format: &str,
    date_time_format: &str,
) -> Result<Vec<u8>, FixedWidthError> {
    let any_value = any_value.into_any_value();

    let any_value = match any_value {
        AnyValue::TimeDate(d) => {
            let format = format_description::parse(date_format).unwrap();
            let formatted = d.format(&format).unwrap();
            AnyValue::String(formatted)
        }
        AnyValue::TimeTime(t) => {
            let format = format_description::parse(time_format).unwrap();
            let formatted = t.format(&format).unwrap();
            AnyValue::String(formatted)
        }
        AnyValue::TimeDateTime(dt) => {
            let format = format_description::parse(date_time_format).unwrap();
            let formatted = dt.format(&format).unwrap();
            AnyValue::String(formatted)
        }
        _ => any_value,
    };
    //let mut bytes = any_value.into_any_value().to_bytes();
    let mut bytes = any_value.to_bytes();

    if bytes.len() > size {
        return Err(FixedWidthError::new(format!(
            "Expected size {}, got {} instead",
            size,
            bytes.len()
        )));
    }

    //let mut v = bytes.to_vec();

    for _ in 0..(size - bytes.len()) {
        match pad_left {
            true => bytes.push(pad),
            false => bytes.insert(0, pad),
        }
    }

    Ok(bytes)
}

pub trait FixedWidth: Sized {
    fn to_bytes(&self) -> Result<Vec<u8>, FixedWidthError>;
    fn to_string(&self) -> Result<String, FixedWidthError> {
        self.to_bytes()
            .and_then(|bytes| Ok(String::from_utf8(bytes).unwrap()))
    }
    //fn to_string(&self) -> Result<String, FixedRecordLengthError>;
}

/*#[derive(Clone, Debug)]
struct Field {
    size: u32,
    pad: Pad,
}

#[derive(Clone, Copy, Debug)]
enum Pad {
    Left,
    Right,
}
*/
