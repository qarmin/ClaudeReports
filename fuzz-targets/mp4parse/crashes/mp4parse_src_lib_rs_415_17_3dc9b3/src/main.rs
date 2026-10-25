use mp4parse::Error;
use mp4parse::Status;

fn main() {
    let _ = Error::from(Status::Oom);
}
