# Changelog

This changelog track changes to the qoqo project starting at version v0.5.0

## v1.0.0-beta.2

* Fixed superoperator construction for general noise pragma

## v1.0.0-alpha.5

* Updated dependencies

## v1.0.0-alpha.4

### Fixed v1.0.0-alpha.4

* `PragmaRepeatedMeasurement` now adds remapped qubits that are not previously in the qubit_mapping of the gate.

## v1.0.0-alpha.3

### Added v1.0.0-alpha.3

* Optional `async` feature including:
    1. AsyncEvaluatingBackend trait for backends that implement async evaluation (especially designed for Backends interfacing a Web-API)
    2. Measurements async evaluating the Future of a register measurement returned from a backend.

### Changed v1.0.0-alpha.3

* Make qubit_remapping more lenient only remapping values found in a HashMap skipping remapping for qubits not found in HashMap instead of returning an error.

## v1.0.0-alpha.2

* Updated to qoqo_calculator 0.8

## v1.0.0-alpha.1

* Device trait added in roqoqo.
* Version updated to 1.0.0-alpha.1

## Release 1.0.0-alpha

prerelease package: documentation not yet complete and new functionalities might be added.

* Removed user access to devices to avoid breaking changes in version 1.
* Updated dependencies and README.
* Added unit tests for serialization of PragmaRepeatedMeasurement operations in a Circuit.
* Removed DoUnitary class from qoqo since functionality replaced by QuantumProgram.

## v0.11.3

* Fixing errors in git pushes

## v0.11.1

### Fixed v0.11.1

* Failed dependency resolution in roqoqo/Cargo.toml

## v0.11

* qoqo can now be built using a source distribution

### Added v0.11

* Semver-style version checking for Circuit serialization. In beta mode (0.y.z) minor version must match (y_library == y_data) in release mode (x.y.z) major version must match (x_library == x_data) and minor version of library must exceed minor version of data (y_library >= y_data).
* `json_schema` implementing `JsonSchema` from schemars for roqoqo data structures.
* Unit tests to validate `json_schema` added for Circuit, QuantumProgram and measurements.
* `roqoqo-test` extended by two new functions for stochastic gate tests: `construct_random_circuit` and `add_random_multi_qubit_gate`.
* A conversion function `to_single_qubit_gate` has been implemented for the OperateSingleQubitGate trait.
* The multiplication function `mul` added to the python interface, i.e. qoqo, for Single Qubit Gates.
* New devices implemented in roqoqo and in qoqo: AllToAllDevice, GenericDevice, GenericChain (only next-neighbour qubits are connected) and GenericGrid (a 2D grid device).
* New rotation gate `RotateXY(theta, phi)` added to the set of single qubit gates.

### Changed v0.11

* The multiplication function `mul` for single qubit gates has been updated so that the result is always normalized.
* `qoqo/examples` has been moved to the new github repository `qoqo_examples` which also includes qoqo examples in Rust now.
* Dependencies have been updated to `qoqo_calculator = v0.7` and `pyo3 = v0.16`. Qoqo python interface has been migrated from #[pyproto] to #[pymethods]. Mutable qoqo_calculator:Calculator has been changed to unmutable where possible after the upgrade to qoqo_calculator version v0.7.
* BasisRotation and CheatedBasisRotation measurements renamed to PauliZProduct and CheatedPauliZProduct measurement to reflect that this is the measurement of the PauliProduct in the z-basis.
* BasisRotation and CheatedBasisRotation measurements renamed to PauliZProduct and CheatedPauliZProduct measurement to reflect that this is the measurement of the PauliProduct in the z-basis.

## v0.10.0

### Fixed v0.10.0

* Bug in running register measurements from a qoqo QuantumProgram (`.run_registers()`)

### Changed v0.10.0

* Increased tolerance for unitary violation when construction unitary matrix for SingleQubitGate from `f64::EPSILON` to `1e-6`.
* Semver-style version checking for Circuit serialization. In beta mode (0.y.z) minor version must match (y_library == y_data) in release mode (x.y.z) major version must match (x_library == x_data) and minor version of library must exceed minor version of data (y_library >= y_data).
* Removed support for deprecated Python 3.6

### Added v0.10.0

* Methon `.input` to return measurement input from measurments in qoqo
* Method `.measurement_type` to return the type of measurement in qoqo

## v0.9.0

### Fixed v0.9.0

* Bug in the probability function of the PragmaDamping gate

### Added v0.9.0

* MultiQubitZZ gate. Rotation under a multi-qubit product of Pauli Z operators.
* `two_qubit_edges` function in Device trait. Used to create a simple graph-library-agnostic representation of the connectivity graph of a device.

## v0.8.1

### Changed v0.8.1

* Updated to pyo3 v0.15.0

## v0.8.0

### Added v0.8.0

* QuantumProgram: A representation of a quantum program that accepts a list of free classical float parameters,
runs measurements on a backend and returns expectation values or the classical register output of the quantum circuits.
QuantumProgram is intended as the main interface between classical software and roqoqo quantum programs.  

### Changed v0.8.0

* In the Device Trait the `change_device` function changed the signature from

    ```rust
    fn change_device(&mut self, operation: &[u8]) -> Result<(), RoqoqoBackendError>;
    ```

    to

    ```rust
    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError>
    ```

    including the `hqslang` name of the operation that changes the device.

### Fixed v0.8.0

* Bug in `wrapped_hqslang` and missing `wrapped_operation` functions in qoqo PragmaChangeDeviceWrapper

## v0.7.0

### Added v0.7.0

* PramgaChangeDevice: A pragma operation acting as a wrapper around device specific Pragmas that can change the device topology.
* change_device interface to Device trait allowing for the modification of Devices by Pragmas

## v0.6.3

### Changed v0.6.3

* Update to rust 2021 edition

### Fixed v0.6.3

* Fix constructing enum MultiQubitGateOperation for all operations implementing OperateMultiQubitGate
* Fixed calculation of superoperator for damping

## v0.6.2

### Changed

* Fixed function signatures in Device trait to uniformly return values instead of references and take references for qubits

## v0.6.1

### Added v0.6.1

* Unittest for the superoperator method of the PragmaGeneralNoise
* NegativeEigenvalue RoqoqoError for matrices that are not positive semi-definite

## v0.6.0

### Added v0.6.0

* Device trait: A minimal trait for quantum computing devices used with roqoqo
* `RoqoqoBackendError` now has a variant `GenericError` for additional backend error types

### Changed v0.6.0

* Rarely used qubit mapping is now the last argument in PragmaRepeatedMeasurement
* PragmaGeneralNoise uses sigma^+ sigma^- and sigma^z as a basis to for Lindblad decoherence rates to avoid using complex rates. Rate and operators parameters of PragmaGeneralNoise have been combined in single parameter rates.

## v0.5.1

### Fixed in roqoqo

* alpha_i function for Tgate

### Fixed in roqoqo_test

* Bugfix measurement selection in stochastic_gate_test

## v0.5.0

### Changed v0.5.0

* Fixed versioning scheme to use the same version number across the project.
* Updated pyo3 dependency to v0.14.1, numpy to v0.14, num-complex to v0.4 and ndarray to v0.15
* Removed sprs dependency to allow update of other dependencies

### Fixed in qoqo

* Wrong Python Class name of ClassicalRegister measurement (was "Cheated")

### Added v0.5.0

* PhaseShiftedControlledZ gate in roqoqo
* QoqoBackendError to use in the python interface of rust based backends
