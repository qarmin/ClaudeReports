fn main() {
    <chacha20::variants::Legacy as chacha20::variants::Variant>::set_block_pos(&mut vec![0u32], 0u64);
}
