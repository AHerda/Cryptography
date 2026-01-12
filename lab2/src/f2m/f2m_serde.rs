use crate::polynomials::Polynomial;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

use crate::{FieldFormat, SERIALIZATION_FORMAT, T};

use super::{Bits8, F2m};

pub const fn deser(s: &str, key_name: &str) -> T {
    let bytes = s.as_bytes();
    let key = key_name.as_bytes();
    let mut i = 0;
    let mut found = false;

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

    let mut in_quotes = false;
    if bytes[i] == b'"' {
        in_quotes = true;
        i += 1;
    }

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

impl<const M: T> Serialize for F2m<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("F2m", 3)?;
        state.serialize_field("M", &M)?;
        state.serialize_field("Poly", &PackedPoly(&self.poly))?;
        state.serialize_field("Modulo", &PackedPoly(&self.modulo))?;
        state.end()
    }
}

struct PackedPoly<'a>(&'a Polynomial<Bits8>);

impl<'a> Serialize for PackedPoly<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = SERIALIZATION_FORMAT.with(|f| f.get());

        let bytes: Vec<u8> = self.0.coef.iter().map(|b| b.0).collect();

        match format {
            FieldFormat::Decimal => {
                let mut num: u128 = 0; // Use u128 or BigInt for safety
                for (i, &b) in bytes.iter().enumerate() {
                    num |= (b as u128) << (i * 8);
                }
                serializer.serialize_str(&num.to_string())
            }
            FieldFormat::Hex => {
                let mut hex_s = String::from("0x");
                for &b in bytes.iter().rev() {
                    hex_s.push_str(&format!("{:02x}", b));
                }
                serializer.serialize_str(&hex_s)
            }
            FieldFormat::Base64 => {
                use base64::{Engine as _, engine::general_purpose};
                let s = general_purpose::STANDARD.encode(&bytes);
                serializer.serialize_str(&s)
            }
        }
    }
}

impl<'de, const M: T> Deserialize<'de> for F2m<M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        enum Field {
            M,
            Poly,
            Modulo,
        }

        struct F2mVisitor<const M: T>;
        impl<const M: T> F2mVisitor<M> {
            fn parse_packed<'de, V>(&self, map: &mut V) -> Result<Polynomial<Bits8>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let s: String = map.next_value()?;
                let bytes = if s.starts_with("0x") {
                    let raw = hex::decode(&s[2..]).map_err(serde::de::Error::custom)?;
                    raw.into_iter().rev().collect()
                } else if s.ends_with('=') {
                    use base64::{Engine as _, engine::general_purpose};
                    general_purpose::STANDARD
                        .decode(s)
                        .map_err(serde::de::Error::custom)?.to_vec()
                } else {
                    let num = s.parse::<u128>().map_err(serde::de::Error::custom)?;
                    num.to_le_bytes().to_vec()
                };

                Ok(Polynomial::new(bytes.into_iter().map(Bits8).collect()))
            }
        }
        impl<'de, const M: T> Visitor<'de> for F2mVisitor<M> {
            type Value = F2m<M>;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("struct F2m")
            }

            fn visit_map<V>(self, mut map: V) -> Result<F2m<M>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut poly = None;
                let mut modulo = None;

                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::M => {
                            let val: T = map.next_value()?;
                            if val != M {
                                return Err(serde::de::Error::custom("M mismatch"));
                            }
                        }
                        Field::Poly => poly = Some(self.parse_packed(&mut map)?),
                        Field::Modulo => modulo = Some(self.parse_packed(&mut map)?),
                    }
                }

                Ok(F2m {
                    poly: poly.ok_or_else(|| serde::de::Error::missing_field("poly"))?,
                    modulo: modulo.ok_or_else(|| serde::de::Error::missing_field("modulo"))?,
                })
            }
        }

        deserializer.deserialize_struct("F2m", &["M", "poly", "modulo"], F2mVisitor::<M>)
    }
}
