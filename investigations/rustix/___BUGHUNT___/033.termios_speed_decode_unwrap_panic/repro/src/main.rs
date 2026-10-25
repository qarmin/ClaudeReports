// Standalone repro for rustix candidate 033 (termios_speed_decode_unwrap_panic).
//
// rustix's `Termios::input_speed()`/`output_speed()` (src/termios/types.rs, rustix 1.1.5 /
// commit 287214b889865d8e1406a0ee71cc409b6f6191c8) call `speed::decode(<raw encoded speed_t
// read back from the real system via cfgetispeed/cfgetospeed or the raw c_ispeed/c_ospeed
// field>).unwrap()` on four platform groups: illumos/Solaris ("solarish"), newlib, AIX (lines
// 109/142) and every other non-Linux/non-BSD platform rustix supports - Haiku, GNU/Hurd, QNX
// Neutrino, Redox, Cygwin, etc. (lines 121/154). `speed::decode` (src/termios/types.rs:844) is a
// `match` over the hardcoded `B0..=B4000000` constants that *rustix itself* defines, returning
// `None` for anything else.
//
// HARDWARE/OS-AVAILABILITY EXCEPTION (CONFIRMATION.md section 11): this bug's "second participant"
// is each target OS's own libc/kernel, which legitimately reports a raw encoded speed_t value for
// a perfectly ordinary, standard baud rate that OS itself defines - not adversarial/fabricated
// input. Demonstrating it end-to-end genuinely requires booting that OS (illumos/Solaris, Haiku,
// GNU/Hurd, AIX, QNX, ...) and calling the real tcgetattr/cfgetispeed there; none of those have a
// rustc target + working cross-linker or a bootable system-emulation image available in this
// sandbox (x86_64 Linux only - verified: no illumos/Haiku/Hurd/AIX/QNX rustup target, no
// qemu-system image for any of them, no cross gcc/binutils package for any of them). This mirrors
// candidate 028's own "no amount of qemu-user emulates a different OS's kernel" limitation, which
// that candidate's RESULT.md explicitly flagged as the same kind of exception.
//
// What this repro actually does instead, in the same spirit as candidate 028's own repro: it
// copies rustix's real, unmodified `speed::decode` match arms verbatim (byte-for-byte, only the
// `#[cfg(...)]` platform gates are pre-resolved for a fixed target_os, exactly as rustc's own
// `cfg` evaluation would resolve them when actually compiled for that target_os - nothing in the
// match logic itself is altered, reordered, or reimplemented), and feeds it the real numeric
// `speed_t` encoding of a real, standard POSIX baud-rate constant that target OS's own `libc` crate
// defines - copied verbatim from the vendored `libc-0.2.189` source, cited by exact file+line below
// - not a hand-invented "exotic" value. The only thing faked is *which OS this process is compiled
// for*; the decode logic and the input value are both the real, independently-sourced things.

/// Verbatim copy of `rustix::termios::types::speed::decode`'s match arms
/// (src/termios/types.rs:844-973), with every `#[cfg(...)]` gate pre-resolved for
/// `target_os = "illumos"` (a `solarish` platform) by hand, citing the same source line numbers.
/// Every arm's cfg here evaluates to the same answer rustc itself would compute for a real
/// `illumos` compilation: `target_os = "aix"` / `"nto"` / `"haiku"` / `"solaris"` / `apple` / `bsd`
/// / `sparc(64)` / `"cygwin"` are all false for illumos; `solarish` is true.
const fn decode_resolved_for_illumos(encoded_speed: u32) -> Option<u32> {
    // Real illumos `B*` constants, copied verbatim from the vendored
    // libc-0.2.189 crate's `src/unix/solarish/mod.rs:1933-1956` (shared by illumos+Solaris
    // ("solarish") - the "solarish" cfg rustix itself gates on, build.rs:149-151).
    const B0: u32 = 0;
    const B50: u32 = 1;
    const B75: u32 = 2;
    const B110: u32 = 3;
    const B134: u32 = 4;
    const B150: u32 = 5;
    const B200: u32 = 6;
    const B300: u32 = 7;
    const B600: u32 = 8;
    const B1200: u32 = 9;
    const B1800: u32 = 10;
    const B2400: u32 = 11;
    const B4800: u32 = 12;
    const B9600: u32 = 13;
    const B19200: u32 = 14;
    const B38400: u32 = 15;
    const B57600: u32 = 16;
    // libc's real B76800 = 17 (src/unix/solarish/mod.rs:1950) - rustix has NO `speed::B76800`
    // constant anywhere in src/termios/types.rs, on any platform, so there can never be a
    // `c::B76800 => ...` match arm for it to hit.
    const B115200: u32 = 18;
    const B230400: u32 = 20;
    const B460800: u32 = 22;
    const B921600: u32 = 23;

    match encoded_speed {
        B0 => Some(0),
        B50 => Some(50),
        B75 => Some(75),
        B110 => Some(110),
        B134 => Some(134),
        B150 => Some(150),
        B200 => Some(200),
        B300 => Some(300),
        B600 => Some(600),
        B1200 => Some(1200),
        B1800 => Some(1800),
        B2400 => Some(2400),
        B4800 => Some(4800),
        B9600 => Some(9600),
        B19200 => Some(19200),
        B38400 => Some(38400),
        B57600 => Some(57600),       // line 863: cfg(not(aix)) -> true on illumos
        B115200 => Some(115_200),    // line 865: cfg(not(aix)) -> true on illumos
        B230400 => Some(230_400),    // line 867: cfg(not(any(aix,nto))) -> true on illumos
        B460800 => Some(460_800),    // line 876: cfg(not(any(apple,aix,dragonfly,haiku,nto,openbsd))) -> true on illumos
        B921600 => Some(921_600),    // line 901: cfg(not(any(apple,aix,dragonfly,haiku,nto,openbsd))) -> true on illumos
        // (B500000/B576000/B1000000-upward arms are all `cfg(not(any(... solarish ...)))` or
        // `cfg(not(any(... solaris ...)))`-excluded or otherwise don't apply; omitted here exactly
        // as they would be stripped by rustc's own cfg evaluation for a real illumos build)
        _ => None,
    }
}

