#[inline]
pub(crate) fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &Default::default()
}

pub(crate) mod into_string {
    use serde::{Deserialize, Serialize, Serializer};

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: std::fmt::Display,
    {
        format!("{}", value).serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        D: ::serde::Deserializer<'de>,
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        String::deserialize(deserializer)?
            .parse::<T>()
            .map_err(::serde::de::Error::custom)
    }
}

pub(crate) mod optional_crc32c {
    use serde::{Deserialize, Serialize, Serializer};
    use std::convert::TryInto;

    pub fn serialize<S>(value: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value
            .map(|v| base64::encode(v.to_be_bytes()))
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Option<u32>, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            Some(v) => Ok(Some(u32::from_be_bytes(
                base64::decode(v)
                    .map_err(::serde::de::Error::custom)?
                    .as_slice()
                    .try_into()
                    .map_err(::serde::de::Error::custom)?,
            ))),
            None => Ok(None),
        }
    }
}

pub(crate) mod optional_timestamp {
    use chrono::{DateTime, Utc};
    use prost_types::Timestamp;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::convert::TryInto;

    pub fn serialize<S>(value: &Option<Timestamp>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value
            .as_ref()
            .map(|value| {
                DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp(
                        value.seconds,
                        value.nanos.try_into().unwrap(),
                    ),
                    Utc,
                )
            })
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<DateTime<Utc>>::deserialize(deserializer)?;

        Ok(value.map(|value| Timestamp {
            seconds: value.timestamp(),
            nanos: value.timestamp_subsec_nanos().try_into().unwrap(),
        }))
    }
}

pub(crate) mod optional_field_mask {
    use prost_types::FieldMask;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(_value: &Option<FieldMask>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // FieldMask is serialized as a query parameter, it is not in the body
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<Option<FieldMask>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // FieldMask is serialized as a query parameter, it is not in the body
        Ok(None)
    }
}
