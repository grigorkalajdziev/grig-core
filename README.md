# Grig — Cryptographic Research Prototype

<p align="center">
  <img src="assets/grig-network-architecture.svg" alt="Abstract visualization of a dynamic, interconnected memory graph" width="100%" />
</p>

<p align="center">
  <strong>Exploring new directions in memory-hard password hashing.</strong>
</p>

> [!WARNING]
> **Grig is an experimental research prototype. Do not use it in production or to protect real passwords or sensitive data.** It has not undergone independent cryptographic review, formal security analysis, or adversarial testing. Its design goals are research hypotheses, not established security properties.

## Overview

Grig is a Rust-based research project exploring a memory-hard function (MHF) for password hashing. It investigates whether a dynamic, state-dependent logical dependency graph can be combined with a carefully designed physical memory-access schedule to reduce observable access-pattern leakage without giving up the intended memory-hard properties.

The central research question is:

**Can logical dependency evolution be made less directly observable through physical memory accesses, while retaining meaningful memory hardness?**

This is an open research question. The current implementation does not establish that this separation or any claimed security property has been achieved.

## Research direction

The proposed architecture explores:

- **Dynamic memory graphs** — dependencies that evolve with the internal cryptographic state.
- **State evolution** — a cryptographic state updated as memory blocks are computed.
- **Block mixing** — combining selected parent blocks with the evolving state.
- **Access-pattern analysis** — studying what an observer can infer from physical memory traffic.
- **Memory-hardness evaluation** — measuring the cost of optimized, parallel, and memory-reduced attacks.

### Conceptual model

The computation is modeled as a directed graph:

`G = (V, E)`

where `V` represents memory blocks and `E` represents dependencies between blocks. The intended graph evolution is state-dependent:

`G(t+1) = F(G(t), S(t))`

This notation is a high-level research model, not a complete algorithm specification.

## Proposed long-term architecture

The following are development objectives, not claims about capabilities already implemented:

- Rust-based cryptographic core with explicit interfaces and invariants.
- A complete, versioned algorithm specification.
- Carefully analyzed dynamic dependency-graph construction.
- Explicit side-channel threat analysis and measurable leakage goals.
- Configurable memory cost, passes, and parallelism with strict parameter bounds.
- Safe memory handling, reproducible test vectors, fuzzing, and adversarial benchmarks.
- Independent cryptographic review before any production consideration.

## Threat model under study

The research considers an attacker with:

- Full knowledge of the algorithm and implementation.
- The ability to perform offline password guessing.
- Access to GPU or ASIC acceleration.
- The ability to observe memory-access patterns.

Physical attacks, fault injection, and microarchitectural attacks (including speculative-execution attacks) are currently outside the stated scope. These exclusions do not imply protection against them.

## Security status and comparison

Argon2id is an established password-hashing construction specified in RFC 9106. Grig is an experimental design and has no comparable security validation.

| Area | Argon2id | Grig |
|---|---|---|
| Standardization | Specified in RFC 9106 | Experimental |
| Independent security analysis | Extensively studied | Not yet reviewed |
| Dynamic dependency graph | Not its design focus | Research objective |
| Side-channel properties | Defined by its design and implementation | Not established |
| Production use | Suitable when configured and implemented correctly | **Not suitable** |

No claim is made that Grig is more secure, more memory-hard, or more side-channel resistant than Argon2id.

## Performance benchmarks

The repository includes Criterion.rs benchmarks for experimental performance measurements. Benchmark results describe the tested implementation and configuration only; they do not demonstrate cryptographic strength or resistance to optimized attacks.

Historical benchmark snapshot (1,024 blocks, 3 rounds):

| Metric | Estimate | Reported interval |
|---|---:|---:|
| Mean | 6.4066 ms | 6.2532–6.5957 ms |
| Median | 6.1860 ms | 6.1234–6.2458 ms |
| Standard deviation | 885.58 µs | 434.12 µs–1.2541 ms |

Results were generated using [Criterion.rs](https://github.com/bheisler/criterion.rs). Re-run benchmarks on the target machine before drawing performance conclusions.

## Development roadmap

1. **Implementation audit** — review indexing, initialization, parameter validation, allocation behavior, and error handling.
2. **Formal specification** — define inputs, outputs, encoding, domain separation, graph rules, and all parameter semantics.
3. **Cryptographic analysis** — investigate graph predictability, dependency coverage, time-memory trade-offs, parallel attacks, and side-channel leakage.
4. **Implementation hardening** — add bounded parameters, robust errors, sensitive-state handling, and deterministic test vectors.
5. **Evaluation** — add property tests, fuzzing, cross-platform tests, and comparisons against optimized attack implementations.
6. **Independent review** — obtain external cryptographic analysis before considering any production use.

## Repository structure

```text
grig-core/
├── src/             # Rust implementation
├── benches/         # Criterion benchmarks
├── tests/           # Tests
├── assets/          # Project illustrations and benchmark images
└── README.md        # Project overview and research notes
```

## Implementation

- **Language:** Rust
- **Focus:** cryptographic research, memory-hard functions, and performance evaluation
- **Benchmarking:** Criterion.rs

## Status

- 🚧 Early-stage research prototype
- 🧪 Not security-audited
- 📚 Intended for experimentation and cryptographic research
- ⛔ Not for production use

## License

MIT
