# Changelog

This changelog track changes to the qoqo project starting at version 0.5.0

## Not released

### Added

## 0.9.1

### Changed 0.9.1

* Increased tolerance for unitary violation when construction unitary matrix for SingleQubitGate from `f64::EPSILON` to `1e-6`.
* Semver-style version checking for Circuit serialization. In beta mode (0.y.z) minor version must match (y_library == y_data) in release mode (x.y.z) major version must match (x_library == x_data) and minor version of library must exceed minor version of data (y_library >= y_data).

## 0.9.0

### Fixed 0.9.0

* Bug in the probability function of the PragmaDamping gate

### Added 0.9.0

* MultiQubitZZ gate. Rotation under a multi-qubit product of Pauli Z operators.
* `two_qubit_edges` function in Device trait. Used to create a simple graph-library-agnostic representation of the connectivity graph of a device.

## 0.8.1

### Changed 0.8.1

* Updated to pyo3 0.15.0

## 0.8.0

### Added 0.8.0

* QuantumProgram: A representation of a quantum program that accepts a list of free classical float parameters,
runs measurements on a backend and returns expectation values or the classical register output of the quantum circuits.
QuantumProgram is intended as the main interface between classical software and roqoqo quantum programs.  

### Changed 0.8.0

* In the Device Trait the `change_device` function changed the signature from

    ```rust
    fn change_device(&mut self, operation: &[u8]) -> Result<(), RoqoqoBackendError>;
    ```

    to

    ```rust
    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError>
    ```

    including the `hqslang` name of the operation that changes the device.

### Fixed 0.8.0

* Bug in `wrapped_hqslang` and missing `wrapped_operation` functions in qoqo PragmaChangeDeviceWrapper

## 0.7.0

### Added 0.7.0

* PramgaChangeDevice: A pragma operation acting as a wrapper around device specific Pragmas that can change the device topology.
* change_device interface to Device trait allowing for the modification of Devices by Pragmas

## 0.6.3

### Changed 0.6.3

* Update to rust 2021 edition

### Fixed 0.6.3

* Fix constructing enum MultiQubitGateOperation for all operations implementing OperateMultiQubitGate
* Fixed calculation of superoperator for damping

## 0.6.2

### Changed

* Fixed function signatures in Device trait to uniformly return values instead of references and take references for qubits

## 0.6.1

### Added 0.6.1

* Unittest for the superoperator method of the PragmaGeneralNoise
* NegativeEigenvalue RoqoqoError for matrices that are not positive semi-definite

## 0.6.0

### Added 0.6.0

* Device trait: A minimal trait for quantum computing devices used with roqoqo
* `RoqoqoBackendError` now has a variant `GenericError` for additional backend error types

### Changed 0.6.0

* Rarely used qubit mapping is now the last argument in PragmaRepeatedMeasurement
* PragmaGeneralNoise uses sigma^+ sigma^- and sigma^z as a basis to for Lindblad decoherence rates to avoid using complex rates. Rate and operators parameters of PragmaGeneralNoise have been combined in single parameter rates.

## 0.5.1

### Fixed in roqoqo

* alpha_i function for Tgate

### Fixed in roqoqo_test

* Bugfix measurement selection in stochastic_gate_test

## 0.5.0

### Changed 0.5.0

* Fixed versioning scheme to use the same version number across the project.
* Updated pyo3 dependency to 0.14.1, numpy to 0.14, num-complex to 0.4 and ndarray to 0.15
* Removed sprs dependency to allow update of other dependencies

### Fixed in qoqo

* Wrong Python Class name of ClassicalRegister measurement (was "Cheated")

### Added 0.5.0

* PhaseShiftedControlledZ gate in roqoqo
* QoqoBackendError to use in the python interface of rust based backends
