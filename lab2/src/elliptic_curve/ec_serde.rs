use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
};

use super::Ec;
use crate::traits::Field;

impl<T: Field + Serialize> Serialize for Ec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Ec", 2)?;
        state.serialize_field("a", &self.a)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

impl<'de, T: Field + Deserialize<'de>> Deserialize<'de> for Ec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum FieldKey {
            A,
            B,
        }

        struct EcVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: Field + Deserialize<'de>> Visitor<'de> for EcVisitor<T> {
            type Value = Ec<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Ec with fields 'a' and 'b'")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Ec<T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut a = None;
                let mut b = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        FieldKey::A => {
                            if a.is_some() {
                                return Err(serde::de::Error::duplicate_field("a"));
                            }
                            a = Some(map.next_value()?);
                        }
                        FieldKey::B => {
                            if b.is_some() {
                                return Err(serde::de::Error::duplicate_field("b"));
                            }
                            b = Some(map.next_value()?);
                        }
                    }
                }

                let a = a.ok_or_else(|| serde::de::Error::missing_field("a"))?;
                let b = b.ok_or_else(|| serde::de::Error::missing_field("b"))?;

                Ok(Ec { a, b })
            }
        }

        const FIELDS: &'static [&'static str] = &["a", "b"];
        deserializer.deserialize_struct("Ec", FIELDS, EcVisitor(std::marker::PhantomData))
    }
}
