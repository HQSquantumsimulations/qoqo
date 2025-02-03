# roqoqo

[![Crates.io](https://img.shields.io/crates/v/roqoqo)](https://crates.io/crates/roqoqo)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo)](https://docs.rs/roqoqo/)
![Crates.io](https://img.shields.io/crates/l/roqoqo)
[![codecov](https://codecov.io/gh/HQSquantumsimulations/qoqo/branch/main/graph/badge.svg?token=S1IN066V2W)](https://codecov.io/gh/HQSquantumsimulations/qoqo)

roqoqo is the core rust library for **qoqo** - a toolkit to represent quantum circuits by [HQS Quantum Simulations](https://quantumsimulations.de). 

For a detailed introduction see the [user documentation](https://hqsquantumsimulations.github.io/qoqo_examples/) and the [qoqo examples repository](https://github.com/HQSquantumsimulations/qoqo_examples)

What roqoqo is:

* A toolkit to represent quantum programs including circuits and measurement information
* A thin runtime to run quantum measurements
* A way to serialize quantum circuits and measurement information
* A set of optional interfaces to devices, simulators and toolkits (e.g. [qoqo_quest](https://github.com/HQSquantumsimulations/qoqo-quest), [qoqo_qiskit](https://github.com/HQSquantumsimulations/qoqo-qiskit), [qoqo_for_braket](https://github.com/HQSquantumsimulations/qoqo-for-braket), [qoqo_iqm](https://github.com/HQSquantumsimulations/qoqo_iqm))

What roqoqo is **not**:

* A decomposer translating circuits to a specific set of gates
* A quantum circuit optimizer
* A collection of quantum algorithms

## Installation

To use roqoqo in a Rust project simply add

```TOML
roqoqo = {version="1.0"}
```

to the `[dependencies]` section of the project Cargo.toml.


When using qoqo in a rust project providing a python interface add

```TOML
qoqo = {version="1.0", default-features=false}
```

to the `[dependencies]` section of the project Cargo.toml.

A source distribution now exists but requires a Rust install with a rust version > 1.47 and a maturin version { >= 0.12, <0.13 } in order to be built.

## Examples

For an expanded collection of examples please see the jupyter notebooks in the extra repository [qoqo_examples](https://github.com/HQSquantumsimulations/qoqo_examples). The qoqo examples require the qoqo_quest and qoqo_mock interfaces.

The jupyter notebooks in **Rust** can be found in [qoqo_examples/roqoqo/notebooks/](https://github.com/HQSquantumsimulations/qoqo_examples/tree/main/roqoqo/notebooks). Alternatively, you can also find pure **Rust** versions of the examples in [qoqo_examples/roqoqo/standalone/](https://github.com/HQSquantumsimulations/qoqo_examples/tree/main/roqoqo/standalone)

## Features

roqoqo provides:

* A `Circuit` struct to represent quantum circuits
* A `QuantumProgram` enum to represent quantum programs using different measurement methods
* Structs representing single-qubit, two-qubit, multi-qubit and measurement operations that can be executed (decomposed) on any universal quantum computer
* Structs representing so-called PRAGMA operations that only apply to certain hardware, simulators or annotate circuits with additional information
* Enums that group operations based on the properties of operations (*e.g.* `Operation` for all operations or `SingleQubitGateOperation` for all unitary operations acting on a single qubit)
* Support for symbolic variables
* Readout based on classical registers
* Measurement structs for evaluating observable measurements based on raw readout date returned by quantum computer backends
* An `EvaluatingBackend` trait defining a standard for interfacing from qoqo to hardware and simulators that can return measured values
* A `Device` trait defining a standard to obtain connectivity information and a noise model for quantum computing devices
* Serialize and deserialize support for `Circuit` and `QuantumProgram` via the serde crate.
