# RiaCoin Core

**RiaCoin** is a next-generation Layer 1 blockchain built from scratch in **Rust**. 
It combines the security of **Ed25519** elliptic curve cryptography, the speed of **Rust**, and the programmability of a **Virtual Machine** for Smart Contracts.

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=flat-square" />
  <img src="https://img.shields.io/badge/Consensus-DPoS-blue?style=flat-square" />
  <img src="https://img.shields.io/badge/Cryptography-Quantum_Ready-black?style=flat-square" />
</p>

## 🚀 Features

-   **High Performance**: Written in Rust for near-instant execution and memory safety without a Garbage Collector.
-   **P2P Networking**: Fully decentralized peer-to-peer network using `libp2p` (mDNS & Floodsub) for node discovery and data propagation.
-   **Smart Contracts**: Integrated Virtual Machine (`VM`) to execute complex logic beyond simple value transfers (e.g., NFT Minting).
-   **Real Cryptography**: Uses `Ed25519` for digital signatures (same standard as Solana) and `SHA-256` for hashing.
-   **Delegated Proof of Stake (DPoS)**: Energy-efficient consensus mechanism replacing the outdated Proof of Work.

## 🛠 Architecture

### Node Structure
-   **`chain.rs`**: Core blockchain logic (Blocks, Transactions, Merkle Trees, State Management).
-   **`wallet.rs`**: Cryptographic key management (Keypair generation, Signing, Verification).
-   **`p2p.rs`**: Networking layer handling Swarm events, Peer Discovery, and Gossip.
-   **`vm.rs`**: The "World Computer" state machine executing smart intents.
-   **`main.rs`**: Async event loop (Tokio) handling P2P events and User I/O.

## 📦 Installation & Usage

### Prerequisites
-   Rust & Cargo installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Running a Node
```bash
git clone git@github.com:DevoRia/riacoin.git
cd riacoin/riacoin_node
cargo run
```

### CLI Commands
Once the node is running, use the interactive REPL:
-   **`t`**: Broadcast a **Transaction** (sends coins).
-   **`s`**: Execute a **Smart Contract** (e.g., Mint an NFT).
-   **`b`**: Check **Balance** and Chain state.
-   **`q`**: Quit the node.

## 🌐 Networking
Start multiple instances in different terminals to see them discover each other automatically via local mDNS and exchange blocks/transactions.

## 📜 License
MIT License.