/// Verbatim copy of the same match arms, pre-resolved for `target_os = "haiku"` (one of the
/// "every other platform" group - not `solarish`/`newlib`/`aix`, so this is the
/// `speed::decode(self.input_speed).unwrap()` path at src/termios/types.rs:121/154).
const fn decode_resolved_for_haiku(encoded_speed: u32) -> Option<u32> {
    // Real Haiku `B*` constants, copied verbatim from the vendored libc-0.2.189 crate's
    // `src/unix/haiku/mod.rs:1294-1313`.
    const B0: u32 = 0x00;
    const B50: u32 = 0x01;
    const B75: u32 = 0x02;
    const B110: u32 = 0x03;
    const B134: u32 = 0x04;
    const B150: u32 = 0x05;
    const B200: u32 = 0x06;
    const B300: u32 = 0x07;
    const B600: u32 = 0x08;
    const B1200: u32 = 0x09;
    const B1800: u32 = 0x0A;
    const B2400: u32 = 0x0B;
    const B4800: u32 = 0x0C;
    const B9600: u32 = 0x0D;
    const B19200: u32 = 0x0E;
    const B38400: u32 = 0x0F;
    const B57600: u32 = 0x10;
    const B115200: u32 = 0x11;
    const B230400: u32 = 0x12;
    // libc's real B31250 = 0x13 (src/unix/haiku/mod.rs:1313) - the standard MIDI serial baud
    // rate (31250 bps). rustix has NO `speed::B31250` constant anywhere, on any platform, so
    // there can never be a `c::B31250 => ...` arm to hit.

    match encoded_speed {
        B0 => Some(0),
        B50 => Some(50),
        B75 => Some(75),
        B110 => Some(110),
        B134 => Some(134),
        B150 => Some(150),
        B200 => Some(200),
        B300 => Some(300),
        B600 => Some(600),
        B1200 => Some(1200),
        B1800 => Some(1800),
        B2400 => Some(2400),
        B4800 => Some(4800),
        B9600 => Some(9600),
        B19200 => Some(19200),
        B38400 => Some(38400),
        B57600 => Some(57600),     // line 863: cfg(not(aix)) -> true on haiku
        B115200 => Some(115_200),  // line 865: cfg(not(aix)) -> true on haiku
        B230400 => Some(230_400),  // line 867: cfg(not(any(aix,nto))) -> true on haiku
        // (every arm from B460800 upward is `cfg(...target_os = "haiku"...)`-excluded, so none of
        // them would even exist in a real haiku build; omitted here the same way)
        _ => None,
    }
}

// Exactly reproduces `speed::decode(<raw c::speed_t>).unwrap()`
// (src/termios/types.rs:109/121/142/154) - the real, unmodified call pattern.
fn input_speed_illumos(raw_cfgetispeed_result: u32) -> u32 {
    decode_resolved_for_illumos(raw_cfgetispeed_result).unwrap()
}

fn input_speed_haiku(raw_c_ispeed_field: u32) -> u32 {
    decode_resolved_for_haiku(raw_c_ispeed_field).unwrap()
}

fn main() {
    // Case 1: illumos/Solaris, B76800 - a real tty configured to 76800 baud (a standard rate a
    // real illumos serial driver, e.g. a USB-UART adapter, can report via a real `cfsetispeed`/
    // `cfsetospeed` call) genuinely sets c_cflag's speed bits to libc's real `B76800` encoding
    // (17). `Termios::input_speed()`'s real call `speed::decode(c::cfgetispeed(...)).unwrap()`
    // then runs on that real encoded value.
    println!("case 1: illumos/Solaris real B76800 (encoded = 17)");
    let result1 = std::panic::catch_unwind(|| input_speed_illumos(17));
    let case1_reproduced = result1.is_err();
    match result1 {
        Ok(v) => println!("  did not reproduce - decoded to {v}"),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            println!("  REPRODUCED - panicked: {msg}");
        }
    }

    // Case 2: Haiku, B31250 - the standard MIDI serial baud rate (31250 bps), a real,
    // completely ordinary configuration for a real Haiku serial/MIDI device, encoded as libc's
    // real `B31250` (0x13 = 19).
    println!("case 2: Haiku real B31250 (encoded = 19, standard MIDI baud rate)");
    let result2 = std::panic::catch_unwind(|| input_speed_haiku(19));
    let case2_reproduced = result2.is_err();
    match result2 {
        Ok(v) => println!("  did not reproduce - decoded to {v}"),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            println!("  REPRODUCED - panicked: {msg}");
        }
    }

    if case1_reproduced || case2_reproduced {
        std::process::exit(1);
    }
}
