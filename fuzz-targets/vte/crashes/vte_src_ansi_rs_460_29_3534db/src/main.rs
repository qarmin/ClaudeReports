use vte::ansi::StdSyncHandler;

fn main() {
    let mut std_sync_handler = StdSyncHandler::default();
    <vte::ansi::StdSyncHandler as vte::ansi::Timeout>::set_timeout(&mut std_sync_handler, std::time::Duration::new(10000000000000000000u64, 33u32 % 1_000_000_000));
}
