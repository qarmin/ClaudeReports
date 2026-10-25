#!/usr/bin/env python3
"""Minimal local HTTP server for the hlsdemux PKCS7-unpad repro.

Serves the given directory on 127.0.0.1:<port>, printing the actually-bound
port on stdout (first line) so the caller can pick an ephemeral port with
`port=0` and still know where to connect.

Usage: serve.py <port> <directory>
"""
import http.server
import functools
import sys

port = int(sys.argv[1]) if len(sys.argv) > 1 else 0
directory = sys.argv[2] if len(sys.argv) > 2 else "."

handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=directory)
httpd = http.server.ThreadingHTTPServer(("127.0.0.1", port), handler)
print(httpd.server_address[1], flush=True)
httpd.serve_forever()
