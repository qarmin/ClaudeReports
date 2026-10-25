use term::terminfo::parm::Param;
use term::terminfo::parm::Variables;

fn main() {
    let mut variables = Variables::default();
    let _ = term::terminfo::parm::expand(&include_bytes!("data_0.bin").to_vec(), &[Param::Number(i32::MAX)], &mut variables);
}
