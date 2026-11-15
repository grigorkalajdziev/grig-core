# Grig

**Experimental research into memory-hard password hashing and dynamic memory-dependency graphs.**

> [!WARNING]
> Grig is a research prototype. It is not suitable for production password storage or protection of sensitive data. It has not received independent cryptographic review, formal security analysis, or adversarial testing. Use a reviewed password-hashing scheme such as Argon2id for real systems.

![Grig dynamic memory graph concept](assets/grig-network-architecture.svg)

## Research direction

Grig explores whether a memory-hard construction can use evolving, state-dependent logical dependencies while reducing information exposed by physical memory-access patterns.

The central research question is whether logical dependency selection can be meaningfully decoupled from observable physical memory access without undermining memory hardness. This is a hypothesis to investigate, not an established security property.

## Long-term architecture goals

- **Rust cryptographic core** with a small, auditable implementation surface.
- **Precise construction specification** defining inputs, state transitions, graph generation, memory layout, output encoding, and parameter constraints.
- **Dynamic dependency graph** with explicit analysis of dependency reachability and graph evolution.
- **Side-channel threat analysis** that distinguishes address-trace leakage from timing, cache, power, and other channels.
- **Configurable resource parameters** for memory cost, passes, and parallelism, with safe bounded validation.
- **Memory safety and secret handling**, including checked arithmetic, allocation limits, and deliberate zeroization.
- **Reproducible test vectors** and cross-platform deterministic behavior.
- **Adversarial evaluation** against GPU/ASIC strategies and time-memory trade-offs.
- **Independent cryptographic review** before any production claim.

These are development goals, not features or security guarantees already achieved.

## Threat model under investigation

The intended research adversary may know the complete algorithm, perform offline password guessing, use GPU/ASIC resources, and observe memory-access patterns. Physical attacks, fault injection, and microarchitectural attacks are currently outside the stated scope. The threat model must be refined before security claims can be made.

## Current implementation

The Rust implementation is early-stage. The current API, graph construction, memory initialization, parameter handling, and cryptographic composition require further review. In particular, the demonstration in `src/main.rs` uses Rust's `DefaultHasher`; it is not a cryptographic password hash and must never be used as one.

## Development checks

The repository includes a GitHub Actions workflow that checks formatting, runs Clippy, and executes the Rust test suite on pushes and pull requests.

Run the same checks locally:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Benchmarks

Criterion.rs is used for performance experiments. Benchmark results are meaningful only when accompanied by the exact source revision, hardware, operating system, compiler version, parameters, and benchmark procedure. Performance alone does not establish memory hardness or cryptographic security.

## Status

- Early-stage research prototype
- Not security-audited
- Not production-ready
- Intended for experimentation and cryptographic research

## License

MIT
