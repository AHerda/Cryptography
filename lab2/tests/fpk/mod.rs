use lab2::{
    FieldFormat, SERIALIZATION_FORMAT, T,
    fpk::{self, Fpk}, polynomials::Polynomial,
    fp::Fp,
};

#[test]
fn base10() -> std::io::Result<()> {
    let pk: Polynomial<Fp<19>> = Polynomial::new(vec![Fp::new(1), Fp::new(2), Fp::new(3), Fp::new(1), Fp::new(2), Fp::new(3)]);
    let poly: Polynomial<Fp<19>> = Polynomial::new(vec![Fp::new(15), Fp::new(9), Fp::new(8), Fp::new(13), Fp::new(5)]);
    let fpk: Fpk<19, 5> = Fpk::new(poly, pk.clone());

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Decimal));
    let serialized = serde_json::to_string(&fpk)?;
    const P: T = fpk::deser(include_str!("data10.json"), "P");
    const K: T = fpk::deser(include_str!("data10.json"), "K");
    let fpk_from_json: Fpk<P, K> = serde_json::from_str(include_str!("data10.json"))?;

    assert_eq!(serialized, include_str!("data10.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fpk_from_json, fpk);
    Ok(())
}
#[test]
fn base16() -> std::io::Result<()> {
    let pk: Polynomial<Fp<19>> = Polynomial::new(vec![Fp::new(1), Fp::new(2), Fp::new(3), Fp::new(1), Fp::new(2), Fp::new(3)]);
    let poly: Polynomial<Fp<19>> = Polynomial::new(vec![Fp::new(15), Fp::new(9), Fp::new(8), Fp::new(13), Fp::new(5)]);
    let fpk: Fpk<19, 5> = Fpk::new(poly, pk.clone());

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Hex));
    let serialized = serde_json::to_string(&fpk)?;
    const P: T = fpk::deser(include_str!("data16.json"), "P");
    const K: T = fpk::deser(include_str!("data16.json"), "K");
    let fpk_from_json: Fpk<P, K> = serde_json::from_str(include_str!("data16.json"))?;

    assert_eq!(serialized, include_str!("data16.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fpk_from_json, fpk);
    Ok(())
}
#[test]
fn base64() -> std::io::Result<()> {
    let pk: Polynomial<Fp<19>> = Polynomial::new(vec![Fp::new(1), Fp::new(2), Fp::new(3), Fp::new(1), Fp::new(2), Fp::new(3)]);
    let poly: Polynomial<Fp<19>> = Polynomial::new(vec![Fp::new(15), Fp::new(9), Fp::new(8), Fp::new(13), Fp::new(5)]);
    let fpk: Fpk<19, 5> = Fpk::new(poly, pk.clone());

    SERIALIZATION_FORMAT.with(|f| f.set(FieldFormat::Base64));
    let serialized = serde_json::to_string(&fpk)?;
    const P: T = fpk::deser(include_str!("data64.json"), "P");
    const K: T = fpk::deser(include_str!("data64.json"), "K");
    let fpk_from_json: Fpk<P, K> = serde_json::from_str(include_str!("data64.json"))?;

    assert_eq!(serialized, include_str!("data64.json").replace(" ", "").replace("\n", ""));
    assert_eq!(fpk_from_json, fpk);
    Ok(())
}
