//! Reproduction for candidate 133: `dicom-ul` silently truncates proposed presentation context
//! IDs to `u8`, so an SCU that proposes more than 128 presentation contexts puts several
//! *different* contexts on the wire under the *same* context ID.
//!
//! This is a real end-to-end reproduction: a real `dicom-ul` SCP (`ServerAssociationOptions`)
//! and a real `dicom-ul` SCU (`ClientAssociationOptions`) negotiate a genuine association over a
//! real loopback TCP socket. Nothing is hand-authored on the wire; the duplicate IDs reported
//! below are what the SCP actually parsed out of the A-ASSOCIATE-RQ the SCU actually sent.
//!
//! Exit code convention: 0 = bug absent, 1 = bug reproduced.

use std::{
    collections::{HashMap, HashSet},
    net::TcpListener,
    thread,
};

use dicom_ul::{ClientAssociationOptions, ServerAssociationOptions};

const TRANSFER_SYNTAX: &str = "1.2.840.10008.1.2.1";

/// Cap on how many colliding IDs get printed in full, to keep the output readable.
const COLLISIONS_SHOWN: usize = 3;

/// Distinct, syntactically valid private UIDs. The actual values are irrelevant; only their
/// distinctness matters.
fn abstract_syntax(i: usize) -> String {
    format!("1.2.826.0.1.3680043.2.1143.{}", 1000 + i)
}

struct Outcome {
    /// (context id, abstract syntax) as the real SCP parsed them out of the real A-ASSOCIATE-RQ.
    server_view: Vec<(u8, String)>,
    /// (context id, abstract syntax) as the real SCU reports its own negotiated contexts.
    client_view: Vec<(u8, String)>,
}

fn run_association(n: usize) -> Result<Outcome, String> {
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let addr = listener.local_addr().map_err(|e| e.to_string())?;

    let server = thread::spawn(move || -> Result<Vec<(u8, String)>, String> {
        let mut options = ServerAssociationOptions::new()
            .accept_any()
            .ae_title("REPRO-SCP")
            .with_transfer_syntax(TRANSFER_SYNTAX);
        for i in 0..n {
            options = options.with_abstract_syntax(abstract_syntax(i));
        }
        let (socket, _) = listener.accept().map_err(|e| format!("accept: {e}"))?;
        let association = options
            .establish(socket)
            .map_err(|e| format!("SCP establish: {e}"))?;
        let view = association
            .presentation_contexts()
            .iter()
            .map(|pc| (pc.id, pc.abstract_syntax.clone()))
            .collect();
        // keep the association alive briefly so the SCU is never racing a socket teardown
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(view)
    });

    let mut client = ClientAssociationOptions::new()
        .calling_ae_title("REPRO-SCU")
        .called_ae_title("REPRO-SCP");
    for i in 0..n {
        client = client.with_presentation_context(abstract_syntax(i), vec![TRANSFER_SYNTAX.to_string()]);
    }

    let client_result = client.establish(addr);

    match client_result {
        Ok(association) => {
            let client_view = association
                .presentation_contexts()
                .iter()
                .map(|pc| (pc.id, pc.abstract_syntax.clone()))
                .collect();
            let server_view = server.join().map_err(|_| "SCP thread panicked".to_string())??;
            Ok(Outcome {
                server_view,
                client_view,
            })
        }
        Err(e) => {
            let _ = server.join();
            Err(format!("SCU establish: {e}"))
        }
    }
}

/// Returns the list of context IDs that were used by more than one distinct abstract syntax.
fn colliding_ids(view: &[(u8, String)]) -> Vec<(u8, Vec<String>)> {
    let mut by_id: HashMap<u8, Vec<String>> = HashMap::new();
    for (id, syntax) in view {
        let entry = by_id.entry(*id).or_default();
        if !entry.contains(syntax) {
            entry.push(syntax.clone());
        }
    }
    let mut collisions: Vec<_> = by_id
        .into_iter()
        .filter(|(_, syntaxes)| syntaxes.len() > 1)
        .collect();
    collisions.sort_by_key(|(id, _)| *id);
    collisions
}

/// `true` when the scenario demonstrated the bug.
fn check(n: usize, over_limit: bool) -> bool {
    println!("=== proposing {n} presentation contexts (protocol maximum is 128) ===");
    match run_association(n) {
        Err(e) => {
            if over_limit {
                println!("  association was refused: {e}");
                println!("  -> bug ABSENT for this input: no wire-corrupt A-ASSOCIATE-RQ was emitted");
                false
            } else {
                println!("  UNEXPECTED: a legal {n}-context association was refused: {e}");
                println!("  -> control case FAILED");
                true
            }
        }
        Ok(outcome) => {
            let ids: Vec<u8> = outcome.server_view.iter().map(|(id, _)| *id).collect();
            let distinct: HashSet<u8> = ids.iter().copied().collect();
            println!(
                "  association established; SCP received {} contexts using {} distinct context IDs",
                ids.len(),
                distinct.len()
            );

            let collisions = colliding_ids(&outcome.server_view);
            let mut mislabelled = 0usize;
            for (rank, (id, syntaxes)) in collisions.iter().enumerate() {
                let client_entries: Vec<&String> = outcome
                    .client_view
                    .iter()
                    .filter(|(cid, _)| cid == id)
                    .map(|(_, syntax)| syntax)
                    .collect();
                let show = rank < COLLISIONS_SHOWN;
                if show {
                    println!(
                        "  context ID {id} was sent by the SCU for {} different abstract syntaxes:",
                        syntaxes.len()
                    );
                    for syntax in syntaxes {
                        println!("      {syntax}   (as parsed by the real SCP)");
                    }
                    println!(
                        "  ...but the SCU's own negotiated list maps ID {id} to: {}",
                        client_entries
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                for syntax in syntaxes {
                    if !client_entries.iter().any(|c| *c == syntax) {
                        if show {
                            println!("      abstract syntax {syntax} is NOT reported by the SCU at all");
                        }
                        mislabelled += 1;
                    }
                }
            }
            if collisions.len() > COLLISIONS_SHOWN {
                println!(
                    "  ... and {} further colliding context ID(s), omitted",
                    collisions.len() - COLLISIONS_SHOWN
                );
            }

            let proposed: HashSet<String> = (0..n).map(abstract_syntax).collect();
            let negotiated: HashSet<String> =
                outcome.client_view.iter().map(|(_, s)| s.clone()).collect();
            let lost: usize = proposed.difference(&negotiated).count();
            println!("  {lost} of the {n} proposed abstract syntaxes are missing from the SCU's negotiated list");

            if collisions.is_empty() && lost == 0 {
                println!("  -> bug ABSENT for this input");
                false
            } else {
                println!(
                    "  -> REPRODUCED: {} colliding context ID(s), {mislabelled} abstract syntax(es) mislabelled by the SCU",
                    collisions.len()
                );
                true
            }
        }
    }
}

fn main() {
    // control: the largest legal number of presentation contexts must keep working
    let control_broken = check(128, false);
    println!();
    // over-limit inputs: 2*i+1 exceeds 255 from i = 128 onwards
    let mut reproduced = check(129, true);
    println!();
    reproduced |= check(200, true);

    println!();
    if control_broken {
        eprintln!("CONTROL FAILED: the legal 128-context case no longer works");
        std::process::exit(1);
    }
    if reproduced {
        eprintln!("REPRODUCED: dicom-ul emitted an A-ASSOCIATE-RQ with colliding presentation context IDs");
        std::process::exit(1);
    }
    println!("did not reproduce - over-limit proposals are rejected and the legal 128-context case still works");
}
