# Changelog

This changelog track changes to the qoqo project starting at version v0.5.0

## Unreleased

* Updated to pyo3 0.23, except the IntoPy and ToPyObject deprecation warnings.
* Updated the noise models to output struqture 2.0 or 1.0 based on installed struqture-py version.
* Updated badges in top-level readme.
* Added qoqo/.cargo/config file with aarch64 and x86_64 targets for macos.

## 1.18.1

## Updated in 1.18.1

* Updated dependencies which were out of date.
* Updated github actions to latest versions.

## Fixed 1.18.1

* Fixed new clippy warnings.

## 1.18.0

### Fixed 1.18.0

* Fixed documentation building by hardcoding the paths.

### Added in 1.18.0

* Updated minimum supported Rust version from 1.70 to 1.76.
* Updated minimum supported Python version from 3.8 to 3.9.
* Updated to struqture 1.11.1.
* Updated to qoqo-calculator 1.4.

## 1.17.0

### Added in 1.17.0

* Added the documentation from qoqo_examples.
* Added `PragmaSimulationRepetitions` operation.
* Updated to pyo3 0.22, thereby adding support for python 3.13.

## 1.16.1

### Fixed in 1.16.1

* Fixed `PhaseShiftedControlledControlledZ` and `PhaseShiftedControlledControlledPhase` unitary matrices.

## 1.16.0

### Fixed in 1.16.0

* Updated nalgebra to 0.33.1, jsonschema to 0.23. Removed unnecessary dependencies.

### Added in 1.16.0

* Added `InvSGate`, `InvTGate`, `SXGate`, `InvSXGate`, `TripleControlledPauliX`, `TripleControlledPauliZ`, `TripleControlledPhaseShift`, `ControlledSWAP`, `PhaseShiftedControlledControlledZ`, `PhaseShiftedControlledControlledPhase` gates.

## 1.15.2

### Fixed in 1.15.2

* Fixed release bug.
* Fixed sphinx documentation and compatibility test 1.14.
* Renamed the readme in roqoqo.

### Added in 1.15.2

* Added support for noise model operator input from struqture 2.x. Output remains in struqture 1.x.
* Pinned clap version to use rust 1.70 (clap,clap_builder and clap_derive 4.4, clap_lex 0.6.0).
* Updated to rust 1.70.
* Updated to jsonschema 0.20.
* Added list of available gates names.

## 1.15.1

### Fixed in 1.15.1

* Fixed package category.

## 1.15.0

### Added in 1.15.0

* Added `SqrtPauliY` and `InvSqrtPauliY` gates.

## 1.14.0

### Added in 1.14.0

* Added the doc_generator feature to qoqo that will create .pyi used for python IDE hints during cargo build.
* Change version mismatch error message to be clearer.
* `PragmaSetStateVector`, `PragmaSetDensityMatrix` and `PragmaGeneralNoise` interface casting errors when handling arrays/matrices
* Dependencies issues caused by Pyo3 0.21 support release

## 1.13.0

### Added in 1.13.0

* Unstable feature `unstable_operation_definition` adding new operations: GateDefinition and CallDefinedGate.

## 1.12.1

### Added in 1.12.1

* Added the `__repr__` method to the SingleQubitOverrotationDescription noise model.

## 1.12.0

### Added in 1.12.0

* Updated to pyo3 0.21.
* Fix typo in name of RoqoqoBackendError::AuthenticationError

## 1.11.0

### Added in 1.11.0

* Added the DecoherenceOnIdle noise model
* Feature `unstable_analog_operations` adding new analog operations: `ApplyConstantSpinHamiltonian`, `ApplyTimeDependentSpinHamiltonian`
* Added `SingleQubitOverrotationDescription`
* Added `SingleQubitOverrotationOnGate` noise model
* Updated cargo (including updating mio from v0.8.10 to v0.8.11)

## 1.10.0

### Added in 1.10.0

* Unstable feature `unstable_spin_boson_operations` adding new spin-boson operations: QuantumRabi, LongitudinalCoupling, JaynesCummings
* Added three new operations to `unstable_spin_boson_operations`: SingleExcitationLoad, SingleExcitationStore, CZQubitResonator

### Fixed in 1.9.2

* Error message in `Device.change_device()`

## 1.9.1

### Fixed in 1.9.1

* Unstable feature `unstable_chain_with_environment` macros have been moved to their own feature instead of incorporating the feature gate into the macros.

## 1.9.0

### Added in 1.9.0

* Unstable feature `unstable_chain_with_environment` implementing a Trait for returning chains in a device with an environment of qubits, where an environment are qubits that are not part of the chain but connected to at least one qubit in the chain.

