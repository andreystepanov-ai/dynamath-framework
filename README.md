# Dynamath Framework

This repository contains a skeleton for the Dynamath framework — a modular implementation of *Dynamic Mathematics*, unifying mathematics, semantics, and cognition.

Details: https://zenodo.org/records/16762105 

Visualization linked to work: https://dynamath-dynaflow.vercel.app/ 


## Project structure

- **core**: Rust library implementing the formal ontology (entities, relations, dynamic graphs, energy, entropy, fields).
- **spec**: Protobuf definitions (`ontology.proto`), operator registry (`operators.json`), frame boundaries (`frames.toml`).
- **viz**: Front-end visualisations (React/WASM) to explore flows, fields, phase spaces and dynamic graphs.
- **labkit**: Python tools (JAX/PyTorch) for experimentation, optimisation and machine learning.
- **kernels**: GPU kernels (Triton/CUDA) for high-performance tensor operations and field evaluations.
- **serve**: Service layer for orchestrating simulations and exposing APIs (REST/gRPC).

Each module is designed to evolve independently while integrating with the others via common specifications.

## Research context

The underlying theory draws on vector semantics, graph theory and transformation algebras to model the evolution of ideas, meanings and cognitive structures over time. Key elements correspond to sections of the [Dynamic Mathematics] paper:

- *Entity* §1: formal definition `E = \langle\mathbf{e}, \mathcal{S}(t), \mathcal{R}, \mathcal{F}\rangle`.
- *Axioms* §2: causality, closure, identity, structure, locality.
- *Metric & Potential* §3: density `\rho`, potential `\Phi`, force `\mathbf{F} = -\nabla \Phi`.
- *Semantic Motion* §4: law of motion `\frac{d\mathbf{e}}{dt} = \mathbf{V}(\mathbf{e}, t)`.
- *Dynamic Graphs* §5: evolution of weights `\frac{d w_{ij}}{dt} = \alpha A - \beta D + \gamma C`.
- *Phase & Attractors* §8: dynamics of states `\frac{d\mathcal{S}}{dt} = \mathcal{F}(\mathcal{S})`.
- *Drift & Entropy* §13: drift distance `D_n = \|s_0 - f_n \dots f_1(s_0)\|` and entropy `H(\mathcal{S})`.
- *Causality & Flux* §14: flux `\Phi_{i\to j}(t) = \frac{d}{dt} I(s_i \to s_j; t)`.
- *Idea Fields* §18.6: field equations `\nabla \cdot \mathbf{I} = \rho`, `\nabla \cdot \boldsymbol{\tau} = 0`, `\nabla \times \mathbf{I} = \frac{\partial \boldsymbol{\tau}}{\partial t}`, `\nabla \times \boldsymbol{\tau} = \mu \mathbf{J} + \varepsilon \frac{\partial \mathbf{I}}{\partial t}`.

These constructs are prototyped across the modules in this repository, aiming for a practical implementation of the Dynamath vision.
