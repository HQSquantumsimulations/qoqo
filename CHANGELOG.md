# Changelog

This changelog track changes to the qoqo project starting at version 0.5.0

## Not released

### Added

* QuantumProgram: A representation of a quantum program that accepts a list of free classical float parameters,
runs measurements on a backend and returns expectation values or the classical register output of the quantum circuits.
QuantumProgram is intended as the main interface between classical software and roqoqo quantum programs.  

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
