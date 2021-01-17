use std::fmt;

use chrono::prelude::*;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::Serializer;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Stamp(DateTime<Utc>);

impl Stamp {
    pub fn format(&self) -> String {
        self.0.to_string()
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl FromStr for Stamp {
    type Err = StampError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let naive = match s.len() {
            10 => s
                .parse::<NaiveDate>()?
                .and_time(NaiveTime::from_hms(0, 0, 0)),
            _ => s.parse::<NaiveDateTime>()?,
        };

        let inner = DateTime::<Utc>::from_utc(naive, Utc);

        Ok(Stamp(inner))
    }
}

#[derive(Error, Debug)]
pub enum StampError {
    #[error(transparent)]
    ParseError(#[from] chrono::ParseError),
    #[error("unknown stamp error")]
    Unknown,
}

pub fn serialize<S>(date: &Stamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(date.timestamp() as f64)
}

pub fn serialize_toml<S>(date: &toml::value::Datetime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date);
    serializer.serialize_str(&s)
}

impl<'de> Deserialize<'de> for Stamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StampVisitor;

        impl<'de> Visitor<'de> for StampVisitor {
            type Value = Stamp;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Stamp")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Stamp::from_str(value).map_err(de::Error::custom)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut stamp = None;

                while let Some((key, value)) = map.next_entry()? {
                    match key {
                        "$__toml_private_datetime" => {
                            stamp = Some(Stamp::from_str(value).map_err(de::Error::custom)?);
                        }
                        _ => return Err(de::Error::unknown_field(value, &["date"])),
                    }
                }

                Ok(stamp.expect("Expected a valid IS08601 datetime with optional time component."))
            }
        }

        const FIELDS: &'static [&'static str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, StampVisitor)
    }
}
