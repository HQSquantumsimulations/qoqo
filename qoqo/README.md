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


[![Documentation Status](https://img.shields.io/badge/docs-read-blue)](https://hqsquantumsimulations.github.io/qoqo/)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo)](https://pypi.org/project/qoqo/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo)](https://pypi.org/project/qoqo/)
[![Crates.io](https://img.shields.io/crates/v/roqoqo)](https://crates.io/crates/qoqo)
![Crates.io](https://img.shields.io/crates/l/qoqo)

qoqo provides the Python interface to the underlying roqoqo library, including:

* A `Circuit` class to represent quantum circuits
* A `QuantumProgram` class to represent quantum programs 
* Classes representing single-qubit, two-qubit, multi-qubit and measurement operations that can be executed (decomposed) on any universal quantum computer
* Classes representing so-called PRAGMA operations that only apply to certain hardware, simulators or annotate circuits with additional information
* Support for symbolic variables
* Readout based on classical registers
* Measurement classes for evaluating observable measurements based on raw readout date returned by quantum computer backends
* Serialization to json and deserialization from json for circuits and measurement information. Serialization support can easily be expanded to other targets with the help of the serde crate.

This project is partly supported by [PlanQK](https://planqk.de).

### Installation

On Linux, macOS and Windows on x86 precompiled packages can be found on PyPi and installed via

```shell
pip install qoqo
```

If no pre-built python wheel is available for your architecture you can install qoqo from the source distribution using a rust toolchain (for example available via rustup) and maturin (also available via pip). After installing the rust toolchain and maturing run the same pip install command as above. In some cases on macOS it can be necessary to provide specific linker arguments as shown below:

```shell
# can be necessary on macOS
pip install qoqo
```

When using qoqo in a rust project providing a python interface add

```TOML
qoqo = {version="1.0.0", default-features=false}
```

to the `[dependencies]` section of the project Cargo.toml.