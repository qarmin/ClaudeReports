// Real, standalone repro for candidate 053.
//
// `rustix::fs::inotify::Reader<'buf, Fd: AsFd>` is generic over *any* file
// descriptor that implements `AsFd` - nothing in its type restricts `Fd` to
// one actually returned by `inotify::init()`. Its `next()` method fills its
// buffer with a plain `read()` syscall, then unconditionally casts the start
// of the buffer to `&inotify_event` and advances by
// `size_of::<inotify_event>() + event.len` - with no check that the buffer
// actually holds that many initialized bytes.
//
// This program drives the *real*, unmodified public `Reader::next()` API
// with a real pipe (a completely ordinary `Fd: AsFd` value) and a real
// `read()`/`write()` syscall pair - no hand-faked kernel behavior, no
// fabricated bytes standing in for what a kernel/peer would produce. A pipe
// legitimately returning fewer bytes than were requested is entirely
// ordinary, documented `read(2)` behavior; nothing here relies on bytecode
// unavailable in production or any kernel bug.
//
// Expected (buggy) behavior: `Reader::next()` returns `Ok(Event { .. })`
// built from mostly-uninitialized stack memory - print the fields below and
// note that the `file_name` bytes are literal uninitialized stack garbage
// (varies run to run), not anything derived from the 4 bytes actually
// written to the pipe.
//
// Expected (fixed) behavior: `Reader::next()` returns
// `Err(Errno::INVAL)`, since the short read can't possibly contain a
// complete `inotify_event`.

use rustix::fs::inotify;
use rustix::io::write;
use rustix::pipe::pipe;
use std::mem::MaybeUninit;

fn main() {
    let (read_end, write_end) = pipe().expect("pipe() failed");

    // Write fewer bytes than size_of::<inotify_event>() (16 bytes on this
    // platform: wd:i32, mask:u32, cookie:u32, len:u32). A real inotify fd
    // is documented to never split a struct across a read() - a pipe (or
    // any other non-inotify fd a caller might legitimately pass to
    // `Reader::new`, since the API doesn't forbid it) has no such guarantee.
    let short_data = [0xAAu8; 4];
    write(&write_end, &short_data).expect("write() failed");
    drop(write_end);

    let mut buf = [MaybeUninit::uninit(); 64];
    let mut reader = inotify::Reader::new(read_end, &mut buf);

    match reader.next() {
        Ok(event) => {
            println!("BUG: Reader::next() returned Ok() from a 4-byte short read");
            println!("  wd         = {}", event.wd());
            println!("  events     = {:?}", event.events());
            println!("  cookie     = {}", event.cookie());
            println!("  file_name  = {:?}", event.file_name());
            println!(
                "  (file_name, if present, is uninitialized stack memory read through an \
                 unbounded CStr::from_ptr - compare across repeated runs, it changes every time)"
            );
            std::process::exit(1);
        }
        Err(e) => {
            println!("FIXED: Reader::next() correctly rejected the short read: {e:?}");
        }
    }
}
