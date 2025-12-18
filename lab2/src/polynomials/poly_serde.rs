use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct, de::{Visitor, MapAccess}};
use super::{Polynomial};
use crate::T;

// impl<T> Serialize for Polynomial<T>
// where
//     T: Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // Define a struct with 1 field: "coeffs"
//         let mut state = serializer.serialize_struct("Polynomial", 1)?;
//         state.serialize_field("coeffs", &self.coef)?;
//         state.end()
//     }
// }

// impl<'de, T> Deserialize<'de> for Polynomial<T>
// where
//     T: Deserialize<'de>,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         #[serde(field_identifier, rename_all = "lowercase")]
//         enum Field { Coeffs }

//         struct PolyVisitor<T>(std::marker::PhantomData<T>);

//         impl<'de, T> Visitor<'de> for PolyVisitor<T>
//         where
//             T: Deserialize<'de>,
//         {
//             type Value = Polynomial<T>;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("struct Polynomial with a 'coeffs' field")
//             }

//             fn visit_map<V>(self, mut map: V) -> Result<Polynomial<T>, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut coeffs = None;

//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         Field::Coeffs => {
//                             if coeffs.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("coeffs"));
//                             }
//                             coeffs = Some(map.next_value()?);
//                         }
//                     }
//                 }

//                 let coef = coeffs.ok_or_else(|| serde::de::Error::missing_field("coeffs"))?;
//                 Ok(Polynomial { coef })
//             }
//         }

//         const FIELDS: &'static [&'static str] = &["coeffs"];
//         deserializer.deserialize_struct("Polynomial", FIELDS, PolyVisitor(std::marker::PhantomData))
//     }
// }

impl<T: Serialize> Serialize for Polynomial<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        // Polynomial just wraps the vector. 
        // When self.coef.serialize is called, it triggers the Fp logic for each element.
        let mut state = serializer.serialize_struct("Polynomial", 1)?;
        state.serialize_field("coeffs", &self.coef)?;
        state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Polynomial<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        #[derive(Deserialize)]
        struct RawPoly<T> { coeffs: Vec<T> }
        
        let raw = RawPoly::<T>::deserialize(deserializer)?;
        Ok(Polynomial { coef: raw.coeffs })
    }
}