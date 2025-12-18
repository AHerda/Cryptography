use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use crate::polynomials::Polynomial;

use crate::{T, SERIALIZATION_FORMAT, FieldFormat};

use super::{F2m, Bits8};

impl<const M: T> Serialize for F2m<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut state = serializer.serialize_struct("F2m", 3)?;
        state.serialize_field("M", &M)?;
        state.serialize_field("poly", &PackedPoly(&self.poly))?;
        state.serialize_field("modulo", &PackedPoly(&self.modulo))?;
        state.end()
    }
}

// Helper struct to handle the bit-packing serialization
struct PackedPoly<'a>(&'a Polynomial<Bits8>);

impl<'a> Serialize for PackedPoly<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let format = SERIALIZATION_FORMAT.with(|f| f.get());
        
        // 1. Concatenate Vec<Bits8> into a large byte array
        let bytes: Vec<u8> = self.0.coef.iter().map(|b| b.0).collect();
        
        // 2. Format based on selected base
        match format {
            FieldFormat::Decimal => {
                // For very large polynomials, Decimal requires BigInt logic.
                // Assuming small enough for T (usize) here, or use a string.
                let mut num: u128 = 0; // Use u128 or BigInt for safety
                for (i, &b) in bytes.iter().enumerate() {
                    num |= (b as u128) << (i * 8);
                }
                serializer.serialize_str(&num.to_string())
            },
            FieldFormat::Hex => {
                let mut hex_s = String::from("0x");
                for &b in bytes.iter().rev() { // rev() for big-endian hex representation
                    hex_s.push_str(&format!("{:02x}", b));
                }
                serializer.serialize_str(&hex_s)
            },
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
    where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum Field { M, Poly, Modulo }

        struct F2mVisitor<const M: T>;
        impl<const M: T> F2mVisitor<M> {
            fn parse_packed<'de, V>(&self, map: &mut V) -> Result<Polynomial<Bits8>, V::Error>
            where V: MapAccess<'de> {
                let s: String = map.next_value()?;
                let bytes = if s.starts_with("0x") {
                    // Hex parsing (Big Endian logic)
                    let raw = hex::decode(&s[2..]).map_err(serde::de::Error::custom)?;
                    raw.into_iter().rev().collect()
                } else if s.len() > 10 { // Base64 heuristic
                    use base64::{Engine as _, engine::general_purpose};
                    general_purpose::STANDARD.decode(s).map_err(serde::de::Error::custom)?
                } else {
                    // Decimal parsing (requires BigInt if > 128 bits)
                    let num = s.parse::<u128>().map_err(serde::de::Error::custom)?;
                    num.to_le_bytes().to_vec()
                };

                Ok(Polynomial { 
                    coef: bytes.into_iter().map(Bits8).collect() 
                })
            }
        }
        impl<'de, const M: T> Visitor<'de> for F2mVisitor<M> {
            type Value = F2m<M>;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("struct F2m")
            }

            fn visit_map<V>(self, mut map: V) -> Result<F2m<M>, V::Error>
            where V: MapAccess<'de> {
                let mut poly = None;
                let mut modulo = None;

                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::M => {
                            let val: T = map.next_value()?;
                            if val != M { return Err(serde::de::Error::custom("M mismatch")); }
                        }
                        Field::Poly => poly = Some(self.parse_packed(&mut map)?),
                        Field::Modulo => modulo = Some(self.parse_packed(&mut map)?),
                    }
                }
                
                Ok(F2m { 
                    poly: poly.ok_or_else(|| serde::de::Error::missing_field("poly"))?,
                    modulo: modulo.ok_or_else(|| serde::de::Error::missing_field("modulo"))? 
                })
            }
        }

        deserializer.deserialize_struct("F2m", &["m", "poly", "modulo"], F2mVisitor::<M>)
    }
}