// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

//! Qoqo quantum operations for quantum computers
//!
//! Quantum programs are represented by linear sequences of quantum operations

mod single_qubit_gate_operations;
pub use single_qubit_gate_operations::*;
mod define_operations;
pub use define_operations::*;
mod pragma_operations;
pub use pragma_operations::*;
mod measurement_operations;
pub use measurement_operations::*;
mod two_qubit_gate_operations;
pub use two_qubit_gate_operations::*;
mod multi_qubit_gate_operations;
pub use multi_qubit_gate_operations::*;
include!(concat!(
    env!("OUT_DIR"),
    "/_auto_generated_operation_conversion.rs"
));
use pyo3::prelude::*;

/// Operations are the atomic instructions in any quantum program that can be represented by qoqo.
///
/// Operations can be of various kinds: Definitions, GateOperations, PRAGMAs or measurement Operations.
///
/// Operations:
///     Definition operations define the classical registers and variables in the Circuit.
///     GateOperations are single-, two- or multi-qubit gate operations that act on a set of qubits
///     and can be executed on a quantum computing device.
///     PRAGMAs are operations that can be used when running a simulation of a quantum computing program.
///     Measurement Operations are operations that perform a measurement either on a quantum computing device (MeasuareQubit)
///     or on a simulation of a quantum computing program (PRAGMA measurement operations).
///
/// .. autosummary::
///    :toctree: generated/
///
///    SingleQubitGate
///    RotateZ
///    RotateY
///    RotateX
///    RotateAroundSphericalAxis
///    PauliZ
///    PauliY
///    PauliX
///    SqrtPauliX
///    InvSqrtPauliX
///    Hadamard
///    TGate
///    SGate
///    DefinitionUsize
///    DefinitionBit
///    DefinitionFloat
///    DefinitionComplex
///    InputSymbolic
///    InputSymbolic
///    MeasureQubit
///    PragmaGetStateVector
///    PragmaGetDensityMatrix
///    PragmaGetOccupationProbability
///    PragmaGetPauliProduct
///    PragmaRepeatedMeasurement
///    PragmaSetNumberOfMeasurements
///    PragmaSetStateVector
///    PragmaSetDensityMatrix
///    PragmaRepeatGate
///    PragmaOverrotation
///    PragmaBoostNoise
///    PragmaStopParallelBlock
///    PragmaGlobalPhase
///    PragmaSleep
///    PragmaActiveReset
///    PragmaStopDecompositionBlock
///    PragmaDamping
///    PragmaDepolarising
///    PragmaDephasing
///    PragmaRandomNoise
///    PragmaGeneralNoise
///    PragmaConditional
///    PragmaLoop
///    CNOT
///    SWAP
///    FSwap
///    ISwap
///    SqrtISwap
///    InvSqrtISwap
///    XY
///    ControlledPhaseShift
///    ControlledPauliY
///    ControlledPauliZ
///    MolmerSorensenXX
///    VariableMSXX
///    GivensRotation
///    GivensRotationLittleEndian
///    Qsim
///    Fsim
///    SpinInteraction
///    Bogoliubov
///    PMInteraction
///    ComplexPMInteraction
///    MultiQubitMS
#[pymodule]

