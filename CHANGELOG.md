# Changelog

This changelog track changes to the qoqo project starting at version 0.5.0

## Not released

## 0.5.1

### Fixed in roqoqo

* alpha_i function for Tgate

### Fixed in roqoqo_test

* Bugfix measurement selection in stochastic_gate_test

## 0.5.0

### Changed

* Fixed versioning scheme to use the same version number across the project.
* Updated pyo3 dependency to 0.14.1, numpy to 0.14, num-complex to 0.4 and ndarray to 0.15
* Removed sprs dependency to allow update of other dependencies

### Fixed in qoqo

* Wrong Python Class name of ClassicalRegister measurement (was "Cheated")

### Added

* PhaseShiftedControlledZ gate in roqoqo
* QoqoBackendError to use in the python interface of rust based backends
