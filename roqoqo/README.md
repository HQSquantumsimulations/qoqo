<img src="qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

# roqoqo

roqoqo is a toolkit to represent quantum circuits by [HQS Quantum Simulations](https://quantumsimulations.de).

For a detailed introduction see the [user documentation](https://hqsquantumsimulations.github.io/qoqo_examples/) and the [qoqo examples repository](https://github.com/HQSquantumsimulations/qoqo_examples)

What roqoqo is:

* A toolkit to represent quantum programs including circuits and measurement information
* A thin runtime to run quantum measurements
* A way to serialize quantum circuits and measurement information
* A set of optional interfaces to devices, simulators and toolkits (e.g. [qoqo_qest](https://github.com/HQSquantumsimulations/qoqo-quest), [qoqo_mock](https://github.com/HQSquantumsimulations/qoqo_mock), [qoqo_qasm](https://github.com/HQSquantumsimulations/qoqo_qasm))

What roqoqo is **not**:

* A decomposer translating circuits to a specific set of gates
* A quantum circuit optimizer
* A collection of quantum algorithms


## roqoqo

[![Crates.io](https://img.shields.io/crates/v/roqoqo)](https://crates.io/crates/roqoqo)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo)](https://docs.rs/roqoqo/)
![Crates.io](https://img.shields.io/crates/l/roqoqo)
[![codecov](https://codecov.io/gh/HQSquantumsimulations/qoqo/branch/main/graph/badge.svg?token=S1IN066V2W)](https://codecov.io/gh/HQSquantumsimulations/qoqo)

roqoqo provides:

* A circuit struct to represent quantum programs
* Single-Qubit, Two-Qubit and Multi-Qubit Operations that can be executed (decomposed) on any universal quantum computer
* PRAGMA Operations that only apply to certain hardware, simulators or annotate circuits with additional information
* Classical Registers and Measurement operations to use with a quantum program
* Measurement structs for evaluating observable measurements based on projective measurements from quantum hardware or simulator readouts
* A Backend trait defining a standard for interfacing from qoqo to other toolkits, hardware and simulators that can return measured values
* Serialize and deserialize support for circuits and measurement information via the serde crate.

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

### Installation

To use roqoqo in a Rust project simply add

```TOML
roqoqo = {version="1.0"}
```

to the `[dependencies]` section of the project Cargo.toml.