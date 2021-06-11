//! Collection of all structs wrapping an ID.

use std::fmt::{Formatter, Result as FmtResult};

use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;
use serde::de::Error;

/// ID of a tweet object
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TweetId(pub u64);

/// ID of an user
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UserId(pub u64);

struct IdVisitor;

impl<'de> Visitor<'de> for IdVisitor {
    type Value = u64;
    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "Expected a string that can be parsed to u64")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse::<u64>().map_err(E::custom)
    }
}

macro_rules! id_impls {
    ($($name:ty)+) => {
        $(
        impl $name {
            /// Join ID's into one `sep` separated string
            pub fn join(slice: &[Self], sep: &str) -> String {
                let mut joined = String::new();
                let mut first = true;
                for id in slice {
                    if !first {
                        joined.push_str(sep);
                    }
                    joined.push_str(&id.0.to_string());
                    first = false;
                }
                joined
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let id = deserializer.deserialize_str(IdVisitor)?;
                Ok(Self(id))
            }
        }
        )+
    };
}

id_impls!(TweetId UserId);
