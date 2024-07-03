// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Test that involved classical behaves as expected

//use crate::RoqoqoError::{CalculatorError, UnitaryMatrixErrror};
use ndarray::array;
use num_complex::Complex64;
use qoqo_calculator::CalculatorFloat;
use roqoqo::{operations::*, Circuit};
use std::collections::HashMap;
use std::{collections::HashSet, f64::consts::PI};
use test_case::test_case;

#[test_case(SingleQubitGateOperation::from(RotateZ::new(0, CalculatorFloat::from("theta"))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from("theta"))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from("theta"))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from("theta"),
    CalculatorFloat::from("spherical_theta"),
    CalculatorFloat::from("spherical_phi"))); "Rotation")]
#[test_case(SingleQubitGateOperation::from(RotateXY::new(
    0,
    CalculatorFloat::from("theta"),
    CalculatorFloat::from("phi"))); "RotationXY")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from("PI/2.0"))); "phaseshiftstate0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from("PI/2.0"))); "phaseshiftstate1")]
fn test_to_single_qubit_gate_symbolic(operation: SingleQubitGateOperation) {
    let gate = SingleQubitGate::new(
        0,
        operation.alpha_r(),
        operation.alpha_i(),
        operation.beta_r(),
        operation.beta_i(),
        operation.global_phase(),
    );
    let gate_test: SingleQubitGate = operation.to_single_qubit_gate();
    assert_eq!(gate, gate_test);
}

/// Test 'to_single_qubit_gate()` for all SingleQubitGateOperations
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(PI/3.0))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(PI/3.0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI/3.0))); "RotateY")]
#[test_case(Operation::from(PauliX::new(0)); "PauliX")]
#[test_case(Operation::from(PauliY::new(0)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(0)); "SGate")]
#[test_case(Operation::from(TGate::new(0)); "TGate")]
#[test_case(Operation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(Operation::from(RotateAroundSphericalAxis::new(
    0,
    CalculatorFloat::from(PI/3.0),
    CalculatorFloat::from(PI/4.0),
    CalculatorFloat::from(PI/2.0))); "Rotation")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI/2.0))); "phaseshiftstate1")]
#[test_case(Operation::from(RotateXY::new(
    0,
    CalculatorFloat::from(PI/3.0),
    CalculatorFloat::from(PI/4.0))); "RotationXY")]
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "Swap")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::FRAC_PI_2)); "VariableMSXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX_pi")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::PI, CalculatorFloat::PI, CalculatorFloat::PI)); "Fsim_PI")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI)); "PhaseShiftedControlledZ_pi")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::ZERO)); "PhaseShiftedControlledZ_zero")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from(PI/(-3.0)))); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PragmaSetStateVector::new(array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ])); "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new( array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ])); "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(2)); "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)); "PragmaOverrotation")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))); "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))); "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], get_reordering())); "PragmaStartDecompositionBlock")]
#[test_case(Operation::from( PragmaStopDecompositionBlock::new(vec![0, 1])); "PragmaStopDecompositionBlock")]
#[test_case(Operation::from( PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(Operation::from(  PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(Operation::from( PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
#[test_case(Operation::from( PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    )); "PragmaRandomNoise")]
#[test_case(Operation::from( PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), array![[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0],])); "PragmaGeneralNoise")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
fn none_involved_classical(operation: Operation) {
    assert_eq!(operation.involved_classical(), InvolvedClassical::None);
}

#[test_case(Operation::from( PragmaRepeatedMeasurement::new("ro".to_string(), 20, None)), InvolvedClassical::AllQubits("ro".to_string()); "PragmaRepeatedMeasurement - none")]

fn involved_classical_all_qubits(operation: Operation, involved: InvolvedClassical) {
    assert_eq!(operation.involved_classical(), involved);
}

#[test_case(Operation::from( PragmaRepeatedMeasurement::new("out".to_string(), 20, Some(get_reordering()))), vec![("out", 0), ("out", 1)]; "PragmaRepeatedMeasurement")]
#[test_case(Operation::from( MeasureQubit::new(0, "out".to_string(), 1)), vec![("out", 1)]; "MeasuareQubit")]
#[test_case(Operation::from( PragmaRepeatedMeasurement::new("out".to_string(), 20, Some(get_reordering()))), vec![("out", 0), ("out", 1)]; "PragmaRepeatedMeasurement - none")]
#[test_case(Operation::from( InputSymbolic::new("a".to_string(), 20.0)), vec![("a", 0)]; "InputSymbolic")]
#[test_case(Operation::from( InputBit::new("ro".to_string(), 2, false)), vec![("ro", 2)]; "InputBit")]
#[test_case(Operation::from( PragmaConditional::new("ro".to_string(), 3, Circuit::new())), vec![("ro", 3)]; "PragmaConditional")]

fn involved_classical_set(operation: Operation, involved: Vec<(&str, usize)>) {
    let a: HashSet<(String, usize)> = involved
        .into_iter()
        .map(|(s, i)| (s.to_string(), i))
        .collect();
    assert_eq!(operation.involved_classical(), InvolvedClassical::Set(a));
}

#[test_case(Operation::from( DefinitionBit::new("out".to_string(), 20, true)), "out"; "DefinitionBit")]
#[test_case(Operation::from( DefinitionFloat::new("out".to_string(), 20, true)), "out"; "DefinitionFloat")]
#[test_case(Operation::from( DefinitionComplex::new("out".to_string(), 20, true)), "out"; "DefinitionComplex")]
#[test_case(Operation::from( DefinitionUsize::new("out".to_string(), 20, true)), "out"; "DefinitionUsize")]
#[test_case(Operation::from( PragmaGetStateVector::new("out".to_string(), None)), "out"; "PragmaGetStateVector")]
#[test_case(Operation::from( PragmaGetDensityMatrix::new("out".to_string(), None)), "out"; "PragmaGetDensityMatrix")]
fn involved_classical_all(operation: Operation, involved: &str) {
    assert_eq!(
        operation.involved_classical(),
        InvolvedClassical::All(involved.to_string())
    );
}

fn get_reordering() -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    map.insert(1, 1);
    map.insert(0, 0);
    map
}
