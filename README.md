<img src="qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

# qoqo

Quantum Operation Quantum Operation  
Yes we use [reduplication](https://en.wikipedia.org/wiki/Reduplication)

qoqo/roqoqo is a toolkit to represent quantum circuits by [HQS Quantum Simulations](https://quantumsimulations.de).

For a detailed introduction see the [user documentation](https://hqsquantumsimulations.github.io/qoqo_examples/) and the [qoqo examples repository](https://github.com/HQSquantumsimulations/qoqo_examples)

What roqoqo/qoqo is:

* A toolkit to represent quantum programs including circuits and measurement information
* A thin runtime to run quantum measurements
* A way to serialize quantum circuits and measurement information
* A set of optional interfaces to devices, simulators and toolkits (e.g. [qoqo_quest](https://github.com/HQSquantumsimulations/qoqo-quest), [qoqo_mock](https://github.com/HQSquantumsimulations/qoqo_mock), [qoqo_qasm](https://github.com/HQSquantumsimulations/qoqo_qasm))

What roqoqo/qoqo is **not**:

* A decomposer translating circuits to a specific set of gates
* A quantum circuit optimizer
* A collection of quantum algorithms

This repository contains two components:

* roqoqo: the core rust library
* qoqo: the python interface to roqoqo

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

## qoqo

[![Documentation Status](https://img.shields.io/badge/docs-read-blue)](https://hqsquantumsimulations.github.io/qoqo/)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo)](https://pypi.org/project/qoqo/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo)](https://pypi.org/project/qoqo/)
[![Crates.io](https://img.shields.io/crates/v/roqoqo)](https://crates.io/crates/qoqo)
![Crates.io](https://img.shields.io/crates/l/qoqo)

qoqo provides a full python interface to the underlying roqoqo library, including:

* A circuit class to represent quantum programs
* Single-Qubit, Two-Qubit and Multi-Qubit Operations that can be executed (decomposed) on any universal quantum computer
* PRAGMA Operations that only apply to certain hardware, simulators or annotate circuits with additional information
* Classical Register and Measurement operations to use with a quantum program
* Measurement structs for evaluating observable measurements based on projective measurements from quantum hardware or simulator readouts
* A QuantumProgram class combining circuits and measurement information in complete quantum programms with a simple interface
* Serialization to json and deserialization from json for circuits and measurement information. Serialization support can easily be expanded to other targets with the help of the serde crate.

### Installation

On Linux, macOS and Windows on x86 precompiled packages can be found on PyPi and installed via

```shell
pip install qoqo
```

Alternatively, installing from the source distribution is possible. For this, a rust toolchain and the maturin Python package need to be already installed. A Rust toolchain can be installed using rustup.
With this Rust toolchain installed, qoqo can be installed using a pip command:

```bash
# After installing the Rust toolchain, execute the following:
pip install maturin
pip install qoqo
```

When using qoqo in a rust project providing a python interface add

```TOML
qoqo = {version="1.0", default-features=false}
```

to the `[dependencies]` section of the project Cargo.toml.

A source distribution now exists but requires a Rust install with a rust version > 1.47 and a maturin version { >= 0.12, <0.13 } in order to be built.

### Examples

Since qoqo provides a full python interface to the underlying roqoqo library, there are examples for python users and for Rust users.

For an expanded collection of examples please see the jupyter notebooks in the extra repository [qoqo_examples](https://github.com/HQSquantumsimulations/qoqo_examples). The qoqo examples require the qoqo_quest and qoqo_mock interfaces.

* **qoqo examples**: For jupyter notebooks in **python**, please refer to [qoqo_examples/qoqo/](https://github.com/HQSquantumsimulations/qoqo_examples/tree/main/qoqo).
* **roqoqo examples**: The jupyter notebooks in **Rust** can be found in [qoqo_examples/roqoqo/notebooks/](https://github.com/HQSquantumsimulations/qoqo_examples/tree/main/roqoqo/notebooks). Alternatively, you can also find pure **Rust** versions of the examples in [qoqo_examples/roqoqo/standalone/](https://github.com/HQSquantumsimulations/qoqo_examples/tree/main/roqoqo/standalone)

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
