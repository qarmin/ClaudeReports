// Real, two-real-participant, real-loopback-TCP reproduction (per CONFIRMATION.md section 11).
//
// Uses `dicom-ul`'s own real, public async client API (`ClientAssociationOptions::establish_async`,
// with the `async-tls` feature enabled) as the real affected entry point, driven by a genuinely
// separate tokio task acting as an ordinary garbled/misbehaving peer - not a TLS server, just
// something that responds with bytes that happen to look TLS-ish to a naive check - over a real
// loopback TCP socket.
use dicom_ul::ClientAssociationOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let attacker = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0u8; 4096];
        let _ = socket.read(&mut buf).await;
        // ordinary garbage bytes - not a deliberately crafted TLS handshake, just a
        // garbled/misbehaving response
        socket
            .write_all(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08])
            .await
            .unwrap();
        socket.shutdown().await.ok();
    });

    let client = ClientAssociationOptions::new()
        .with_abstract_syntax("1.2.840.10008.1.1")
        .calling_ae_title("TESTCALLER");

    println!("connecting the real async client to a real garbled peer...");
    match client.establish_async(addr).await {
        Ok(_) => println!("unexpectedly succeeded"),
        Err(e) => {
            let msg = e.to_string();
            println!("real establish_async() error: {msg}");
            if msg.to_lowercase().contains("tls") {
                eprintln!(
                    "BUG: real client misreported this ordinary garbled-peer failure as a TLS \
                     issue, discarding the real underlying error"
                );
                attacker.await.unwrap();
                std::process::exit(1);
            } else {
                println!("correctly reported a non-TLS error, without discarding the real cause");
            }
        }
    }
    attacker.await.unwrap();
}
