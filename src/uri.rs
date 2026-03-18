use serde::{Deserialize, Serialize, de::Error};
use std::{hash::Hash, ops::Deref, path::Path, str::FromStr};

/// Newtype struct around `fluent_uri::Uri<String>` with serialization implementations that use `as_str()` and 'from_str()' respectively.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uri(fluent_uri::Uri<String>);

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        fluent_uri::Uri::<String>::from_str(&string)
            .map(Uri)
            .map_err(|error| Error::custom(error.to_string()))
    }
}

impl Deref for Uri {
    type Target = fluent_uri::Uri<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Uri {
    type Err = <fluent_uri::Uri<String> as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fluent_uri::Uri::from_str(s).map(Self)
    }
}
