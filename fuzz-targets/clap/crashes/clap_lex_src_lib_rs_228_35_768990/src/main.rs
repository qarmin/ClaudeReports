use clap_lex::RawArgs;

fn main() {
    let raw_args = RawArgs::default();
    let raw_args2 = RawArgs::default();
    let mut arg_cursor = raw_args2.cursor();
    let _ = raw_args.next(&mut arg_cursor);
    let _ = raw_args2.remaining(&mut arg_cursor);
}
