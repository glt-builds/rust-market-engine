## rust-market-engine

An async market data processing engine written in Rust.

The system ingests price ticks from multiple simulated venues,
processes them concurrently, computes rolling metrics (e.g. 10s ADV),
and surfaces cross-venue price differences.

The focus is on correctness, concurrency, and clear system design
rather than trading or execution.
