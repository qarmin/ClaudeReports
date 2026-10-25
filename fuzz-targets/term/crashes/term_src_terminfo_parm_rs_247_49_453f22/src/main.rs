use term::terminfo::parm::Param;
use term::terminfo::parm::Variables;

fn main() {
    let mut variables = Variables::new();
    let _ = term::terminfo::parm::expand(&vec![37u8, 105u8], &[Param::default(), Param::Number(i32::MAX)], &mut variables);
}
