use weezl::BitOrder;
use weezl::decode::Configuration;

fn main() {
    let _ = Configuration::with_tiff_size_switch(BitOrder::Lsb, 100u8);
}
