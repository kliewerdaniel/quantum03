# 🔐 Quantum-Secured Messaging Application

[![Rust](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://www.typescriptlang.org)
[![React](https://img.shields.io/badge/React-18+-61dafb.svg)](https://reactjs.org)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)
[![Security: Quantum Resistant](https://img.shields.io/badge/Security-Quantum_Resistant-purple.svg)](#)

> **A next-generation secure messaging application implementing post-quantum cryptography with a roadmap for quantum hardware integration.**

## 🚀 Overview

This project implements a defense-in-depth cryptographic messaging system designed to withstand attacks from both classical and quantum computers. Built with modern cryptographic standards and designed for future quantum hardware integration.

**Current Status:** Classical post-quantum cryptography implementation
**Future Path:** Hardware integration for QKD and quantum authentication
**Security Model:** Hybrid classical-quantum architecture with graceful degradation

## ✨ Key Features

- 🔒 **Post-Quantum Security**: NIST-standardized ML-KEM and ML-DSA algorithms
- ⚡ **High Performance**: Optimized Rust core with WebAssembly bindings
- 🔄 **Future-Ready**: Hardware abstraction layer for quantum devices
- 📱 **Cross-Platform**: Desktop (Electron) and mobile (React Native) clients
- 🌐 **Decentralized**: No central server stores message contents
- 🔐 **End-to-End Encryption**: Messages encrypted before leaving your device
- 🚀 **Modern Stack**: Rust cryptographic core, TypeScript/React frontend

## 🏗️ Architecture Overview

The application uses a layered architecture designed for both current security needs and future quantum integration:

```
┌─────────────────────────────────────────────────────────┐
│         User Interface Layer (React/Electron)           │
├─────────────────────────────────────────────────────────┤
│       Application Logic Layer (TypeScript/Node.js)      │
├─────────────────────────────────────────────────────────┤
│     Cryptographic Abstraction Layer (Rust/WASM)        │
├─────────────────────────────────────────────────────────┤
│   Post-Quantum Crypto Engine (liboqs, Kyber, Dilithium)│
├─────────────────────────────────────────────────────────┤
│  Quantum Simulation Layer (Classical QKD Simulation)    │
├─────────────────────────────────────────────────────────┤
│      Hardware Abstraction Layer (Future QKD HAL)        │
├─────────────────────────────────────────────────────────┤
│         Transport Layer (TLS 1.3 + PQ Extensions)       │
└─────────────────────────────────────────────────────────┘
```

### 🔐 Security Features

- **Post-Quantum Cryptography**: NIST-standardized ML-KEM and ML-DSA algorithms
- **Forward Secrecy**: Each message uses unique ephemeral keys
- **End-to-End Encryption**: Messages encrypted before leaving your device
- **Metadata Protection**: Message timing and size obfuscation
- **Deniable Authentication**: Messages verifiable but not provable to third parties

### 🛠️ Technology Stack

- **Frontend**: React 18+ with TypeScript, Electron for desktop
- **Cryptographic Core**: Rust 1.75+ with WebAssembly bindings
- **Server**: Rust with Actix-Web framework
- **Database**: PostgreSQL with Redis caching
- **Deployment**: Docker containerization

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.75+ (with `wasm-pack` for WebAssembly)
- **Node.js** 20+ (with npm/pnpm)
- **PostgreSQL** 16+ (for server)
- **Redis** 7+ (for caching)

### One-Line Setup

```bash
# Clone and setup
git clone https://github.com/your-org/quantum-messenger.git
cd quantum-messenger

# Install dependencies and build
./scripts/setup.sh

# Start development environment
docker-compose up -d
```

### Manual Installation

```bash
# 1. Build cryptographic core
cd crypto-core
cargo build --release
wasm-pack build --target web --out-dir ../client/src/wasm

# 2. Setup client
cd ../client
npm install
npm run build

# 3. Setup server
cd ../server
cargo build --release
export DATABASE_URL="postgresql://user:pass@localhost/qmessenger"
cargo run --bin migrate
cargo run --release
```

### Development

```bash
# Run tests
./scripts/test.sh

# Build for production
./scripts/build.sh

# Development mode
npm run dev  # Client
cargo run   # Server
```

## 📖 Usage

### Basic Messaging

```typescript
import { QuantumMessenger } from './client/src/services/crypto.service';

// Initialize client
const client = new QuantumMessenger();

// Register user
await client.register('alice');

// Send encrypted message
await client.sendMessage('bob', 'Hello, quantum world!');

// Receive messages
const messages = await client.pollMessages();
console.log(messages[0].plaintext); // "Hello, quantum world!"
```

### Server API

```bash
# Register user
curl -X POST http://localhost:8080/v1/register \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "identity_public_key": "..."}'

# Send message
curl -X POST http://localhost:8080/v1/messages \
  -H "Authorization: Bearer <token>" \
  -d "<encrypted_message_binary>"

# Get messages
curl http://localhost:8080/v1/messages \
  -H "Authorization: Bearer <token>"
```

## 🔧 Configuration

### Environment Variables

```bash
# Server
DATABASE_URL=postgresql://user:pass@localhost/qmessenger
REDIS_URL=redis://localhost:6379
RUST_LOG=info

# Client
REACT_APP_API_URL=http://localhost:8080
```

### Docker Deployment

```bash
# Production deployment
docker-compose -f docker-compose.prod.yml up -d

# With monitoring
docker-compose -f docker-compose.monitoring.yml up -d
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Quick Start for Contributors

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Make** your changes with tests
4. **Commit** with conventional commits: `git commit -m "feat: add amazing feature"`
5. **Push** and create a Pull Request

### Development Workflow

```bash
# Setup development environment
./scripts/setup.sh

# Run all tests
./scripts/test.sh

# Check code quality
cargo clippy -- -D warnings
npm run lint
```

### Areas for Contribution

- 🔒 **Cryptography**: Post-quantum algorithm implementations
- 📱 **Mobile**: React Native client development
- 🌐 **Web**: Frontend features and UI improvements
- 🚀 **Performance**: Optimization and benchmarking
- 📚 **Documentation**: Guides and API documentation
- 🧪 **Testing**: Unit, integration, and security tests

## 📋 Roadmap

### Phase 1: Foundation (Current)
- ✅ Post-quantum cryptographic core
- ✅ Basic messaging protocol
- 🔄 Desktop application (Electron)

### Phase 2: Mobile & Features
- 📱 Mobile application (React Native)
- 👥 Group messaging
- 📎 File sharing (encrypted)
- 🔍 Message search

### Phase 3: Quantum Integration
- 🔌 Hardware abstraction layer
- ⚛️ Quantum random number generation
- 🌐 Quantum key distribution support

### Phase 4: Production Ready
- ☁️ Cloud deployment guides
- 📊 Monitoring and analytics
- 🛡️ Security audits and certifications

## 📄 License

This project is licensed under the **AGPL-3.0 License** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **NIST** for post-quantum cryptography standards
- **Signal Protocol** for double ratchet implementation
- **Open Quantum Safe** project for liboqs
- **SIPRI** for quantum technology research

## 📞 Support

- 📖 **Documentation**: [docs/](docs/)
- 🐛 **Issues**: [GitHub Issues](https://github.com/your-org/quantum-messenger/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/your-org/quantum-messenger/discussions)
- 🔒 **Security**: [security@quantummessenger.example.com](mailto:security@quantummessenger.example.com)

---

<div align="center">

**⭐ Star this repo if you find it interesting!**

[![GitHub stars](https://img.shields.io/github/stars/your-org/quantum-messenger.svg?style=social&label=Star)](https://github.com/your-org/quantum-messenger)
[![GitHub forks](https://img.shields.io/github/forks/your-org/quantum-messenger.svg?style=social&label=Fork)](https://github.com/your-org/quantum-messenger/fork)

</div>
