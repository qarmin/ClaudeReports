use semver_parser::Compat;
use semver_parser::RangeSet;
use semver_parser::version::Identifier;

fn main() {
    let identifier = Identifier::Numeric(4294967295u64);
    let string = identifier.to_string();
    let identifier2 = identifier.clone().concat(&string);
    let string2 = identifier2.to_string();
    let _ = RangeSet::parse(&string2, Compat::Npm);
}
