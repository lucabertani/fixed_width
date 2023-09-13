use std::path::PathBuf;

use bigdecimal::BigDecimal;

use crate::FixedWidth;

// struct for keep a value of most used type
#[derive(Debug, Clone)]
pub enum AnyValue {
    String(String),
    TimeDate(time::Date),
    TimeTime(time::Time),
    TimeDateTime(time::PrimitiveDateTime),
    ChronoDate(chrono::NaiveDate),
    ChronoTime(chrono::NaiveTime),
    ChronoDateTime(chrono::NaiveDateTime),
    Number(AnyNumber),
    Bool(bool),
    Null(Option<String>),
    List(Vec<AnyValue>),
}
#[derive(Debug, Clone)]
pub enum AnyNumber {
    SmallInt(i16),
    Integer(i32),
    BigInteger(i64),
    BigDecimal(BigDecimal),
    //Float(f32),
    //Real(f64),
}

impl AnyValue {
    //TODO invertire la logica. Tutta la libreria ragiona in byte, non ha senso che qui ragiona in stringhe per poi riconvertirla in byte
    // lasciamo la conversione in byte come ultima operazione, direttamente dentro il trait FixedWidth
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn to_string(&self) -> String {
        match self {
            AnyValue::String(s) => s.to_string(),
            AnyValue::Number(n) => match n {
                AnyNumber::SmallInt(si) => si.to_string(),
                AnyNumber::Integer(i) => i.to_string(),
                AnyNumber::BigInteger(bi) => bi.to_string(),
                AnyNumber::BigDecimal(bi) => bi.to_string(),
                /*AnyNumber::Float(f) => f.to_string(),
                AnyNumber::Real(r) => r.to_string(),*/
            },
            AnyValue::Bool(b) => format!("{}", i16::from(*b)),
            AnyValue::Null(_) => String::new(),
            AnyValue::List(list) => {
                let mut res = String::new();
                for el in list {
                    let s = el.to_string();
                    res.push_str(s.as_str());
                }
                res
            }
            _ => panic!("can not call .to_string() on variable {:?}", self),
        }
    }
}

// Trait for convert a value into AnyValue
pub trait AnyValueTrait: Send + Sync {
    fn into_any_value(&self) -> AnyValue;
}
pub struct AnyValueNull {}
impl AnyValueNull {
    pub fn new() -> AnyValueNull {
        AnyValueNull {}
    }
}
impl AnyValueTrait for AnyValueNull {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Null(None)
    }
}

impl AnyValueTrait for PathBuf {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::String(self.to_string_lossy().as_ref().to_string())
    }
}

impl AnyValueTrait for &str {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::String(self.to_string())
    }
}
impl AnyValueTrait for String {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::String(self.clone())
    }
}
impl AnyValueTrait for u16 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::SmallInt(*self as i16))
    }
}
impl AnyValueTrait for i16 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::SmallInt(*self))
    }
}
impl AnyValueTrait for i32 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Integer(*self))
    }
}
impl AnyValueTrait for u32 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Integer(*self as i32))
    }
}
/*impl AnyValueTrait for f32 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Float(*self))
    }
}*/
impl AnyValueTrait for BigDecimal {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::BigDecimal(self.clone()))
    }
}
impl AnyValueTrait for i64 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::BigInteger(*self))
    }
}
impl AnyValueTrait for Option<i64> {
    fn into_any_value(&self) -> AnyValue {
        match self {
            Some(v) => AnyValue::Number(AnyNumber::BigInteger(*v)),
            None => AnyValue::Null(None),
        }
    }
}
impl AnyValueTrait for u64 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::BigInteger(*self as i64))
    }
}
/*impl AnyValueTrait for f64 {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Number(AnyNumber::Real(*self))
    }
}*/
impl AnyValueTrait for time::Date {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::TimeDate(self.clone())
    }
}
impl AnyValueTrait for time::Time {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::TimeTime(self.clone())
    }
}
impl AnyValueTrait for time::PrimitiveDateTime {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::TimeDateTime(self.clone())
    }
}
impl AnyValueTrait for chrono::NaiveDate {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::ChronoDate(self.clone())
    }
}
impl AnyValueTrait for chrono::NaiveTime {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::ChronoTime(self.clone())
    }
}
impl AnyValueTrait for chrono::NaiveDateTime {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::ChronoDateTime(self.clone())
    }
}

impl AnyValueTrait for bool {
    fn into_any_value(&self) -> AnyValue {
        AnyValue::Bool(*self)
    }
}

impl<T> AnyValueTrait for Vec<T>
where
    T: FixedWidth + Send + Sync,
{
    fn into_any_value(&self) -> AnyValue {
        let mut results = Vec::new();
        for el in self {
            let s = el.to_string().unwrap();
            results.push(AnyValue::String(s));
        }
        AnyValue::List(results)
    }
}
