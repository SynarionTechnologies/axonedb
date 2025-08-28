# Vulkan Acceleration Layer

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
- [Internal API](#internal-api)
- [Configuration and Feature Flags](#configuration-and-feature-flags)
- [Observability](#observability)
- [Security](#security)
- [Performance Testing](#performance-testing)
- [Open Questions](#open-questions)

## Overview
AxoneDB plans an optional compute layer using Vulkan to accelerate select workloads. It targets four analytic cases:

1. **Massively Parallel Calculations** – offload large numeric workloads to the GPU.
2. **Vector Indexing & Similarity Search** – build and query high‑dimensional indexes.
3. **Compression & Cryptography** – accelerate buffer transforms with synchronous APIs.
4. **Real‑time OLAP Queries** – execute columnar scans and aggregates at low latency.

Benefits include increased throughput and freeing CPU resources. Limits: data transfer over PCIe, minimum batch sizes and a restricted operator set.

## Architecture
The layer introduces a broker that routes requests to either CPU or GPU backends. Data uses a columnar format for OLAP operators; memory transfers favor zero‑copy where possible. Batches are preferred, but streaming is allowed when latency budgets permit.

## Internal API
```text
axonedb.accel.vector.knn(query, index, k, metric)
axonedb.accel.vector.build_index(vectors, index_type, params)
axonedb.accel.crypto.encrypt(buffer, algo)
axonedb.accel.crypto.decrypt(buffer, algo)
axonedb.accel.compress(buffer, algo)
axonedb.accel.decompress(buffer, algo)
axonedb.accel.olap.aggregate(plan)
```
Operators must be idempotent and deterministic. Backends are separable to allow future CUDA or ROCm implementations without changing the API.

Vector KNN uses IVF/HNSW techniques with L2 or cosine distance and returns top‑k results with distances. OLAP operators perform columnar scans, group‑bys and reductions; plans below a cost threshold execute on CPU. Crypto and compression fall back to CPU if the algorithm is unsupported.

## Configuration and Feature Flags
Vulkan acceleration is off by default and enabled via feature flag. `AXONEDB_ACCEL` accepts `off` or `vulkan`:

```bash
AXONEDB_ACCEL=vulkan
ACCEL_MIN_BATCH=100000
ACCEL_DEVICE_ID=0
ACCEL_TIMEOUT_MS=5000
```

## Observability
Metrics record execution time, throughput and bytes processed. Counters track CPU fallbacks. Logs note activation, device ID and timeout events.

## Security
GPU execution is sandboxed. All SPIR‑V shaders undergo validation. Each request has a quota; sensitive buffers are wiped after cryptographic operations.

## Performance Testing
Methodology only:
- Micro‑benchmarks per operator.
- TPC‑H style scenarios for OLAP.
- ANN recall@k benchmarks for vector search.

## Open Questions
> OPEN QUESTION: optimal batching thresholds per operator.
> OPEN QUESTION: memory layout standards for user‑provided buffers.
