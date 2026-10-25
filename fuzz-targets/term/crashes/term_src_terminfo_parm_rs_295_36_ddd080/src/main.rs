use term::terminfo::parm::Variables;

fn main() {
    let mut variables = Variables::new();
    let _ = term::terminfo::parm::expand(&vec![37u8, 112u8, 48u8], &[], &mut variables);
}
