//! Reproduction for candidate 137: `ul/src/pdu/writer.rs`'s `write_chunk_u16` computes the
//! 2-byte length field it writes with `data.len() as u16`, then unconditionally writes the FULL,
//! untruncated `data` afterward - for any chunk whose actual content exceeds 65535 bytes, the
//! declared length and the actual bytes on the wire disagree, corrupting the rest of the PDU for
//! any receiver parsing it sequentially.
//!
//! This is reachable through several public, unvalidated call sites (`UserIdentity::new()`'s
//! `primary_field`/`secondary_field`, `UserVariableItem::SopClassExtendedNegotiationSubItem`'s
//! raw `data`), all funneling through the same buggy helper. This repro exercises the most
//! directly reachable one: `ClientAssociationOptions::kerberos_service_ticket()`, a public
//! builder method taking an arbitrary caller-supplied string with no length check anywhere on
//! the way to the wire.
//!
//! Real end-to-end reproduction: a real `dicom-ul` SCP (`ServerAssociationOptions`) and a real
//! `dicom-ul` SCU (`ClientAssociationOptions`) attempt a genuine association over a real loopback
//! TCP socket. Nothing on the wire is hand-authored; the SCU's own real `write_pdu` call produces
//! the corrupted bytes, and the SCP's own real PDU reader is what then fails on them.
//!
//! Exit code convention: 0 = bug absent, 1 = bug reproduced.

use std::net::TcpListener;
use std::thread;

use dicom_ul::{ClientAssociationOptions, ServerAssociationOptions};

const TRANSFER_SYNTAX: &str = "1.2.840.10008.1.2.1";
const ABSTRACT_SYNTAX: &str = "1.2.826.0.1.3680043.2.1143.1.1";

enum Outcome {
    /// Association established successfully.
    Established,
    /// The SCU itself refused to build/send the oversized PDU - a clean, local, upfront error,
    /// before a single byte reached the wire.
    ScuRefusedToSend(String),
    /// Something else failed - the SCU got past sending (meaning it silently wrote a
    /// wire-corrupt PDU) and either it failed later receiving the response, or the SCP itself
    /// choked trying to parse the corrupted bytes.
    CorruptedOnWire(String),
}

fn run_association_checked(ticket_len: usize) -> Outcome {
    match run_association(ticket_len) {
        Ok(()) => Outcome::Established,
        Err(e) if e.contains("SCU result: Err(failed to send pdu)") => {
            Outcome::ScuRefusedToSend(e)
        }
        Err(e) => Outcome::CorruptedOnWire(e),
    }
}

fn run_association(ticket_len: usize) -> Result<(), String> {
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let addr = listener.local_addr().map_err(|e| e.to_string())?;

    let server = thread::spawn(move || -> Result<(), String> {
        let options = ServerAssociationOptions::new()
            .accept_any()
            .ae_title("REPRO-SCP")
            .with_transfer_syntax(TRANSFER_SYNTAX)
            .with_abstract_syntax(ABSTRACT_SYNTAX);
        let (socket, _) = listener.accept().map_err(|e| format!("accept: {e}"))?;
        match options.establish(socket) {
            Ok(_association) => Ok(()),
            Err(e) => Err(format!("SCP establish: {e}")),
        }
    });

    // A caller-supplied Kerberos service ticket larger than a 2-byte length field can express.
    // Real Kerberos tickets carrying a large PAC (many group memberships) can exceed 64 KiB;
    // nothing in this crate's public API rejects an oversized one before it reaches the wire.
    let ticket = "K".repeat(ticket_len);

    let client = ClientAssociationOptions::new()
        .calling_ae_title("REPRO-SCU")
        .called_ae_title("REPRO-SCP")
        .with_presentation_context(ABSTRACT_SYNTAX, vec![TRANSFER_SYNTAX])
        .kerberos_service_ticket(ticket);

    let client_result = client.establish(addr);
    let server_result = server.join().map_err(|_| "SCP thread panicked".to_string())?;

    match (&client_result, &server_result) {
        (Ok(_), Ok(())) => Ok(()),
        _ => Err(format!(
            "SCU result: {}; SCP result: {}",
            client_result
                .as_ref()
                .map(|_| "Ok".to_string())
                .unwrap_or_else(|e| format!("Err({e})")),
            server_result
                .as_ref()
                .map(|_| "Ok".to_string())
                .unwrap_or_else(|e| format!("Err({e})"))
        )),
    }
}

fn main() {
    let mut ok = true;

    println!("=== control: Kerberos service ticket well under the 65535-byte limit ===");
    match run_association(1000) {
        Ok(()) => println!("  association established normally, as expected"),
        Err(e) => {
            println!("  UNEXPECTED: a legal small ticket broke the association: {e}");
            ok = false;
        }
    }

    println!();
    println!("=== a Kerberos service ticket larger than a u16 can express (70000 bytes) ===");
    match run_association_checked(70_000) {
        Outcome::Established => {
            println!(
                "  UNEXPECTED: association established anyway - the truncated-length corruption \
                 did not surface as a failure this run"
            );
            ok = false;
        }
        Outcome::ScuRefusedToSend(e) => {
            println!("  association failed: {e}");
            println!(
                "  -> bug ABSENT: the SCU itself refused to build the oversized chunk before \
                 writing anything to the wire (a clean, local error - see the fix)"
            );
        }
        Outcome::CorruptedOnWire(e) => {
            println!("  association failed: {e}");
            println!(
                "  -> BUG: the SCU got past writing/sending the PDU (meaning it silently emitted \
                 a wire-corrupt A-ASSOCIATE-RQ - declared length {} instead of the real {} bytes \
                 for the secondary field) and the failure above is something *downstream* \
                 choking on the corrupted bytes instead of a clean upfront refusal; a real DICOM \
                 SCP that isn't this crate's own reader could behave far worse than a failed parse",
                70_000usize % 65536,
                70_000
            );
            ok = false;
        }
    }

    println!();
    if ok {
        println!("RESULT: no wire corruption observed");
        std::process::exit(0);
    } else {
        println!("RESULT: bug reproduced or control case failed (see above)");
        std::process::exit(1);
    }
}
