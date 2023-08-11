use std::fmt::Display;

use serde::{
    de::{
        self, value::EnumAccessDeserializer, DeserializeSeed, IntoDeserializer,
        Visitor,
    },
    Deserialize, Deserializer,
};
use thiserror::Error;

pub fn deserialize_command_from_str<'de, T>(
    s: &'de str,
) -> Result<T, DeserializeCommandError>
where
    T: Deserialize<'de>,
{
    let deserializer =
        EnumAccessDeserializer::new(SpaceSeparatedCommandEnumAccess { s });
    T::deserialize(deserializer)
}

#[derive(Debug, Error)]
pub enum DeserializeCommandError {
    #[error("{0}")]
    Custom(String),
    #[error("no space in command")]
    NoSpace,
    #[error("error when deserializing data: {0}")]
    DeserializeDataError(#[from] serde_json::Error),
}

impl de::Error for DeserializeCommandError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DeserializeCommandError::Custom(msg.to_string())
    }
}

struct SpaceSeparatedCommandEnumAccess<'de> {
    s: &'de str,
}

impl<'de> de::EnumAccess<'de> for SpaceSeparatedCommandEnumAccess<'de> {
    type Error = DeserializeCommandError;

    type Variant = SpaceSeparatedCommandVariantAccess<'de>;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // TODO are there any commands that return empty data? There would be no
        // space in the command in that case
        match self.s.split_once(' ') {
            None => Err(DeserializeCommandError::NoSpace),
            Some((command, data)) => {
                let command_deserializer: de::value::StrDeserializer<
                    '_,
                    Self::Error,
                > = command.into_deserializer();
                let value = seed.deserialize(command_deserializer)?;
                Ok((value, SpaceSeparatedCommandVariantAccess { data }))
            }
        }
    }
}

struct SpaceSeparatedCommandVariantAccess<'de> {
    data: &'de str,
}

impl<'de> de::VariantAccess<'de> for SpaceSeparatedCommandVariantAccess<'de> {
    type Error = DeserializeCommandError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(()) // data is completely ignored
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let value = seed
            .deserialize(&mut serde_json::Deserializer::from_str(self.data))?;
        Ok(value)
    }

    fn tuple_variant<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = serde_json::Deserializer::from_str(self.data)
            .deserialize_any(visitor)?;
        Ok(value)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = serde_json::Deserializer::from_str(self.data)
            .deserialize_any(visitor)?;
        Ok(value)
    }
}
