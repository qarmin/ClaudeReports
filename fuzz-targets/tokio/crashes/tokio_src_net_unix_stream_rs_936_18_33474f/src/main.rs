use tokio::net::UnixStream;

fn main() {
    let _ = UnixStream::pair();
}
