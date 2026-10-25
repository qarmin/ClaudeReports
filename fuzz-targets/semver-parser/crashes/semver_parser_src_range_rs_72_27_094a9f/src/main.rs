use semver_parser::version::Identifier;

fn main() {
    let identifier = Identifier::Numeric(u64::MAX);
    let string = identifier.to_string();
    let _ = <semver_parser::RangeSet as ::core::str::FromStr>::from_str(&string);
}
