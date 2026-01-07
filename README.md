# UnifyRE 🚀

[![Build Status](https://img.shields.io/badge/status-active-success.svg)](https://github.com/hexria/UnifyRE)
[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/hexria/UnifyRE/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos-lightgrey.svg)](#)

**UnifyRE** is a next-generation, high-performance binary analysis and reverse engineering tool built in Rust. It unifies static heuristics, deterministic diffing, and intelligent transparency into a single, cohesive CLI platform.

---

## 🌟 Core Pillars

### ⚙️ Deterministic Analysis
UnifyRE produces identical results across runs by enforcing stable address-based sorting for all internal collections. Your analysis is reproducible, consistent, and audit-ready.

### 🧠 Trust & Transparency
No more "black box" findings. The integrated **Explanation Engine** provides deep technical context for every detection. Use `unifyre explain <ID>` to understand the *why* and *how* behind any security risk.

### 🔌 Extensible Architecture
Built on a trait-based abstraction layer, UnifyRE supports dynamic plugin loading (`.so`/`.dylib`) and headless automation via `.ure` scripts.

---

## 🛠 Features

### 🔍 Static Intelligence
- **Shannon Entropy**: Per-section entropy calculation to detect packed or encrypted data.
- **Suspicious Heuristics**: Detection of NOP sleds, shellcode sequences, and unusual section names.
- **Multi-Format Support**: Native handling of ELF, PE, and Mach-O binaries.

### ⚖️ Comparison & Diff Engine
Compare two binaries to find structural differences:
- **Symbol Diffing**: Track added, removed, or modified symbols.
- **Section Entropy Trends**: Visualize how data randomness changes between versions.
- **Address-Level Precision**: Instruction-level awareness.

### 📊 Professional Reporting
- **Human Output**: Beautiful, colorized terminal reports for rapid triage.
- **JSON Export**: Strictly versioned schema for CI/CD integration.
- **HTML Dashboards**: Standalone, interactive visual reports with modern aesthetics.

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/hexria/UnifyRE.git
cd UnifyRE

# Build for release
cargo build --release

# Install to path
cargo install --path .
```

### Basic Usage

**Analyze a binary with a specific profile:**
```bash
unifyre analyze /bin/ls --profile malware
```

**Compare two versions of a binary:**
```bash
unifyre diff v1/app v2/app --format json
```

**Get a deep-dive on a finding:**
```bash
unifyre explain HIGH_ENTROPY
```

---

## 📘 How UnifyRE Thinks

### The Analysis Pipeline
UnifyRE operates in three distinct layers:
1. **Loader Layer**: Normalizes binary formats (ELF/PE/Mach-O) into a unified internal representation.
2. **Analysis Engine**: Applies heuristics and pattern scanners based on the selected **Analysis Profile** (`audit`, `malware`, `exploit`).
3. **Transparency Layer**: Maps finding IDs to a curated technical knowledge base to assist reverse engineers.

---

## 🏛 Breaking Changes Policy
As of **v1.0.0**, UnifyRE adheres to Semantic Versioning (SemVer):
- **Major versions (X.0.0)** may introduce breaking changes to the JSON schema or CLI.
- **Minor versions (1.X.0)** will maintain CLI and argument backward compatibility.
- **Patch versions (1.0.X)** focus exclusively on bug fixes and performance.

---

## 🤝 Contributing
Contributions are welcome! Please see our architecture documentation for details on implementing new disassembler backends or analysis heuristics.

## 📄 License
UnifyRE is released under the **MIT License**.
