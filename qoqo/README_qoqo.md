# qoqo

[![Documentation Status](https://img.shields.io/badge/docs-read-blue)](https://hqsquantumsimulations.github.io/qoqo/)
![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/actions/workflows/hqs-ci-test-rust-pyo3.yml/badge.svg)
[![PyPI](https://img.shields.io/pypi/v/qoqo)](https://pypi.org/project/qoqo/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo)](https://pypi.org/project/qoqo/)
[![Crates.io](https://img.shields.io/crates/v/roqoqo)](https://crates.io/crates/qoqo)
![Crates.io](https://img.shields.io/crates/l/qoqo)

**qoqo** is a toolkit to represent quantum circuits by [HQS Quantum Simulations](https://quantumsimulations.de). The name “qoqo” stands for “Quantum Operation Quantum Operation,” making use of [reduplication](https://en.wikipedia.org/wiki/Reduplication). 

For a detailed introduction see the [user documentation](https://hqsquantumsimulations.github.io/qoqo) and the [qoqo examples repository](https://github.com/HQSquantumsimulations/qoqo_examples).

What qoqo is:

* A toolkit to represent quantum programs including circuits and measurement information.
* A thin runtime to run quantum measurements.
* A way to serialize quantum circuits and measurement information.
* A set of optional interfaces to devices, simulators and toolkits (e.g. [qoqo_quest](https://github.com/HQSquantumsimulations/qoqo-quest), [qoqo_qiskit](https://github.com/HQSquantumsimulations/qoqo-qiskit), [qoqo_for_braket](https://github.com/HQSquantumsimulations/qoqo-for-braket), [qoqo_iqm](https://github.com/HQSquantumsimulations/qoqo_iqm)).

What qoqo is **not**:

* A decomposer translating circuits to a specific set of gates
* A quantum circuit optimizer
* A collection of quantum algorithms

If you are looking for a comprehensive package that integrates all these features, we invite you to explore our [HQStage](https://cloud.quantumsimulations.de/) software.


## Installation

On Linux, macOS and Windows on x86 precompiled packages can be found on PyPi and installed via

```shell
pip install qoqo
```

### Installing from source (optional)

If no pre-built python wheel is available for your architecture, you can install qoqo from the source distribution using a rust toolchain (for example available via rustup) and maturin (also available via pip). After installing the rust toolchain and maturing run the same pip install command as above. In some cases on macOS it can be necessary to provide specific linker arguments as shown below:

```shell
# can be necessary on mscOS
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" pip install qoqo
```

```shell
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" maturin build -m qoqo/Cargo.toml  --release
pip install target/wheels/$NAME_OF_WHEEL
```

### Using in a rust project

When using qoqo in a rust project providing a python interface add

```TOML
qoqo = {version="1.0.0", default-features=false}
```

to the `[dependencies]` section of the project Cargo.toml.


## Create your first circuit in qoqo

The following code snippet can be used to create your first circuit in qoqo. For an expanded collection of **examples** please see the jupyter notebooks in the [qoqo_examples](https://github.com/HQSquantumsimulations/qoqo_examples) repository.

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo_quest import Backend

circuit = Circuit()
circuit += ops.DefinitionBit(name="classical_reg", length=2, is_output=True)
circuit += ops.PauliX(qubit=0)
circuit += ops.CNOT(0,1)
circuit += ops.PragmaDamping(qubit=0, gate_time=0.1, rate=0.1)
circuit += ops.PragmaDamping(qubit=1, gate_time=0.1, rate=0.1)
circuit += ops.MeasureQubit(qubit=0, readout="classical_reg", readout_index=0)
circuit += ops.MeasureQubit(qubit=1, readout="classical_reg", readout_index=1)
circuit += ops.PragmaSetNumberOfMeasurements(number_measurements=1000, readout="classical_reg")

backend = Backend(number_qubits=2)
bit_registers, float_registers, complex_registers = backend.run_circuit(circuit)
```

## Features

**qoqo** provides the following functionalities and features:

* A `Circuit` class to represent quantum circuits
* A `QuantumProgram` class to represent quantum programs 
* Classes representing single-qubit, two-qubit, multi-qubit and measurement operations that can be executed (decomposed) on any universal quantum computer
* Classes representing so-called PRAGMA operations that only apply to certain hardware, simulators or annotate circuits with additional information
* Support for symbolic variables
* Readout based on classical registers
* Measurement classes for evaluating observable measurements based on raw readout date returned by quantum computer backends
* Serialization to json and deserialization from json for circuits and measurement information. Serialization support can easily be expanded to other targets with the help of the serde crate.

Please refer to the [API documentation](https://hqsquantumsimulations.github.io/qoqo/python_api_docs/index.html) for technical details on qoqo functions.

## Support

This project has been partly supported by [PlanQK](https://planqk.de) and is partially supported by [QSolid](https://www.q-solid.de/) and [PhoQuant](https://www.quantentechnologien.de/forschung/foerderung/quantencomputer-demonstrationsaufbauten/phoquant.html).

## Contribution

We welcome contributions to the project. If you want to contribute code, please have a look at [CONTRIBUTE.md](https://github.com/HQSquantumsimulations/qoqo/blob/main/CONTRIBUTE.md) for our code contribution guidelines.

In order to facilitate the contribution of the addition of a new gate, please also have a look at [add_new_gate.md](https://github.com/HQSquantumsimulations/qoqo/blob/main/add_new_gate.md) to read a quick explanation of the main steps necessary.

## Citation

If you use **qoqo**, please cite it by including the URL of this github repository, or using the provided BibTeX entry:

```
@misc{qoqo2021,
      title={qoqo - toolkit to represent quantum circuits},
      author={HQS Quantum Simulations GmbH},
      year={2021},
      url={https://github.com/HQSquantumsimulations/qoqo},
}
```