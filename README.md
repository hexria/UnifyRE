# UnifyRE 🚀

Next-generation, CLI-based reverse engineering and binary analysis tool.

## Overview

UnifyRE is a high-performance tool designed to unify static analysis, dynamic debugging, and automation. Built with Rust, it offers a professional-grade platform for binary security audits.

## Phase 2 Highlights (Professionalization & Intelligence)

- **Hardened Architecture**: Trait-based abstractions for massive extensibility.
- **Smart Intelligence**: Built-in Shannon entropy calculation and suspicious sequence detection (NOP sleds, etc.).
- **Plugin System**: Dynamic loading of shared libraries for custom analysis logic.
- **Headless Automation**: Orchestrate complex workflows using `.ure` scripts.
- **Professional Reporting**: Standalone HTML dashboards with modern aesthetics.

## Advanced Usage

### Generate an HTML Security Report

```bash
unifyre report /bin/ls --out report.html --html
```

### Run an Automation Script

```bash
unifyre run examples/security_audit.ure /bin/ls
```

### Pattern Scanning with Suspicious Detection

```bash
unifyre scan patterns /bin/ls --pattern 90909090
```

## Plugin Development

UnifyRE supports dynamic plugins. Implement the `AnalyzerComponent` trait and export the constructor using `declare_plugin!`.

```rust
use unifyre::core::traits::AnalyzerComponent;
use unifyre::declare_plugin;

struct MyPlugin;
impl AnalyzerComponent for MyPlugin {
    fn name(&self) -> &str { "MyPlugin" }
    fn run(&self, provider: &dyn BinaryProvider) -> Result<serde_json::Value> {
        // Custom logic
    }
}

declare_plugin!(MyPlugin, MyPlugin::new);
```

## Installation

```bash
cargo build --release
```

## License

MIT License
