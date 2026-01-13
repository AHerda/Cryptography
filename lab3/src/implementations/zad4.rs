pub trait Schnorr {
    fn encode(r: Self, m: &str) -> String;
}
