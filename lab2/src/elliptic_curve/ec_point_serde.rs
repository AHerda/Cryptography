use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{Visitor, MapAccess}};
use crate::traits::Field;
use super::EcPoint;

impl<T: Field + Serialize> Serialize for EcPoint<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EcPoint::Point { x, y, ec } => {
                use serde::ser::SerializeStruct;
                // We wrap the variant in a tagged structure
                let mut state = serializer.serialize_struct("Point", 3)?;
                state.serialize_field("x", x)?;
                state.serialize_field("y", y)?;
                state.serialize_field("ec", ec)?;
                state.end()
            }
            EcPoint::Infinity => serializer.serialize_str("Infinity"),
        }
    }
}

impl<'de, T: Field + Deserialize<'de>> Deserialize<'de> for EcPoint<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EcPointVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: Field + Deserialize<'de>> Visitor<'de> for EcPointVisitor<T> {
            type Value = EcPoint<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Point or string 'Infinity'")
            }

            // Handle the "Infinity" string case
            fn visit_str<E>(self, value: &str) -> Result<EcPoint<T>, E>
            where
                E: serde::de::Error,
            {
                if value == "Infinity" {
                    Ok(EcPoint::Infinity)
                } else {
                    Err(serde::de::Error::unknown_variant(value, &["Infinity"]))
                }
            }

            // Handle the Point { x, y, ec } case
            fn visit_map<V>(self, mut map: V) -> Result<EcPoint<T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                let mut ec = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        "ec" => ec = Some(map.next_value()?),
                        _ => { let _: serde::de::IgnoredAny = map.next_value()?; }
                    }
                }

                let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                let ec = ec.ok_or_else(|| serde::de::Error::missing_field("ec"))?;

                Ok(EcPoint::Point { x, y, ec })
            }
        }

        deserializer.deserialize_any(EcPointVisitor(std::marker::PhantomData))
    }
}