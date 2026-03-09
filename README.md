# SIRFZ

**Ephemeral State-Machine Chat Terminal**

Zero-filesystem. Zero-persistence. Memory-locked. Self-destructing. The ultimate peer-to-peer encrypted group chat terminal engineered for aggressive forensic deniability. 

When you launch SIRFZ, you enter a secure shell mimicking a root environment (`[root@sirfz]~#`). The architecture combines a secure hardware-level Rust hypervisor with a highly concurrent zero-copy Go transport mesh.

---

## ⚡ Architecture

SIRFZ dynamically links a Rust hypervisor with a Go shared library at runtime. 

```text
SIRFZ/
├── Makefile             # Unified build system (make all, make run-server, make run-client)
├── transport/           # Go 1.21 — Yamux/mTLS zero-copy multiplexed transport (→ libsirfz.so)
│   ├── clib/            # CGo FFI exports (StartNode, SendMessage, RecvMessage)
│   ├── internal/
│   │   ├── auth/        # in-memory TLS 1.3, Ephemeral Ed25519 identity generation
│   │   ├── transport/   # HostRouter — 64-slot fd-table, pool-backed O(1) Broadcast
│   │   └── tunnel/      # Yamux session setup, sync.Pool zero-copy relay & zeroize
│
└── hypervisor/          # Rust 1.70+ — Security hypervisor & Terminal UI (→ sirfz binary)
    └── src/
        ├── hardening/   # mlockall, prctl, ptrace watchdog, seccomp, namespaces, termios
        ├── ffi/         # libloading dynamic FFI loader for libsirfz.so
        ├── crypto/      # Ed25519/X25519 ephemeral identity, Double-Ratchet AEAD
        ├── chat/        # send_loop, recv_loop, raw terminal event orchestrator
        ├── secrets/     # Secure memory registry, wipe-on-shutdown
        └── shutdown.rs  # Volatile wipe + compiler_fence + munlockall
```

---

## 🛡️ Security Properties

Every security primitive in SIRFZ is designed to ensure that if the machine is seized while running, or inspected after shutdown, **no cryptographic state or message traces exist**.

| Property | Mechanism |
|----------|-----------|
| **Memory confinement** | `mlockall(MCL_CURRENT \| MCL_FUTURE)` prevents keys from hitting swap. |
| **Forensic nullification** | `prctl(PR_SET_DUMPABLE, 0)` + `setrlimit(RLIMIT_CORE, 0)` disables core dumps. |
| **Debugger rejection** | `/proc/self/status` TracerPid check at startup. Exits if attached. |
| **Tracer occupancy** | The watchdog fork intentionally consumes the kernel's sole `ptrace` slot. |
| **Syscall restriction** | Strict `seccomp` allowlist (Trap mode) for networking only. |
| **Namespace isolation** | `unshare(CLONE_NEWUSER \| CLONE_NEWNS)` detaches from host namespace. |
| **Zero persistence** | Ed25519 & X25519 keys generated purely in RAM. No disk I/O. |
| **Encrypted transport** | Double-Ratchet (HKDF-SHA256 + ChaCha20-Poly1305) over Yamux/mTLS 1.3. |
| **Hardware UI input** | Raw `termios` configuration disables OS buffering (`ICANON` + `ECHO` = 0). |
| **Entropic destruction** | `write_volatile(0x00)` + `compiler_fence(SeqCst)` on `SIGINT`/`SIGTERM` or `exit`. |

---

## 🚀 Quick Start Guide

### Prerequisites
- **Go** (for compiling the transport layer)
- **Rust / Cargo** (for compiling the hypervisor)
- **Make** (for using the unified build scripts)

### 1. Build Both Components
From the root structure:
```bash
make all
```
This automatically builds the Go `libsirfz.so` (3.8MB) and the Rust `sirfz` binary (456KB stripped), copying the `.so` next to the binary in `hypervisor/target/release/`.

### 2. Start the Host Node (Server)
The first peer initializes the mesh.
```bash
make run-server
```
You will see the red hacker skull ASCII art, and the server will listen on `0.0.0.0:9000`.

### 3. Connect a Client Node (Peer)
Open a separate terminal on the same network:
```bash
make run-client
```
The terminal connects to `127.0.0.1:9000` (update `Makefile` if connecting remote).

### 4. Engaging Network
Just start typing at the `[root@sirfz]~#` prompt. Incoming messages will safely format in bright green `[PEER]` text above your active line.

### 5. Self-Destruct
Type `exit`, `quit`, or press `Ctrl+C`. The hypervisor intercepts this, zeroes all key arenas, unlocks memory, and safely exits. No logs. No history.

---

## 🚫 Zero-Comment Policy
By design, all codebase logic in SIRFZ is self-documenting through Human-Semantic Naming. No prose, no ghost code.
