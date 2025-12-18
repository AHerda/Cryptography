use super::{Fpk, T};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
};

pub const fn deser(s: &str, key_name: &str) -> T {
    let bytes = s.as_bytes();
    let key = key_name.as_bytes();
    let mut i = 0;
    let mut num: T = 0;
    let mut found = false;

    // Search for "<key_name>":
    while i + key.len() + 3 <= bytes.len() {
        if bytes[i] == b'"' {
            let mut match_key = true;
            let mut j = 0;
            while j < key.len() {
                if bytes[i + 1 + j] != key[j] {
                    match_key = false;
                    break;
                }
                j += 1;
            }
            if match_key && bytes[i + 1 + key.len()] == b'"' && bytes[i + 2 + key.len()] == b':' {
                i += 3 + key.len();
                found = true;
                break;
            }
        }
        i += 1;
    }

    if !found {
        return 0;
    }

    // Skip whitespace
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b':') {
        i += 1;
    }

    while i < bytes.len() {
        let b = bytes[i];
        if b < b'0' || b > b'9' {
            break;
        }
        num = num * 10 + (b - b'0') as T;
        i += 1;
    }
    num
}

impl<const P: T, const K: T> Serialize for Fpk<P, K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Now using 4 fields: P, K, poly, modulo
        let mut state = serializer.serialize_struct("Fpk", 4)?;
        state.serialize_field("P", &P)?;
        state.serialize_field("K", &K)?;
        state.serialize_field("poly", &self.poly)?;
        state.serialize_field("modulo", &self.modulo)?;
        state.end()
    }
}

impl<'de, const P: T, const K: T> Deserialize<'de> for Fpk<P, K> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            P,
            K,
            Poly,
            Modulo,
        }

        struct FpkVisitor<const P: T, const K: T>;

        impl<'de, const P: T, const K: T> Visitor<'de> for FpkVisitor<P, K> {
            type Value = Fpk<P, K>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Fpk")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Fpk<P, K>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut poly = None;
                let mut modulo = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::P => {
                            let val: T = map.next_value()?;
                            if val != P {
                                return Err(serde::de::Error::custom(format!(
                                    "P mismatch: expected {}, got {}",
                                    P, val
                                )));
                            }
                        }
                        Field::K => {
                            let val: T = map.next_value()?;
                            if val != K {
                                return Err(serde::de::Error::custom(format!(
                                    "K mismatch: expected {}, got {}",
                                    K, val
                                )));
                            }
                        }
                        Field::Poly => {
                            if poly.is_some() {
                                return Err(serde::de::Error::duplicate_field("poly"));
                            }
                            poly = Some(map.next_value()?);
                        }
                        Field::Modulo => {
                            if modulo.is_some() {
                                return Err(serde::de::Error::duplicate_field("modulo"));
                            }
                            modulo = Some(map.next_value()?);
                        }
                    }
                }

                let poly = poly.ok_or_else(|| serde::de::Error::missing_field("poly"))?;
                let modulo = modulo.ok_or_else(|| serde::de::Error::missing_field("modulo"))?;

                Ok(Fpk { poly, modulo })
            }
        }

        const FIELDS: &'static [&'static str] = &["P", "K", "poly", "modulo"];
        deserializer.deserialize_struct("Fpk", FIELDS, FpkVisitor::<P, K>)
    }
}
