#!/usr/bin/env python3
"""
Live reproduction for the movescu-listener-skipped-on-destination-mismatch bug.

Starts a fake "MOVE SCP" that accepts a TCP connection and then does nothing
(never completing a real DICOM association), so `run_move_scu` blocks waiting
for a response instead of exiting immediately. While it's blocked, this script
checks whether movescu's own listener port (used to receive the C-STORE
sub-operations of the C-MOVE) is open.

Usage: check_listener.py <path-to-dicom-movescu-binary>
"""
import socket
import subprocess
import sys
import time

movescu_bin = sys.argv[1]

fake_scp = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
fake_scp.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
fake_scp.bind(("127.0.0.1", 12999))
fake_scp.listen(1)

proc = subprocess.Popen(
    [
        movescu_bin,
        "127.0.0.1:12999",
        "--calling-ae-title", "CALLER",
        "--move-destination", "DEST",  # deliberately different from calling-ae-title
        "-p", "22000",
        "-q", "PatientID=TEST",
        "-o", "/tmp/movescu_repro_out",
    ],
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
)

# accept the connection from movescu's C-MOVE association attempt, then hang
try:
    fake_scp.settimeout(5)
    conn, _addr = fake_scp.accept()
except Exception as e:
    print(f"fake SCP never got a connection: {e}")
    proc.kill()
    sys.exit(2)

time.sleep(1.5)

probe = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
probe.settimeout(1)
try:
    probe.connect(("127.0.0.1", 22000))
    print("PORT 22000 IS OPEN - movescu's listener started correctly")
    result_ok = True
except Exception as e:
    print(f"port 22000 NOT open ({e}) - movescu's listener was skipped (BUG)")
    result_ok = False

proc.kill()
proc.wait()
fake_scp.close()

sys.exit(0 if result_ok else 1)
