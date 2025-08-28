# AxoneDB
![Acceleration Planned](https://img.shields.io/badge/Acceleration-planned-lightgrey)

AxoneDB — An ultra-low latency, distributed database built for high-performance, real-time applications.

## Introduction
AxoneDB targets workloads that demand microsecond responsiveness and predictable tail latency. It combines a shard-per-core architecture with modern storage primitives to deliver deterministic performance.

## Features
- Shard-per-core execution model
- Raft-based replication
- Durable write-ahead log and snapshots
- Change Data Capture (CDC) streams
- Comprehensive observability

## Acceleration (planned)
An optional Vulkan-based acceleration layer for heavy analytic workloads is under design. See [docs/specs/acceleration/vulkan.md](docs/specs/acceleration/vulkan.md) for details.

## Roadmap
See [ROADMAP.md](ROADMAP.md) for phased milestones.

## Getting Started
1. Ensure Rust stable is installed.
2. Build the workspace:
   ```bash
   cargo build --workspace
   ```

## Contributing
Contributions are welcome! Review the [CONTRIBUTING.md](CONTRIBUTING.md) guidelines. Detailed specs are in [SPEC.md](SPEC.md) and AI/dev guidance is in [AGENTS.md](AGENTS.md).

## License
Licensed under the [Apache License, Version 2.0](LICENSE).
