use symphonia_core::codecs::audio::well_known::CODEC_ID_ALAC;
use symphonia_core::codecs::audio::{AudioCodecParameters, AudioDecoder, AudioDecoderOptions};
use symphonia_core::packet::PacketRef;
use symphonia_core::units::{Duration, Timestamp};
use symphonia_codec_alac::AlacDecoder;

struct BitW { buf: Vec<u8>, acc: u8, n: u32 }
impl BitW {
    fn new() -> Self { BitW { buf: Vec::new(), acc: 0, n: 0 } }
    fn put(&mut self, val: u64, bits: u32) {
        for i in (0..bits).rev() {
            let b = ((val >> i) & 1) as u8;
            self.acc = (self.acc << 1) | b;
            self.n += 1;
            if self.n == 8 { self.buf.push(self.acc); self.acc = 0; self.n = 0; }
        }
    }
    fn finish(mut self) -> Vec<u8> {
        if self.n > 0 { self.acc <<= 8 - self.n; self.buf.push(self.acc); }
        self.buf
    }
}

fn magic_cookie(frame_length: u32, bit_depth: u8, num_channels: u8) -> Vec<u8> {
    let mut c = Vec::new();
    c.extend_from_slice(&frame_length.to_be_bytes());
    c.push(0);
    c.push(bit_depth);
    c.push(40);
    c.push(10);
    c.push(14);
    c.push(num_channels);
    c.extend_from_slice(&255u16.to_be_bytes());
    c.extend_from_slice(&0u32.to_be_bytes());
    c.extend_from_slice(&0u32.to_be_bytes());
    c.extend_from_slice(&44100u32.to_be_bytes());
    assert_eq!(c.len(), 24);
    c
}

fn main() {
    const N: u32 = 4;
    const BIT_DEPTH: u8 = 24;
    let cookie = magic_cookie(N, BIT_DEPTH, 2);

    let mut params = AudioCodecParameters::new();
    params.for_codec(CODEC_ID_ALAC).with_extra_data(cookie.into_boxed_slice());

    let mut dec = AlacDecoder::try_new(&params, &AudioDecoderOptions::default())
        .expect("decoder init");

    let bps = u32::from(BIT_DEPTH) + 1;

    let mut w = BitW::new();
    w.put(1, 3);
    w.put(0, 4);
    w.put(0, 12);
    w.put(0, 1);
    w.put(0, 2);
    w.put(0, 1);
    w.put(0, 8);
    w.put(0x80, 8);
    w.put(0, 16);
    w.put(0, 16);
    for _ in 0..(2 * N) {
        w.put(0x1FF, 9);
        w.put((1u64 << bps) - 1, bps);
    }
    let data = w.finish();

    println!("alac: bit_depth={BIT_DEPTH} frame_length={N} CPE mid_side_weight=-128, packet={} bytes", data.len());
    let pkt = PacketRef::new(0, Timestamp::ZERO, Duration::from(u64::from(N)), &data);
    match dec.decode_ref(&pkt) {
        Ok(_) => println!("decoded ok"),
        Err(e) => println!("decode error: {e}"),
    }
}