## 1.8.0

### Added in 1.8.0

* Updated to pyo3 0.20.
* Added `PragmaAnnotatedOp` operation.
* Added EchoCrossResonance gate.
* Added PhaseDisplacement gate.

## 1.7.1

### Fixed in 1.7.1

* Fixed missing `Identity` gate from operations module

## 1.7.0

### Fixed in 1.7.0

* Renamed unstable ErrorOnGate NoiseModel to DecoherenceOnGate
* Fixed the factors of the depolarising rate of ContinuousDecoherenceModel by dividing by 2 (now equivalent to GenericDevice)

### Added in 1.7.0

* Added `Identity` gate
* NoiseModels are now a stable feature

## 1.6.2

### Added in 1.6.2

* Added the json_schema feature info to the NoiseModels (unstable)

## 1.6.1

### Fixed in 1.6.1

* Fixed serialisation of unstable error-on-gate noise model to json

## 1.6.0

### Added in 1.6.0

* Added support for JsonSchema.
* Added first bosonic operations traits in `roqoqo` and `qoqo`.

## 1.5.1

### Fixed in 1.5.1

* Fixed wrong minimum required roqoqo version when serializing circuits with some operations from versions 1.3, 1.4. and 1.5.

### Updated in 1.5.1

* pyo3 updated to 0.19

## 1.5.0

### Added in 1.5.0

* `PragmaControlledCircuit` operation for a Circuit that is controlled by a single qubit.

## 1.4.0

### Fixed in 1.4.0

* Added non-exhaustive attribute to enums where it was obmitted by mistake
* Fixed semver violation in Device trait

### Added in 1.4.0

* Added GPi and GPi2 gates

## 1.3.2

### Fixed in 1.3.2

* Measurement inputs can now be passed properly in Python interface (previously, there were serialisation functions missing)

## 1.3.1

### Fixed in 1.3.1

* Devices are no longer serialized with wrong roqoqo version number

## 1.3.0

### Added in 1.3.0

* Added Three-qubit gates support
* Added ControlledRotateX, ControlledRotateXY, ControlledControlledPauliZ, ControlledControlledPhaseShift, Toffoli to qoqo
* Added device function to return all gate names
* Added unstable QoqoDevice feature. Prototype for future qoqo devices without stability guarantees.

## Fixed in 1.3.0

* Wrong angle in circuit decomposition of MultiQubitZZ and MultiQubitMS

## 1.2.5

* Updated to pyo3 0.18.1

## 1.2.4

* Updated to pyo3 0.18, qoqo_calculator 1.1.1 and nalgebra 0.32

## 1.2.3

* Modified the downcast method for measurements passed to the QuantumProgram in qoqo
* Added to_bincode and from_bincode methods to all measurements in qoqo

## 1.2.2

* Removed references to `pyo3::prepare_freethreaded_python();` outside of tests

## 1.2.0

* Activated circuitdag feature by default
* Modified serialization roqoqo version to use lowest compatible version
* Relaxed compatability check for Python arguments to allow backward compatability
* Added CircuitDag documentation
* Added PhaseShiftedControlledPhase to qoqo
* Updated dependencies
* Updated nalgebra to 0.31

## 1.1.0

### Changed v1.1.0

* Fixed nalgebra version to 0.30

### Added v1.1.0

* Added 1.0.0 compatibility tests
* Added rich comparison for Python interface of Measurements
* Added PragmaLoop
* Allowed creating PragmaSetStateVector from float or integer numpy arrays.
* Added `InputBit` to set bit in a (readout) bit register to a value.
* Added `InvolvedClassical` functionality to return which classical register variables are involved in an operation
* Added `CircuitDag` direct acyclical graph representation of `Circuit`

## v1.0.0

### Fixed v1.0.0

* Fixed superoperator construction for general noise pragma
* Updated dependencies
* `PragmaRepeatedMeasurement` now adds remapped qubits that are not previously in the qubit_mapping of the gate.

### Added v1.0.0

* Optional `async` feature including:
    1. AsyncEvaluatingBackend trait for backends that implement async evaluation (especially designed for Backends interfacing a Web-API)
    2. Measurements async evaluating the Future of a register measurement returned from a backend.
* Device trait added in roqoqo.
* Added unit tests for serialization of PragmaRepeatedMeasurement operations in a Circuit.

### Changed v1.0.0

* Make qubit_remapping more lenient only remapping values found in a HashMap skipping remapping for qubits not found in HashMap instead of returning an error.
* Updated to qoqo_calculator 1.0
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
