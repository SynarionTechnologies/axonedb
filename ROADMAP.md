# AxoneDB Roadmap

## 0–3 Months
- **Deliverables:** Workspace scaffold, core storage prototype, CI pipeline
- **Targets:** Compile-time checks passing; basic WAL and snapshot write/read
- **Success Criteria:** Repository builds on CI; prototype persists and recovers data

## 3–6 Months
- **Deliverables:** Raft replication, networking layer, metrics crate
- **Targets:** Stable leader election; baseline metrics exported
- **Success Criteria:** 3-node cluster passes consistency tests; observability dashboard

## 6–9 Months
- **Deliverables:** CDC streams, admin CLI, SDK stubs
- **Targets:** Subscriptions deliver ordered events; CLI manages clusters
- **Success Criteria:** CDC demo with external consumer; cluster operations via CLI

## 9–15 Months
- **Deliverables:** Performance optimizations, durability tiers
- **Targets:** p99 < 1 ms for single key ops; ULTRA and STRICT modes implemented
- **Success Criteria:** Benchmarks meet latency SLOs; durability verified in failure tests

## 15–24 Months
- **Deliverables:** Multi-language SDKs, extended tooling, GA release
- **Targets:** SDKs for Rust, Go, TypeScript, .NET; benchmark suite comparable to YCSB
- **Success Criteria:** Production-ready release with documentation and ecosystem support
