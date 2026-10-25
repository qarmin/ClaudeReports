use semver_parser::version::Version;

fn main() {
    let version = Version { major: 0u64, minor: u64::MAX, patch: 0u64, pre: vec![], build: vec![] };
    let string = version.to_string();
    let _ = <semver_parser::RangeSet as ::core::str::FromStr>::from_str(&string);
}
