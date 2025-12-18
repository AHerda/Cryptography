use super::{Fp, T};
use crate::{FieldFormat, SERIALIZATION_FORMAT};
use serde::ser::SerializeStruct;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
};
use serde_json::Value;

pub const fn deser(s: &str, key_name: &str) -> T {
    let bytes = s.as_bytes();
    let key = key_name.as_bytes();
    let mut i = 0;
    let mut found = false;

    // 1. Find the key "NAME":
    while i + key.len() + 3 <= bytes.len() {
        if bytes[i] == b'"' {
            let mut j = 0;
            let mut match_key = true;
            while j < key.len() {
                if bytes[i + 1 + j] != key[j] {
                    match_key = false;
                    break;
                }
                j += 1;
            }
            if match_key && bytes[i + 1 + key.len()] == b'"' {
                i += key.len() + 2;
                // Look for the colon
                while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b':') {
                    i += 1;
                }
                found = true;
                break;
            }
        }
        i += 1;
    }

    if !found || i >= bytes.len() {
        return 0;
    }

    // 2. Handle potential opening quote for the value
    let mut in_quotes = false;
    if bytes[i] == b'"' {
        in_quotes = true;
        i += 1;
    }

    // 3. Detect Hex prefix
    let mut num: T = 0;
    if i + 2 < bytes.len() && bytes[i] == b'0' && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
        i += 2;
        while i < bytes.len() {
            let b = bytes[i];
            let val = match b {
                b'0'..=b'9' => (b - b'0') as T,
                b'a'..=b'f' => (b - b'a' + 10) as T,
                b'A'..=b'F' => (b - b'A' + 10) as T,
                _ => break,
            };
            num = num * 16 + val;
            i += 1;
        }
    } else {
        // 4. Handle Decimal
        while i < bytes.len() {
            let b = bytes[i];
            if b < b'0' || b > b'9' {
                break;
            }
            num = num * 10 + (b - b'0') as T;
            i += 1;
        }
    }
    num
}

impl<const P: T> Serialize for Fp<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = SERIALIZATION_FORMAT.with(|f| f.get());
        let mut state = serializer.serialize_struct("Fp", 2)?;

        match format {
            FieldFormat::Decimal => {
                state.serialize_field("number", &self.0)?;
                state.serialize_field("modulo", &P)?;
            }
            FieldFormat::Hex => {
                state.serialize_field("number", &format!("0x{:x}", self.0))?;
                state.serialize_field("modulo", &format!("0x{:x}", &P))?;
            }
            FieldFormat::Base64 => {
                use base64::{Engine as _, engine::general_purpose};
                let s = general_purpose::STANDARD.encode(self.0.to_be_bytes());
                state.serialize_field("number", &s)?;
                let s = general_purpose::STANDARD.encode(P.to_be_bytes());
                state.serialize_field("modulo", &s)?;
            }
        };
        state.end()
    }
}

impl<'de, const P: T> Deserialize<'de> for Fp<P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Number,
            Modulo,
        }

        struct FpVisitor<const P: T>;

        impl<'de, const P: T> Visitor<'de> for FpVisitor<P> {
            type Value = Fp<P>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Fp")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Fp<P>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut number = None;
                let mut modulo: Option<T> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Number => {
                            let val = map.next_value::<Value>()?;
                            number = Some(match val {
                                Value::String(s) => {
                                    if s.starts_with("0x") {
                                        T::from_str_radix(&s[2..], 16)
                                            .map_err(serde::de::Error::custom)?
                                    } else if s.len() > 10 {
                                        // Simple heuristic for B64
                                        use base64::{Engine as _, engine::general_purpose};
                                        let bytes = general_purpose::STANDARD
                                            .decode(s)
                                            .map_err(serde::de::Error::custom)?;
                                        // Convert bytes back to usize (assuming T is usize/u64)
                                        let mut arr = [0u8; 8];
                                        arr.copy_from_slice(&bytes[bytes.len() - 8..]);
                                        usize::from_be_bytes(arr)
                                    } else {
                                        s.parse::<T>().map_err(serde::de::Error::custom)?
                                    }
                                }
                                Value::Number(n) => n.as_u64().unwrap_or(0) as T,
                                _ => return Err(serde::de::Error::custom("Invalid number format")),
                            });
                        }
                        Field::Modulo => {
                            if modulo.is_some() {
                                return Err(serde::de::Error::duplicate_field("modulo"));
                            }
                            let m: T = map.next_value()?;
                            // Validation: Ensure the JSON modulo matches the Type modulo
                            if m != P {
                                return Err(serde::de::Error::custom(format!(
                                    "Modulo mismatch: expected {}, got {}",
                                    P, m
                                )));
                            }
                            modulo = Some(m);
                        }
                    }
                }

                let number = number.ok_or_else(|| serde::de::Error::missing_field("number"))?;
                Ok(Fp::new(number))
            }
        }

        const FIELDS: &'static [&'static str] = &["number", "modulo"];
        deserializer.deserialize_struct("Fp", FIELDS, FpVisitor::<P>)
    }
}
