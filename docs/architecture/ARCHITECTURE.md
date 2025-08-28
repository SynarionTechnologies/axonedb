# Architecture

## Table of Contents
- [Execution Flow](#execution-flow)
- [Acceleration Layer](#acceleration-layer)

## Execution Flow
AxoneDB follows a shard-per-core pipeline with parsing, planning and execution stages before durability and replication. This document focuses on the planned acceleration layer.

## Acceleration Layer
The database adds an Acceleration Broker that decides whether operations run on CPU or an optional Vulkan backend.

```mermaid
flowchart LR
  Client --> Parser --> Planner --> Executor --> Broker{Acceleration Broker}
  Broker -->|CPU path| CPU[CPU Operators]
  Broker -->|Vulkan path| VK[Vulkan Compute Backend]
  VK --> Device[GPU Device(s)]
```

Routing rules:
- If the planner estimates cost above a threshold and the batch is large enough, the broker attempts Vulkan execution.
- Otherwise, operations run on the CPU.
- Automatic fallback occurs when the GPU device is unavailable or times out.
Feature flags keep the layer disabled by default.
