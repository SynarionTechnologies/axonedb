# ADR-0001: Acceleration with Vulkan

## Table of Contents
- [Status](#status)
- [Context](#context)
- [Decision](#decision)
- [Consequences](#consequences)
- [Open Questions](#open-questions)

## Status
Proposed

## Context
AxoneDB aims to accelerate compute-intensive workloads on GPUs while remaining portable across Linux and Windows. Vulkan compute offers vendor neutrality unlike CUDA and avoids ecosystem lock-in.

## Decision
Adopt Vulkan compute as the initial GPU backend. Introduce an Acceleration Broker that routes eligible operations and exposes a stable internal API decoupled from execution backends.

## Consequences
- Memory handling grows more complex; columnar formats are recommended for OLAP.
- A broker adds routing overhead but enables future backends such as CUDA or ROCm without changing the API.
- PCIe transfer costs mandate minimum batch sizes.
- Sensitive data must be cleared from GPU memory after use.

## Open Questions
> OPEN QUESTION: cross-platform GPU sandboxing and SPIR-V validation pipeline.
