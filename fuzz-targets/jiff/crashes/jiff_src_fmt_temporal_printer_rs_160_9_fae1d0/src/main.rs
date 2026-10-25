use jiff::fmt::temporal::DateTimePrinter;

fn main() {
    let date_time_printer = DateTimePrinter::new();
    let _ = date_time_printer.separator(255u8);
}
