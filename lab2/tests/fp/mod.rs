use lab2::{
    FieldFormat, SERIALIZATION_FORMAT, T,
    fp::{self, Fp},
};

#[test]
fn base10() -> std::io::Result<()> {
    let fp: Fp<19> = Fp::new(10);

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Decimal));
    let serialized = serde_json::to_string(&fp)?;
    const P: T = fp::deser(include_str!("data10.json"), "modulo");
    let fp_from_json: Fp<P> = serde_json::from_str(include_str!("data10.json"))?;

    assert_eq!(serialized, include_str!("data10.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fp_from_json, fp);
    Ok(())
}
#[test]
fn base16() -> std::io::Result<()> {
    let fp: Fp<19> = Fp::new(10);

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Hex));
    let serialized = serde_json::to_string(&fp)?;
    const P: T = fp::deser(include_str!("data16.json"), "modulo");
    let fp_from_json: Fp<P> = serde_json::from_str(include_str!("data16.json"))?;

    assert_eq!(serialized, include_str!("data16.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fp_from_json, fp);
    Ok(())
}
#[test]
fn base64() -> std::io::Result<()> {
    let fp: Fp<19> = Fp::new(10);

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Base64));
    let serialized = serde_json::to_string(&fp)?;
    const P: T = fp::deser(include_str!("data64.json"), "modulo");
    let fp_from_json: Fp<19> = serde_json::from_str(include_str!("data64.json"))?;

    assert_eq!(serialized, include_str!("data64.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fp_from_json, fp);
    Ok(())
}
