# AxoneDB Contribution Guide

## 0. Overview
- **Purpose:** Canonical instructions for all contributors and AI assistants.
- **Scope:** Entire repository.
- **Principles:** Precision, measurability, security, Trunk-Based Development.

## 1. Product Introduction
**One-liner:** AxoneDB is an ultra-low latency, distributed database for real-time and mission-critical applications.

**Vision**
- Premium reliability and latency discipline.
- Operational simplicity.

**Non-goals**
- Not a general analytics warehouse.
- Not a drop-in SQL replacement.

**Core Capabilities**
- KV with secondary indexes (ART).
- Replication via Raft.
- Change Data Capture (CDC).
- Snapshots.
- Write-Ahead Log (io_uring).
- Observability.
- Security by default.

**Tenets**
1. Fast-path first.
2. Durability tiers.
3. Simplicity of operations.
4. Measurable quality.
5. Secure by default.

## 2. Specifications & Hard Constraints

### Performance & Latency
| Operation | Target p99 | Target p999 |
|-----------|------------|-------------|
| GET/PUT single key (intra-zone) | < 1 ms | < 2 ms |
| Batch ≤8 | < 2 ms | – |
| Tail latency bound | \<= 10× median | – |
| Throughput (3-node NVMe 10–25G) | \>= 1M ops/s | – |

### Robustness & High Availability
| Metric | Target | Max |
|--------|--------|-----|
| Crash recovery (WAL+snapshot) | < 2 s | 5 s |
| Leader failover | < 1 s | 2 s |
| Data loss in STRICT/EXTREME | 0 | 0 |

### Storage & Durability
| Component | Requirement |
|-----------|-------------|
| WAL | Append-only, O_DIRECT + io_uring, group commit 50–200 µs |
| Snapshots | Incremental; full \<= 30 min; delta cadence \<= 1 min |
| Durability profiles | ULTRA (ack after replicated memory), STRICT (fsync + quorum), EXTREME (kernel-bypass/RDMA) |

### Data Model
- Binary key–value (key \<= 8 KiB, value \<= 1 MiB for v1).
- Primary index: cache-aligned Robin Hood hash (in-memory).
- Secondary indexes: ART (range/prefix).
- TTL per key/namespace.
- CDC: ordered streams with backpressure.

### Security
| Aspect | Requirement |
|--------|-------------|
| TLS | TLS 1.3 mandatory |
| Encryption at rest | AES-256-GCM, per-namespace keys |
| Auth | API keys & optional JWT; ACLs & minimal RBAC |
| Audit | Immutable audit log |

### Observability
| Item | Requirement |
|------|-------------|
| Metrics | hdrhistogram p50/p90/p99/p999; latency heatmaps |
| Tracing | OTEL traces |
| Logging | Structured JSON logs; flamegraphs on demand |

### Compatibility & Platform
| Requirement | Details |
|-------------|---------|
| OS | Linux x86-64, kernel \>= 5.15, io_uring |
| Hardware | NUMA-aware; pinned threads; NIC 10–25G |
| Cloud | AWS, GCP, Azure, and bare metal |

### Release Gate Checklist
- [ ] Meets all latency/throughput SLOs.
- [ ] Crash recovery and failover within targets.
- [ ] Durability profile tested.
- [ ] Security features (TLS, auth, audit) enabled.
- [ ] Observability dashboards updated.

## 3. Architecture Guidance
- **Shard-per-core, NUMA-aware** runtime with actor model and SPSC rings.
- **Value-log append-only** with in-memory indexes and bounded-budget GC/compaction.
- **Raft per shard** (pipelined) with lease-based follower reads.
- **Binary data-plane** (FlatBuffers/Cap’n Proto) over persistent TCP; **control-plane** via gRPC.

**DDD/CQRS/Event Sourcing**
- AxoneDB is a database engine; avoid app-level DDD/CQRS.
- Use modular boundaries: `core/`, `storage/`, `raft/`, `net/`, `cdc/`, `metrics/`, `admin/`.
- Internal event-log discipline for WAL/replication.

**Data Flow**
`net -> parse -> route(shard) -> exec -> durability -> replicate -> ack`

**Rationale**
This architecture maximizes cache locality, minimizes cross-core contention, and avoids microservice overhead or heavy DDD frameworks unsuitable for a DB engine.

## 4. Rust Coding Standards

### Language & Toolchain
- Rust stable; MSRV pinned.
- `rustfmt`, `clippy` (deny warnings), `cargo-audit`, `cargo-deny` mandatory.

### Safety
- `unsafe` forbidden except in audited leaf modules.
- Document invariants with `// SAFETY:` comments.

