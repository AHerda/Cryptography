use lab2::{
    FieldFormat, SERIALIZATION_FORMAT, T,
    f2m::{self, F2m, bit::Bits8}, polynomials::Polynomial,
    fp,
};

#[test]
fn base10() -> std::io::Result<()> {
    let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b101101), Bits8(0b101101)]);
    let poly: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b1011), Bits8(0b1011)]);
    let f2m: F2m<13> = F2m::new(poly, pk.clone());

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Decimal));
    let serialized = serde_json::to_string(&f2m)?;
    const M: T = fp::deser(include_str!("data10.json"), "M");
    let f2m_from_json: F2m<M> = serde_json::from_str(include_str!("data10.json"))?;

    assert_eq!(serialized, include_str!("data10.json").replace(" ", "").replace("\n", ""));
    assert_eq!(f2m_from_json, f2m);
    Ok(())
}
#[test]
fn base16() -> std::io::Result<()> {
    let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b101101), Bits8(0b101101)]);
    let poly: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b1011), Bits8(0b1011)]);
    let fpk: F2m<13> = F2m::new(poly, pk.clone());

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Hex));
    let serialized = serde_json::to_string(&fpk)?;
    const M: T = fp::deser(include_str!("data16.json"), "M");
    let fpk_from_json: F2m<M> = serde_json::from_str(include_str!("data16.json"))?;

    assert_eq!(serialized, include_str!("data16.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fpk_from_json, fpk);
    Ok(())
}
#[test]
fn base64() -> std::io::Result<()> {
    let pk: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b101101), Bits8(0b101101)]);
    let poly: Polynomial<Bits8> = Polynomial::new(vec![Bits8(0b1011), Bits8(0b1011)]);
    let fpk: F2m<13> = F2m::new(poly, pk.clone());

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Base64));
    let serialized = serde_json::to_string(&fpk)?;
    const M: T = fp::deser(include_str!("data64.json"), "M");
    let fpk_from_json: F2m<M> = serde_json::from_str(include_str!("data64.json"))?;

    assert_eq!(serialized, include_str!("data64.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fpk_from_json, fpk);
    Ok(())
}
