# Grig (Research Prototype)

> ⚠️ **WARNING:** This is a research prototype and **MUST NOT** be used in production systems. It has NOT been cryptographically reviewed, formally analyzed, or tested against real attackers. Use at your own risk.

## Overview

Grig is a proposed research framework for a next-generation memory-hard function (MHF) designed for password hashing. It explores the combination of dynamic, state-dependent memory graphs with leakage-resistant memory access techniques. 

While modern systems like Argon2id provide an excellent balance between memory hardness and side-channel resistance, a fundamental tension remains between data-dependent hardness and data-independent leakage protection. Grig aims to address this gap.

## Design Goals

*   **Strong memory hardness** to significantly increase the cost of password guessing attacks.
*   **Dynamic dependency evolution** to thwart algorithmic optimizations.
*   **Reduced observable leakage:** Separating logical dependencies from physical memory observations using hidden selection mechanisms and fixed access schedules.
*   **Hardware Resistance:** Mitigate risks from GPU, ASIC, and time-memory tradeoff (TMTO) optimizations.

## Architecture

The computation is represented as a dynamic directed graph $G = (V,E)$, where nodes represent memory blocks and edges represent cryptographic dependencies.

The graph evolves progressively based on the internal cryptographic state $S(t)$:

$$G(t+1) = F(G(t), S(t))$$

### Core Components

*   **Memory Block:** Represents a fixed-size memory unit.
*   **State:** Maintains the evolving internal cryptographic state.
*   **Graph:** Generates dynamic node dependencies.
*   **Mixer:** Combines parent blocks and state into new memory blocks.
*   **Permutation Layer:** Experiments with hiding physical memory access patterns.

## Algorithm Flow

1.  **Initialization:** Establish the initial state using a cryptographic hash:
    $$S_0 = H(\text{password} \parallel \text{salt} \parallel \text{parameters})$$
2.  **Memory Expansion:** Generate $N$ memory blocks.
3.  **Graph Generation:** Create dynamic node dependencies using the cryptographic state.
4.  **Mixing:** Update memory blocks using parent dependencies.
5.  **State Evolution:** Produce the next internal state after rounds.
6.  **Finalization:** Generate the final password hash output.

## Engineering & Implementation

Rust is the primary implementation language due to its guarantees of memory safety and systems-level performance. C may be utilized later for low-level hardware optimization.

### Repository Structure

```text
grig/
├── grig-core/       # Core Rust hashing library
├── benchmarks/      # Performance evaluation tools
├── tests/           # Testing suite
└── documentation/   # Academic drafts and specifications