pub fn operations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SingleQubitGateWrapper>()?;
    m.add_class::<RotateZWrapper>()?;
    m.add_class::<RotateYWrapper>()?;
    m.add_class::<RotateXWrapper>()?;
    m.add_class::<RotateXYWrapper>()?;
    m.add_class::<RotateAroundSphericalAxisWrapper>()?;
    m.add_class::<PauliZWrapper>()?;
    m.add_class::<PauliYWrapper>()?;
    m.add_class::<PauliXWrapper>()?;
    m.add_class::<SqrtPauliXWrapper>()?;
    m.add_class::<InvSqrtPauliXWrapper>()?;
    m.add_class::<HadamardWrapper>()?;
    m.add_class::<TGateWrapper>()?;
    m.add_class::<SGateWrapper>()?;
    m.add_class::<DefinitionUsizeWrapper>()?;
    m.add_class::<DefinitionBitWrapper>()?;
    m.add_class::<DefinitionFloatWrapper>()?;
    m.add_class::<DefinitionComplexWrapper>()?;
    m.add_class::<InputSymbolicWrapper>()?;
    m.add_class::<InputBitWrapper>()?;
    m.add_class::<MeasureQubitWrapper>()?;
    m.add_class::<PragmaGetStateVectorWrapper>()?;
    m.add_class::<PragmaGetDensityMatrixWrapper>()?;
    m.add_class::<PragmaGetOccupationProbabilityWrapper>()?;
    m.add_class::<PragmaGetPauliProductWrapper>()?;
    m.add_class::<PragmaRepeatedMeasurementWrapper>()?;
    m.add_class::<PragmaSetNumberOfMeasurementsWrapper>()?;
    m.add_class::<PragmaSetStateVectorWrapper>()?;
    m.add_class::<PragmaSetDensityMatrixWrapper>()?;
    m.add_class::<PragmaRepeatGateWrapper>()?;
    m.add_class::<PragmaOverrotationWrapper>()?;
    m.add_class::<PragmaBoostNoiseWrapper>()?;
    m.add_class::<PragmaStopParallelBlockWrapper>()?;
    m.add_class::<PragmaGlobalPhaseWrapper>()?;
    m.add_class::<PragmaSleepWrapper>()?;
    m.add_class::<PragmaActiveResetWrapper>()?;
    m.add_class::<PragmaStartDecompositionBlockWrapper>()?;
    m.add_class::<PragmaStopDecompositionBlockWrapper>()?;
    m.add_class::<PragmaDampingWrapper>()?;
    m.add_class::<PragmaDepolarisingWrapper>()?;
    m.add_class::<PragmaDephasingWrapper>()?;
    m.add_class::<PragmaRandomNoiseWrapper>()?;
    m.add_class::<PragmaGeneralNoiseWrapper>()?;
    m.add_class::<PragmaConditionalWrapper>()?;
    m.add_class::<PragmaChangeDeviceWrapper>()?;
    m.add_class::<PragmaLoopWrapper>()?;
    m.add_class::<CNOTWrapper>()?;
    m.add_class::<SWAPWrapper>()?;
    m.add_class::<FSwapWrapper>()?;
    m.add_class::<ISwapWrapper>()?;
    m.add_class::<SqrtISwapWrapper>()?;
    m.add_class::<InvSqrtISwapWrapper>()?;
    m.add_class::<XYWrapper>()?;
    m.add_class::<ControlledPhaseShiftWrapper>()?;
    m.add_class::<ControlledPauliYWrapper>()?;
    m.add_class::<ControlledPauliZWrapper>()?;
    m.add_class::<MolmerSorensenXXWrapper>()?;
    m.add_class::<VariableMSXXWrapper>()?;
    m.add_class::<GivensRotationWrapper>()?;
    m.add_class::<GivensRotationLittleEndianWrapper>()?;
    m.add_class::<QsimWrapper>()?;
    m.add_class::<FsimWrapper>()?;
    m.add_class::<SpinInteractionWrapper>()?;
    m.add_class::<BogoliubovWrapper>()?;
    m.add_class::<PMInteractionWrapper>()?;
    m.add_class::<ComplexPMInteractionWrapper>()?;
    m.add_class::<PhaseShiftedControlledZWrapper>()?;
    m.add_class::<PhaseShiftedControlledPhaseWrapper>()?;
    m.add_class::<PhaseShiftState0Wrapper>()?;
    m.add_class::<PhaseShiftState1Wrapper>()?;
    m.add_class::<MultiQubitMSWrapper>()?;
    m.add_class::<MultiQubitZZWrapper>()?;
    Ok(())
}
