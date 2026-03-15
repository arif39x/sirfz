# sirfz

![Logo Description](asset/Logo.jpeg)

Ephemeral State-Machine Chat Terminal

sirfz is a peer-to-peer, encrypted group chat terminal engineered for absolute forensic deniability. It operates entirely without a filesystem footprint, maintaining zero persistent state.

## What it is

sirfz is not a normal chat application. It is a highly specialized piece of software that forcibly links a low-level Rust hardware security hypervisor with a highly concurrent, zero-copy Go transport mesh. It runs directly in your terminal, providing a secure shell environment (mimicking a root prompt) where operators can exchange messages that exist only in locked RAM.

## What it can do

- Establish a bidirectional, multiplexed peer-to-peer communication mesh.
- Forward and broadcast text messages to all connected peers with near-zero latency.
- Provide end-to-end encryption using a Double-Ratchet mechanism.
- Self-destruct instantaneously upon exit, intercepting standard termination signals to wipe all cryptographic material before the OS regains control.

## How it does it

The system boots in three rigorous stages:

1. **Host Environment Isolation**: The Rust hypervisor leverages Linux kernel primitives to harden the process. It drops the process into a new user and mount namespace, disables core dumps via `prctl`, and actively monitors `/proc/self/status` to ensure no foreign debuggers are attached. To prevent the OS from ever writing keys to the swap file, it explicitly locks the process memory using `mlockall`.
2. **Ephemeral Identity Generation**: The Go shared library generates Ed25519 and X25519 keypairs purely in RAM. It never writes to disk. It uses these keys to establish a mutual TLS 1.3 (mTLS) session over a custom Yamux multiplexer.
3. **Hardware Input Interception**: The terminal UI bypasses standard OS line-buffering routines. It forces the terminal into raw mode (`termios` modifications), ensuring keystrokes flow directly from hardware interrupt into the locked memory arena, bypassing standard shell history or caching mechanisms.

Messages transmitted across the wire are encrypted via ChaCha20-Poly1305, with keys continuously rolling forward via a HKDF-SHA256 Key Derivation Function (KDF) chain.

## Why this is a better approach for anonymity

Traditional secure messengers (like Signal, Session, or Matrix) solve transmission security but fail at endpoint security. They rely on local databases, persistent configuration files, and operating systems that freely page application memory to disk. If an adversary seizes the physical hardware while those applications are running or after they have closed, forensic analysis of the hard drive or swap partition can routinely yield encryption keys, contact lists, and message histories.

SIRFZ assumes the host operating system is hostile. By never touching the disk—not for caching, not for configuration, not for identity generation—it eliminates the endpoint footprint entirely. It operates as a mathematical state machine that exists only so long as power flows through the RAM modules.

## How it protects your anonymity

Anonymity in SIRFZ is not achieved through routing (like Tor or I2P), but through absolute state deniability.

- **No Identities**: You do not have a username, an account, or a persistent phone number. Your identity is a mathematical keypair generated milliseconds after you launch the binary.
- **No History**: Because there is no database, there is no history. The moment a message scrolls off your screen or you exit the application, it ceases to exist.
- **Entropic Destruction**: When you type `exit` or press `Ctrl+C`, the hypervisor intercepts the command. Before allowing the process to terminate, it executes a volatile memory wipe (writing `0x00`) over all encryption keys and enforces a compiler fence to prevent optimization reordering. Only after the memory is mathematically scrubbed does it unlock the RAM and exit.

If the machine is seized, unplugged, or inspected, there is nothing to find.

## Quick Start Guide

### Prerequisites

- Go 1.21+
- Rust / Cargo 1.70+
- Make

### Building

```bash
make all
```

### Running the Host (Server)

```bash
make run-server
```

The application will listen on `0.0.0.0:9000`

### Running the Peer (Client)

Open a separate terminal and run:

```bash
make run-client
```

The peer connects to `127.0.0.1:9000`. Once connected, type at the prompt to broadcast encrypted messages to the mesh. Type `exit` to trigger the secure shutdown sequence.
