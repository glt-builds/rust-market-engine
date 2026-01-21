## rust-market-engine

An async market data processing engine written in Rust.

The system ingests price ticks from multiple simulated venues, processes them concurrently, computes short-window rolling metrics (e.g. mid-price, micro-price, order book imbalance, and VWAP), and surfaces cross-venue price differences.

The focus is on correctness, concurrency, and high-throughput system design rather than trading strategies or execution.

