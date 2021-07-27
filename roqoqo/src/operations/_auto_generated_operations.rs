use crate::operations::*;
#[doc = r" Enum of all Operations implementing [Operate]"]
#[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, Substitute)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum Operation {
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SingleQubitGate"]
    SingleQubitGate(SingleQubitGate),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateZ"]
    RotateZ(RotateZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateX"]
    RotateX(RotateX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateY"]
    RotateY(RotateY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PauliX"]
    PauliX(PauliX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PauliY"]
    PauliY(PauliY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PauliZ"]
    PauliZ(PauliZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SqrtPauliX"]
    SqrtPauliX(SqrtPauliX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for InvSqrtPauliX"]
    InvSqrtPauliX(InvSqrtPauliX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Hadamard"]
    Hadamard(Hadamard),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SGate"]
    SGate(SGate),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for TGate"]
    TGate(TGate),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftState0"]
    PhaseShiftState0(PhaseShiftState0),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftState1"]
    PhaseShiftState1(PhaseShiftState1),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateAroundSphericalAxis"]
    RotateAroundSphericalAxis(RotateAroundSphericalAxis),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaSetNumberOfMeasurements"]
    PragmaSetNumberOfMeasurements(PragmaSetNumberOfMeasurements),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaSetStateVector"]
    PragmaSetStateVector(PragmaSetStateVector),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaSetDensityMatrix"]
    PragmaSetDensityMatrix(PragmaSetDensityMatrix),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaRepeatGate"]
    PragmaRepeatGate(PragmaRepeatGate),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaOverrotation"]
    PragmaOverrotation(PragmaOverrotation),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaBoostNoise"]
    PragmaBoostNoise(PragmaBoostNoise),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaStopParallelBlock"]
    PragmaStopParallelBlock(PragmaStopParallelBlock),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaGlobalPhase"]
    PragmaGlobalPhase(PragmaGlobalPhase),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaSleep"]
    PragmaSleep(PragmaSleep),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaActiveReset"]
    PragmaActiveReset(PragmaActiveReset),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaStartDecompositionBlock"]
    PragmaStartDecompositionBlock(PragmaStartDecompositionBlock),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaStopDecompositionBlock"]
    PragmaStopDecompositionBlock(PragmaStopDecompositionBlock),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaDamping"]
    PragmaDamping(PragmaDamping),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaDepolarising"]
    PragmaDepolarising(PragmaDepolarising),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaDephasing"]
    PragmaDephasing(PragmaDephasing),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaRandomNoise"]
    PragmaRandomNoise(PragmaRandomNoise),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaGeneralNoise"]
    PragmaGeneralNoise(PragmaGeneralNoise),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaConditional"]
    PragmaConditional(PragmaConditional),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for CNOT"]
    CNOT(CNOT),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SWAP"]
    SWAP(SWAP),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ISwap"]
    ISwap(ISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for FSwap"]
    FSwap(FSwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SqrtISwap"]
    SqrtISwap(SqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for InvSqrtISwap"]
    InvSqrtISwap(InvSqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for XY"]
    XY(XY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPhaseShift"]
    ControlledPhaseShift(ControlledPhaseShift),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliY"]
    ControlledPauliY(ControlledPauliY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliZ"]
    ControlledPauliZ(ControlledPauliZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for MolmerSorensenXX"]
    MolmerSorensenXX(MolmerSorensenXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for VariableMSXX"]
    VariableMSXX(VariableMSXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotation"]
    GivensRotation(GivensRotation),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotationLittleEndian"]
    GivensRotationLittleEndian(GivensRotationLittleEndian),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Qsim"]
    Qsim(Qsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Fsim"]
    Fsim(Fsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SpinInteraction"]
    SpinInteraction(SpinInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Bogoliubov"]
    Bogoliubov(Bogoliubov),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PMInteraction"]
    PMInteraction(PMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ComplexPMInteraction"]
    ComplexPMInteraction(ComplexPMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftedControlledZ"]
    PhaseShiftedControlledZ(PhaseShiftedControlledZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for MeasureQubit"]
    MeasureQubit(MeasureQubit),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaGetStateVector"]
    PragmaGetStateVector(PragmaGetStateVector),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaGetDensityMatrix"]
    PragmaGetDensityMatrix(PragmaGetDensityMatrix),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaGetOccupationProbability"]
    PragmaGetOccupationProbability(PragmaGetOccupationProbability),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaGetPauliProduct"]
    PragmaGetPauliProduct(PragmaGetPauliProduct),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PragmaRepeatedMeasurement"]
    PragmaRepeatedMeasurement(PragmaRepeatedMeasurement),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for DefinitionFloat"]
    DefinitionFloat(DefinitionFloat),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for DefinitionComplex"]
    DefinitionComplex(DefinitionComplex),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for DefinitionUsize"]
    DefinitionUsize(DefinitionUsize),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for DefinitionBit"]
    DefinitionBit(DefinitionBit),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for InputSymbolic"]
    InputSymbolic(InputSymbolic),
}
#[doc = r" Enum of all Operations implementing [OperateSingleQubit]"]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperateSingleQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum SingleQubitOperation {
    #[doc = "Variant for SingleQubitGate"]
    SingleQubitGate(SingleQubitGate),
    #[doc = "Variant for RotateZ"]
    RotateZ(RotateZ),
    #[doc = "Variant for RotateX"]
    RotateX(RotateX),
    #[doc = "Variant for RotateY"]
    RotateY(RotateY),
    #[doc = "Variant for PauliX"]
    PauliX(PauliX),
    #[doc = "Variant for PauliY"]
    PauliY(PauliY),
    #[doc = "Variant for PauliZ"]
    PauliZ(PauliZ),
    #[doc = "Variant for SqrtPauliX"]
    SqrtPauliX(SqrtPauliX),
    #[doc = "Variant for InvSqrtPauliX"]
    InvSqrtPauliX(InvSqrtPauliX),
    #[doc = "Variant for Hadamard"]
    Hadamard(Hadamard),
    #[doc = "Variant for SGate"]
    SGate(SGate),
    #[doc = "Variant for TGate"]
    TGate(TGate),
    #[doc = "Variant for PhaseShiftState0"]
    PhaseShiftState0(PhaseShiftState0),
    #[doc = "Variant for PhaseShiftState1"]
    PhaseShiftState1(PhaseShiftState1),
    #[doc = "Variant for RotateAroundSphericalAxis"]
    RotateAroundSphericalAxis(RotateAroundSphericalAxis),
    #[doc = "Variant for PragmaActiveReset"]
    PragmaActiveReset(PragmaActiveReset),
    #[doc = "Variant for PragmaDamping"]
    PragmaDamping(PragmaDamping),
    #[doc = "Variant for PragmaDepolarising"]
    PragmaDepolarising(PragmaDepolarising),
    #[doc = "Variant for PragmaDephasing"]
    PragmaDephasing(PragmaDephasing),
    #[doc = "Variant for PragmaRandomNoise"]
    PragmaRandomNoise(PragmaRandomNoise),
    #[doc = "Variant for PragmaGeneralNoise"]
    PragmaGeneralNoise(PragmaGeneralNoise),
    #[doc = "Variant for MeasureQubit"]
    MeasureQubit(MeasureQubit),
}
#[doc = r" Enum of all Operations implementing [OperateTwoQubit]"]
#[derive(
    Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateTwoQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum TwoQubitOperation {
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for CNOT"]
    CNOT(CNOT),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SWAP"]
    SWAP(SWAP),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ISwap"]
    ISwap(ISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for FSwap"]
    FSwap(FSwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SqrtISwap"]
    SqrtISwap(SqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for InvSqrtISwap"]
    InvSqrtISwap(InvSqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for XY"]
    XY(XY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPhaseShift"]
    ControlledPhaseShift(ControlledPhaseShift),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliY"]
    ControlledPauliY(ControlledPauliY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliZ"]
    ControlledPauliZ(ControlledPauliZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for MolmerSorensenXX"]
    MolmerSorensenXX(MolmerSorensenXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for VariableMSXX"]
    VariableMSXX(VariableMSXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotation"]
    GivensRotation(GivensRotation),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotationLittleEndian"]
    GivensRotationLittleEndian(GivensRotationLittleEndian),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Qsim"]
    Qsim(Qsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Fsim"]
    Fsim(Fsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SpinInteraction"]
    SpinInteraction(SpinInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Bogoliubov"]
    Bogoliubov(Bogoliubov),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PMInteraction"]
    PMInteraction(PMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ComplexPMInteraction"]
    ComplexPMInteraction(ComplexPMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftedControlledZ"]
    PhaseShiftedControlledZ(PhaseShiftedControlledZ),
}
#[doc = r" Enum of all Operations implementing [OperateMultiQubit]"]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperateMultiQubit,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiQubitOperation {
    #[doc = "Variant for PragmaOverrotation"]
    PragmaOverrotation(PragmaOverrotation),
    #[doc = "Variant for PragmaStopParallelBlock"]
    PragmaStopParallelBlock(PragmaStopParallelBlock),
    #[doc = "Variant for PragmaSleep"]
    PragmaSleep(PragmaSleep),
    #[doc = "Variant for PragmaStartDecompositionBlock"]
    PragmaStartDecompositionBlock(PragmaStartDecompositionBlock),
    #[doc = "Variant for PragmaStopDecompositionBlock"]
    PragmaStopDecompositionBlock(PragmaStopDecompositionBlock),
}
#[doc = r" Enum of all Operations implementing [OperatePragma]"]
#[derive(
    Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperatePragma,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum PragmaOperation {
    #[doc = "Variant for PragmaSetNumberOfMeasurements"]
    PragmaSetNumberOfMeasurements(PragmaSetNumberOfMeasurements),
    #[doc = "Variant for PragmaSetStateVector"]
    PragmaSetStateVector(PragmaSetStateVector),
    #[doc = "Variant for PragmaSetDensityMatrix"]
    PragmaSetDensityMatrix(PragmaSetDensityMatrix),
    #[doc = "Variant for PragmaRepeatGate"]
    PragmaRepeatGate(PragmaRepeatGate),
    #[doc = "Variant for PragmaOverrotation"]
    PragmaOverrotation(PragmaOverrotation),
    #[doc = "Variant for PragmaBoostNoise"]
    PragmaBoostNoise(PragmaBoostNoise),
    #[doc = "Variant for PragmaStopParallelBlock"]
    PragmaStopParallelBlock(PragmaStopParallelBlock),
    #[doc = "Variant for PragmaGlobalPhase"]
    PragmaGlobalPhase(PragmaGlobalPhase),
    #[doc = "Variant for PragmaSleep"]
    PragmaSleep(PragmaSleep),
    #[doc = "Variant for PragmaActiveReset"]
    PragmaActiveReset(PragmaActiveReset),
    #[doc = "Variant for PragmaStartDecompositionBlock"]
    PragmaStartDecompositionBlock(PragmaStartDecompositionBlock),
    #[doc = "Variant for PragmaStopDecompositionBlock"]
    PragmaStopDecompositionBlock(PragmaStopDecompositionBlock),
    #[doc = "Variant for PragmaDamping"]
    PragmaDamping(PragmaDamping),
    #[doc = "Variant for PragmaDepolarising"]
    PragmaDepolarising(PragmaDepolarising),
    #[doc = "Variant for PragmaDephasing"]
    PragmaDephasing(PragmaDephasing),
    #[doc = "Variant for PragmaRandomNoise"]
    PragmaRandomNoise(PragmaRandomNoise),
    #[doc = "Variant for PragmaGeneralNoise"]
    PragmaGeneralNoise(PragmaGeneralNoise),
    #[doc = "Variant for PragmaConditional"]
    PragmaConditional(PragmaConditional),
    #[doc = "Variant for PragmaGetStateVector"]
    PragmaGetStateVector(PragmaGetStateVector),
    #[doc = "Variant for PragmaGetDensityMatrix"]
    PragmaGetDensityMatrix(PragmaGetDensityMatrix),
    #[doc = "Variant for PragmaGetOccupationProbability"]
    PragmaGetOccupationProbability(PragmaGetOccupationProbability),
    #[doc = "Variant for PragmaGetPauliProduct"]
    PragmaGetPauliProduct(PragmaGetPauliProduct),
    #[doc = "Variant for PragmaRepeatedMeasurement"]
    PragmaRepeatedMeasurement(PragmaRepeatedMeasurement),
}
#[doc = r" Enum of all Operations implementing [OperatePragmaNoise]"]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperatePragma,
    OperatePragmaNoise,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum PragmaNoiseOperation {
    #[doc = "Variant for PragmaDamping"]
    PragmaDamping(PragmaDamping),
    #[doc = "Variant for PragmaDepolarising"]
    PragmaDepolarising(PragmaDepolarising),
    #[doc = "Variant for PragmaDephasing"]
    PragmaDephasing(PragmaDephasing),
    #[doc = "Variant for PragmaRandomNoise"]
    PragmaRandomNoise(PragmaRandomNoise),
}
#[doc = r" Enum of all Operations implementing [OperateGate]"]
#[derive(
    Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateGate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum GateOperation {
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SingleQubitGate"]
    SingleQubitGate(SingleQubitGate),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateZ"]
    RotateZ(RotateZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateX"]
    RotateX(RotateX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateY"]
    RotateY(RotateY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PauliX"]
    PauliX(PauliX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PauliY"]
    PauliY(PauliY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PauliZ"]
    PauliZ(PauliZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SqrtPauliX"]
    SqrtPauliX(SqrtPauliX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for InvSqrtPauliX"]
    InvSqrtPauliX(InvSqrtPauliX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Hadamard"]
    Hadamard(Hadamard),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SGate"]
    SGate(SGate),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for TGate"]
    TGate(TGate),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftState0"]
    PhaseShiftState0(PhaseShiftState0),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftState1"]
    PhaseShiftState1(PhaseShiftState1),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for RotateAroundSphericalAxis"]
    RotateAroundSphericalAxis(RotateAroundSphericalAxis),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for CNOT"]
    CNOT(CNOT),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SWAP"]
    SWAP(SWAP),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ISwap"]
    ISwap(ISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for FSwap"]
    FSwap(FSwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SqrtISwap"]
    SqrtISwap(SqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for InvSqrtISwap"]
    InvSqrtISwap(InvSqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for XY"]
    XY(XY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPhaseShift"]
    ControlledPhaseShift(ControlledPhaseShift),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliY"]
    ControlledPauliY(ControlledPauliY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliZ"]
    ControlledPauliZ(ControlledPauliZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for MolmerSorensenXX"]
    MolmerSorensenXX(MolmerSorensenXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for VariableMSXX"]
    VariableMSXX(VariableMSXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotation"]
    GivensRotation(GivensRotation),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotationLittleEndian"]
    GivensRotationLittleEndian(GivensRotationLittleEndian),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Qsim"]
    Qsim(Qsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Fsim"]
    Fsim(Fsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SpinInteraction"]
    SpinInteraction(SpinInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Bogoliubov"]
    Bogoliubov(Bogoliubov),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PMInteraction"]
    PMInteraction(PMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ComplexPMInteraction"]
    ComplexPMInteraction(ComplexPMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftedControlledZ"]
    PhaseShiftedControlledZ(PhaseShiftedControlledZ),
}
#[doc = r" Enum of all Operations implementing [Rotate]"]
#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperateGate,
    Rotate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum Rotation {
    #[doc = "Variant for RotateZ"]
    RotateZ(RotateZ),
    #[doc = "Variant for RotateX"]
    RotateX(RotateX),
    #[doc = "Variant for RotateY"]
    RotateY(RotateY),
    #[doc = "Variant for PhaseShiftState0"]
    PhaseShiftState0(PhaseShiftState0),
    #[doc = "Variant for PhaseShiftState1"]
    PhaseShiftState1(PhaseShiftState1),
    #[doc = "Variant for RotateAroundSphericalAxis"]
    RotateAroundSphericalAxis(RotateAroundSphericalAxis),
    #[doc = "Variant for XY"]
    XY(XY),
    #[doc = "Variant for ControlledPhaseShift"]
    ControlledPhaseShift(ControlledPhaseShift),
    #[doc = "Variant for VariableMSXX"]
    VariableMSXX(VariableMSXX),
    #[doc = "Variant for GivensRotation"]
    GivensRotation(GivensRotation),
    #[doc = "Variant for GivensRotationLittleEndian"]
    GivensRotationLittleEndian(GivensRotationLittleEndian),
}
#[doc = r" Enum of all Operations implementing [Define]"]
#[derive(
    Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, Define,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum Definition {
    #[doc = "Variant for DefinitionFloat"]
    DefinitionFloat(DefinitionFloat),
    #[doc = "Variant for DefinitionComplex"]
    DefinitionComplex(DefinitionComplex),
    #[doc = "Variant for DefinitionUsize"]
    DefinitionUsize(DefinitionUsize),
    #[doc = "Variant for DefinitionBit"]
    DefinitionBit(DefinitionBit),
    #[doc = "Variant for InputSymbolic"]
    InputSymbolic(InputSymbolic),
}
#[doc = r" Enum of all Operations implementing [OperateConstantGate]"]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperateGate,
    OperateConstantGate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstantGateOperation {}
#[doc = r" Enum of all Operations implementing [OperateSingleQubitGate]"]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperateGate,
    OperateSingleQubit,
    OperateSingleQubitGate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum SingleQubitGateOperation {
    #[doc = "Variant for SingleQubitGate"]
    SingleQubitGate(SingleQubitGate),
    #[doc = "Variant for RotateZ"]
    RotateZ(RotateZ),
    #[doc = "Variant for RotateX"]
    RotateX(RotateX),
    #[doc = "Variant for RotateY"]
    RotateY(RotateY),
    #[doc = "Variant for PauliX"]
    PauliX(PauliX),
    #[doc = "Variant for PauliY"]
    PauliY(PauliY),
    #[doc = "Variant for PauliZ"]
    PauliZ(PauliZ),
    #[doc = "Variant for SqrtPauliX"]
    SqrtPauliX(SqrtPauliX),
    #[doc = "Variant for InvSqrtPauliX"]
    InvSqrtPauliX(InvSqrtPauliX),
    #[doc = "Variant for Hadamard"]
    Hadamard(Hadamard),
    #[doc = "Variant for SGate"]
    SGate(SGate),
    #[doc = "Variant for TGate"]
    TGate(TGate),
    #[doc = "Variant for PhaseShiftState0"]
    PhaseShiftState0(PhaseShiftState0),
    #[doc = "Variant for PhaseShiftState1"]
    PhaseShiftState1(PhaseShiftState1),
    #[doc = "Variant for RotateAroundSphericalAxis"]
    RotateAroundSphericalAxis(RotateAroundSphericalAxis),
}
#[doc = r" Enum of all Operations implementing [OperateTwoQubitGate]"]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperateGate,
    OperateTwoQubit,
    OperateTwoQubitGate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum TwoQubitGateOperation {
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for CNOT"]
    CNOT(CNOT),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SWAP"]
    SWAP(SWAP),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ISwap"]
    ISwap(ISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for FSwap"]
    FSwap(FSwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SqrtISwap"]
    SqrtISwap(SqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for InvSqrtISwap"]
    InvSqrtISwap(InvSqrtISwap),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for XY"]
    XY(XY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPhaseShift"]
    ControlledPhaseShift(ControlledPhaseShift),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliY"]
    ControlledPauliY(ControlledPauliY),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ControlledPauliZ"]
    ControlledPauliZ(ControlledPauliZ),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for MolmerSorensenXX"]
    MolmerSorensenXX(MolmerSorensenXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for VariableMSXX"]
    VariableMSXX(VariableMSXX),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotation"]
    GivensRotation(GivensRotation),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for GivensRotationLittleEndian"]
    GivensRotationLittleEndian(GivensRotationLittleEndian),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Qsim"]
    Qsim(Qsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Fsim"]
    Fsim(Fsim),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for SpinInteraction"]
    SpinInteraction(SpinInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for Bogoliubov"]
    Bogoliubov(Bogoliubov),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PMInteraction"]
    PMInteraction(PMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for ComplexPMInteraction"]
    ComplexPMInteraction(ComplexPMInteraction),
    #[allow(clippy::upper_case_acronyms)]
    #[doc = "Variant for PhaseShiftedControlledZ"]
    PhaseShiftedControlledZ(PhaseShiftedControlledZ),
}
#[doc = r" Enum of all Operations implementing [OperateMultiQubitGate]"]
#[derive(
    Debug,
    Clone,
    PartialEq,
    InvolveQubits,
    Operate,
    OperateTryFromEnum,
    Substitute,
    OperateGate,
    OperateMultiQubit,
    OperateMultiQubitGate,
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiQubitGateOperation {}
