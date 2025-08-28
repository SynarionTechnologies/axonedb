# Query Planner

## Table of Contents
- [Acceleration Cost Heuristics](#acceleration-cost-heuristics)
- [Policy](#policy)
- [Open Questions](#open-questions)

## Acceleration Cost Heuristics
The planner evaluates GPU offload using:
- Data size in bytes and expected row counts.
- Predicate selectivity and projected result size.
- Estimated device latency and queue depth.
- PCIe transfer overhead for inputs and outputs.

## Policy
"Small → CPU, Large → Vulkan". If the estimated cost exceeds a threshold and the batch size meets `ACCEL_MIN_BATCH`, the planner recommends the Vulkan path; otherwise the CPU path is chosen. Users may override with the hint `/*+ accel:vulkan */` in query text.

## Open Questions
> OPEN QUESTION: dynamic calibration of thresholds across heterogeneous hardware.
