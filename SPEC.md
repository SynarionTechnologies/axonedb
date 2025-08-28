# AxoneDB Technical Specification

## Performance Targets
| Operation | p99 | p999 |
|-----------|-----|------|
| GET/PUT single key (intra-zone) | < 1 ms | < 2 ms |
| Batch ≤8 operations | < 2 ms | — |
| Tail latency bound | \<= 10× median | — |
| Throughput (3-node NVMe 10–25G) | \>= 1M ops/s | — |

## Durability Tiers
| Tier | Description |
|------|-------------|
| ULTRA | Ack after in-memory replication |
| STRICT | fsync + quorum |
| EXTREME | Kernel-bypass / RDMA |

## Storage Model
- Append-only write-ahead log (WAL) using O_DIRECT + io_uring
- Incremental snapshots with full snapshot ≤ 30 min, delta cadence ≤ 1 min
- Key-value data model with TTL per key or namespace

## Change Data Capture (CDC)
- Ordered streams with backpressure
- Subscription-based delivery

## Replication
- Raft consensus per shard
- Lease-based follower reads

## Observability
- hdrhistogram metrics (p50/p90/p99/p999)
- OpenTelemetry tracing
- Structured JSON logging

## Constraints
- Binary keys up to 8 KiB; values up to 1 MiB (v1)
- Linux x86-64, kernel ≥ 5.15 with io_uring
- No unsecured mode; TLS 1.3 mandatory
