#!/usr/bin/env python3
"""Accept TCP connections on PORT and hex-dump what the client sends."""
import socket, sys, threading
port = int(sys.argv[1]) if len(sys.argv) > 1 else 6900
s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(("127.0.0.1", port)); s.listen(4)
print(f"listening {port}", flush=True)
def h(c, a):
    while True:
        d = c.recv(4096)
        if not d: print(f"{a} closed", flush=True); return
        print(f"{a} <- {len(d)} bytes: {d.hex()}", flush=True)
while True:
    c, a = s.accept(); print(f"connect from {a}", flush=True)
    threading.Thread(target=h, args=(c, a), daemon=True).start()