### Performance Policy
- No heap allocations on the hot path; use arenas/pools.
- Branchless and cache-aware code where practical.
- Avoid syscalls in hot loops; use io_uring & batching.
- Use `#[inline]` judiciously; measure before merging.

### API Design
- Small, composable traits.
- Explicit errors via `thiserror`.
- No panics on recoverable paths.
- `Result` everywhere; explicit timeouts & backpressure.

### Naming & Style
- Idiomatic Rust naming.
- Cohesive modules; minimal public APIs.

### Docs
- Rustdoc for all public items: Summary, Arguments, Returns, Errors, Panics/Safety, Examples (compile-tested).

### Examples
```rust
// KV put/get
let mut db = axonedb::open();
db.put(b"k", b"v")?;
let v = db.get(b"k")?;

// WAL write
wal.append(&record)?;

// Raft step
raft.step(msg)?;
```

### Style Checklist
- [ ] `cargo fmt --all`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo bench` (if affected)
- [ ] `cargo audit && cargo deny`

### PR Gate
All items in the style checklist must pass before merge.

## 5. Documentation Rules
- Rustdoc mandatory with compile-tested examples.
- `/docs/en/` is the source of truth; keep `/docs/fr/` in sync when present.
- Maintain `GLOSSARY.md` and `/docs/fr/GLOSSARY.md`.
- Update `README.md` and `ROADMAP.md` when public APIs or milestones change.

**Feature PR Requirements**
- API reference delta.
- Usage notes.
- Operational guidance (limits, tuning flags).
- Security notes (if relevant).

**Doc PR Template**
```
## Summary

## API Changes

## Usage

## Operations

## Security
```

## 6. Testing & Benchmarking
- Unit tests for every module (happy & edge cases).
- Integration tests: KV semantics, index correctness, TTL expiry, CDC ordering.
- Property-based tests: WAL replay idempotence, Raft safety, hash/ART invariants.
- Failure injection: crash-mid-write, torn writes, disk full, network partitions.
- Performance: YCSB-like suite; publish p50/p90/p99/p999 and tail heatmaps.
- Release blocker: any regression >5% in p99 or throughput fails CI.

**Command Cheatsheet**
- `cargo test`
- `cargo test --features integration`
- `cargo bench`
- `cargo test -p storage -- --ignored` (failure injection)

## 7. Trunk-Based Development (TBD)

### Branching
- Single trunk: `main`.
- Small, short-lived feature branches; merge daily.

### Feature Flags
- All incomplete features behind flags documented in `/docs/en/flags.md`.
- No dead flags > 2 releases.

### CI Gates
- Format + clippy (deny warnings).
- Unit + integration + property tests.
- Benchmarks (latency budget diff).
- `cargo-audit` & `cargo-deny`.

### Code Review
- ≥1 reviewer; performance-critical code requires 2 and benchmark evidence.

### Releases
- Frequent, small; rolling upgrades validated on a 3-node testbed.

### Backport/Hotfix
- Tag strategy; cherry-pick with test evidence; post-mortem template.

### TBD Checklist
- [ ] Branch merged daily.
- [ ] Feature flags documented.
- [ ] CI gates green.
- [ ] Reviewer approval.

**Flag Lifecycle**
| Stage | Action |
|-------|--------|
| Introduced | Added with docs & tests |
| Released | Enabled by default |
| Retired | Removed within 2 releases |

## 8. Security & Compliance Baseline
- Secrets handling, TLS cert rotation, key management per namespace.
- Threat model: MITM, replay, node compromise.
- Mandatory audit log with redaction policies.
- No insecure mode by default.

## 9. Contributor Prompts & Templates

**Feature Request Prompt**
```
Feature: <concise description>
Why: <user/value/latency target>
Scope: <modules>
Interfaces: <API changes>
Durability Profile Impact: <ULTRA/STRICT/EXTREME>
Provide:
1) Rust code (modules, traits, tests)
2) Bench results (p50/p90/p99/p999)
3) Docs (/docs/en + Rustdoc)
4) Flags (if partial)
```

**Bug Report Prompt**
```
Summary:
Expected vs Actual:
Repro steps:
Logs/metrics:
Impact on p99/tail:
Proposed fix & tests:
```

**Performance PR Exit Criteria**
- p99 not worse.
- p999 not worse.
- Throughput \>= baseline.
- Memory stable.
- No new allocations on the hot path.

## 10. Non-negotiable Rules
1. Do not break architecture boundaries.
2. Do not bypass tests, benches, or security defaults.
3. No feature merges without docs and flags as needed.
4. Everything measurable; if it isn’t measured, it doesn’t ship.

## Verify Before Submit
- [ ] Style checklist passes.
- [ ] Tests and benchmarks run.
- [ ] Docs updated.
- [ ] Flags documented.
- [ ] Reviewers satisfied.

