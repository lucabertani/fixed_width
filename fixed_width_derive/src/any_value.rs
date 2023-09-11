use std::path::PathBuf;

// struct for keep a value of most used type
#[derive(Debug, Clone)]
pub enum AnyValue {
    String(String),
    Date(time::Date),
    DateTime(time::PrimitiveDateTime),
    Number(AnyNumber),
    //List(Vec<AnyValue>),
    Bool(bool),
    Null(Option<String>),
}
#[derive(Debug, Clone)]
pub enum AnyNumber {
    SmallInt(i16),
    Integer(i32),
    BigInteger(i64),
    Float(f32),
    Real(f64),
}

impl AnyValue {
    pub fn to_string(&self) -> String {
        match self {
            AnyValue::String(s) => s.to_string(),
            AnyValue::Date(d) => todo!(),
            AnyValue::DateTime(dt) => todo!(),
            AnyValue::Number(n) => match n {
                AnyNumber::SmallInt(si) => si.to_string(),
                AnyNumber::Integer(i) => i.to_string(),
                AnyNumber::BigInteger(bi) => bi.to_string(),
                AnyNumber::Float(f) => f.to_string(),
                AnyNumber::Real(r) => r.to_string(),
            },
            AnyValue::Bool(b) => format!("{}", i16::from(*b)),
            AnyValue::Null(_) => String::new(),
        }
        /*match self {
            AnyValue::Sql(s) => Ok(s.to_string()), // used only in internal functions (like increase Version field)
            AnyValue::Bool(b) => Ok(format!("{}", i16::from(*b))),
            AnyValue::String(s) => Ok(format!("'{}'", sanitize(s))),
            AnyValue::ByteArray(array) => {
                let str = array
                    .iter()
                    .map(|el| format!("{:02X}", el))
                    .collect::<Vec<String>>()
                    .join("");
                Ok(format!("0x{str}"))
            }
            AnyValue::Date(d) => {
                // https://learn.microsoft.com/en-us/sql/t-sql/functions/cast-and-convert-transact-sql?view=sql-server-ver16
                //let date_formatted =  d.format("%Y-%m-%d")
                //format!("CONVERT(DATE, '{}', 20)", d.format("%Y-%m-%d"))
                let format = format_description!("[year padding:none]-[month]-[day]");
                let date_formatted = d
                    .format(format)
                    .context(format!("Unable to format date {:#?}", d))?;
                Ok(format!("CONVERT(DATE, '{}', 20)", date_formatted))
            }
            AnyValue::DateTime(dt) => {
                //format!("CONVERT(DATETIME, '{}', 20)", dt.format("%Y-%m-%d %H:%M:%S"))
                let format = format_description!(
                    "[year padding:none]-[month]-[day] [hour padding:none]:[minute]:[second]"
                );
                let date_formatted = dt
                    .format(format)
                    .context(format!("Unable to format date {:#?}", dt))?;
                Ok(format!("CONVERT(DATETIME, '{}', 20)", date_formatted))
            }
            AnyValue::Number(n) => match n {
                AnyNumber::SmallInt(si) => Ok(format!("{}", si)),
                AnyNumber::Integer(i) => Ok(format!("{}", i)),
                AnyNumber::BigInteger(bi) => Ok(format!("{}", bi)),
                AnyNumber::Float(f) => Ok(format!("{}", f)),
                AnyNumber::Real(r) => Ok(format!("{}", r)),
            },
            AnyValue::List(list) => {
                let mut result = String::new();
                let n = list.len();
                for (i, any_value) in list.into_iter().enumerate() {
                    let s = any_value.format_to_sql()?;
                    result.push_str(s.as_str());

                    if (i + 1) < n {
                        result.push_str(", ");
                    }
                }

                Ok(result)
            }
            AnyValue::Null(_) => Ok(format!("NULL")),
        }*/
    }
}

// Trait for convert a value into AnyValue
pub trait AnyValueSql: Send + Sync {
    fn into_any_value(&self) -> AnyValue;
}
pub struct AnyValueNull {}
impl AnyValueNull {
    pub fn new() -> AnyValueNull {
        AnyValueNull {}
    }
}
impl AnyValueSql for AnyValueNull {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Null(None)
    }
}

impl AnyValueSql for PathBuf {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::String(self.to_string_lossy().as_ref().to_string())
    }
}

impl AnyValueSql for &str {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::String(self.to_string())
    }
}
impl AnyValueSql for String {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::String(self.clone())
    }
}
impl AnyValueSql for u16 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::SmallInt(*self as i16))
    }
}
impl AnyValueSql for i16 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::SmallInt(*self))
    }
}
impl AnyValueSql for i32 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Integer(*self))
    }
}
impl AnyValueSql for u32 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Integer(*self as i32))
    }
}
impl AnyValueSql for f32 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Float(*self))
    }
}
impl AnyValueSql for i64 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::BigInteger(*self))
    }
}
impl AnyValueSql for Option<i64> {
    fn into_any_value(&self) -> AnyValue {
        match self {
            Some(v) => AnyValue::Number(AnyNumber::BigInteger(*v)),
            None => AnyValue::Null(None),
        }
    }
}
impl AnyValueSql for u64 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::BigInteger(*self as i64))
    }
}
impl AnyValueSql for f64 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Real(*self))
    }
}
impl AnyValueSql for time::Date {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Date(self.clone())
    }
}
impl AnyValueSql for time::PrimitiveDateTime {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::DateTime(self.clone())
    }
}

impl AnyValueSql for bool {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Bool(*self)
    }
}