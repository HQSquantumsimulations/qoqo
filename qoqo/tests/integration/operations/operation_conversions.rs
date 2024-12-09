// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use ndarray::{array, Array1, Array2};
use num_complex::Complex64;
use pyo3::Python;
use qoqo::operations::*;
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use roqoqo::Circuit;
use std::collections::HashMap;
use std::f64::consts::PI;
#[cfg(feature = "unstable_analog_operations")]
use struqture::prelude::*;
#[cfg(feature = "unstable_analog_operations")]
use struqture::spins::*;
use test_case::test_case;

/// Test convert_operation_to_pyobject and convert_pyany_to_operation
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(RotateXY::new(0, CalculatorFloat::from(0), 1.0.into())); "RotateXY float")]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(GPi::new(3, 0.1.into())); "GPi")]
#[test_case(Operation::from(GPi2::new(3, 0.1.into())); "GPi2")]
#[test_case(Operation::from(Identity::new(3)); "Identity")]
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(EchoCrossResonance::new(0, 1)); "EchoCrossResonance")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::FRAC_PI_4, 0.1.into())); "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::FRAC_PI_4)); "PhaseShiftedState1")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(ControlledRotateX::new(0, 1, 0.1.into())); "ControlledRotateX")]
#[test_case(Operation::from(ControlledRotateXY::new(0, 1, 0.1.into(), 0.5.into())); "ControlledRotateXY")]
#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, 0.1.into())); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], 0.1.into())); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1, 2], 0.1.into())); "MultiQubitZZ")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::PI)); "VariableMSXX")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)); "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Qsim")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Fsim")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(2.0), CalculatorFloat::from(-1.0))); "SpinInteraction")]
#[test_case(Operation::from(Bogoliubov::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "Bogoliubov")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::PI)); "PMInteraction")]
#[test_case(Operation::from(ComplexPMInteraction::new(0, 1, CalculatorFloat::from(1.0), CalculatorFloat::from(-1.0))); "ComplexPMInteraction")]
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
#[test_case(Operation::from(MeasureQubit::new(0, String::from("ro"), 1)); "MeasureQubit")]
#[test_case(Operation::from(PragmaGetStateVector::new(String::from("ro"), Some(create_circuit()))); "PragmaGetStateVector")]
#[test_case(Operation::from(PragmaGetDensityMatrix::new(String::from("ro"), Some(create_circuit()))); "PragmaGetDensityMatrix")]
#[test_case(Operation::from(PragmaGetOccupationProbability::new(String::from("ro"), Some(create_circuit()))); "PragmaGetOccupationProbability")]
#[test_case(Operation::from(PragmaGetPauliProduct::new(reordering(), String::from("ro"), create_circuit())); "PragmaGetPauliProduct")]
#[test_case(Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(reordering()))); "PragmaRepeatedMeasurement")]
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(1, String::from("ro"))); "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaSetStateVector::new(statevector())); "PragmaSetStateVector")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(densitymatrix())); "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)); "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.003))); "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.05))); "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001))); "PragmaSleep")]
#[test_case(Operation::from(PragmaActiveReset::new(0)); "PragmaActiveReset")]
#[test_case(Operation::from(PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001)); "PragmaOverrotation")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0, 1], reordering())); "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0, 1])); "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDamping")]
#[test_case(Operation::from(PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDepolarising")]
#[test_case(Operation::from(PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02))); "PragmaDephasing")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02), CalculatorFloat::from(0.01))); "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators())); "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaConditional::new(String::from("ro"), 1, create_circuit())); "PragmaConditional")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::from(0.005), create_circuit())); "PragmaLoop")]
#[test_case(Operation::from(PragmaControlledCircuit::new(0, create_circuit())); "PragmaControlledCircuit")]
#[test_case(Operation::from(Squeezing::new(0, CalculatorFloat::from(0.005), 1.0.into())); "Squeezing")]
#[test_case(Operation::from(PhaseShift::new(0, CalculatorFloat::from(0.005))); "PhaseShift")]
#[test_case(Operation::from(PhaseDisplacement::new(0, CalculatorFloat::from(0.005), 1.0.into())); "PhaseDisplacement")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, CalculatorFloat::from(0.005), 1.0.into())); "Beamsplitter")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)); "PhotonDetection")]
#[test_case(Operation::from(PragmaAnnotatedOp::new(PauliX::new(0).into(), "test".to_string())); "PragmaAnnotatedOp")]
#[test_case(Operation::from(QuantumRabi::new(0, 1, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(0, 1, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(0, 1, 1.0.into())); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationStore::new(0, 1)); "SingleExcitationStore")]
#[test_case(Operation::from(SingleExcitationLoad::new(0, 1)); "SingleExcitationLoad")]
#[test_case(Operation::from(CZQubitResonator::new(0, 1)); "CZQubitResonator")]
#[test_case(Operation::from(SqrtPauliY::new(100)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(100)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(1)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(1)); "InvSXGate")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::PI, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
#[test_case(
    Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX"
)]
#[test_case(
    Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ"
)]
#[test_case(
    Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::PI)); "TripleControlledPhaseShift"
)]
fn test_conversion(input: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input.clone()).unwrap();
        let output = convert_pyany_to_operation(operation.bind(py)).unwrap();
        assert_eq!(input, output)
    })
}

#[cfg(feature = "unstable_simulation_repetitions")]
#[test_case(
    Operation::from(PragmaSimulationRepetitions::new(100)); "PragmaSimulationRepetitions"
)]
fn test_conversion_unstable(input: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input.clone()).unwrap();
        let output = convert_pyany_to_operation(operation.bind(py)).unwrap();
        assert_eq!(input, output)
    })
}

#[cfg(feature = "unstable_analog_operations")]
fn create_apply_constant_spin_hamiltonian<T>(p: T) -> ApplyConstantSpinHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();
    ApplyConstantSpinHamiltonian::new(hamiltonian, 1.0.into())
}
#[cfg(feature = "unstable_analog_operations")]
fn create_apply_timedependent_spin_hamiltonian<T>(p: T) -> ApplyTimeDependentSpinHamiltonian
where
    CalculatorFloat: From<T>,
{
    let pp = PauliProduct::new().z(0);
    let mut hamiltonian = SpinHamiltonian::new();
    hamiltonian
        .add_operator_product(pp.clone(), CalculatorFloat::from(p))
        .unwrap();

    let mut values = HashMap::new();
    values.insert("omega".to_string(), vec![1.0]);

    ApplyTimeDependentSpinHamiltonian::new(hamiltonian, vec![1.0], values.clone())
}

#[cfg(feature = "unstable_analog_operations")]
#[test_case(Operation::from(create_apply_constant_spin_hamiltonian(1.0)); "ApplyConstantSpinHamiltonian")]
#[test_case(Operation::from(create_apply_timedependent_spin_hamiltonian("omega")); "ApplyTimeDependentSpinHamiltonian")]
fn test_conversion_feature(input: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input.clone()).unwrap();
        let output = convert_pyany_to_operation(operation.bind(py)).unwrap();
        assert_eq!(input, output)
    })
}

#[cfg(feature = "unstable_operation_definition")]
#[test_case(Operation::from(GateDefinition::new(create_circuit(), "name".into(), vec![1, 2], vec!["test".into()])); "GateDefinition")]
#[test_case(Operation::from(CallDefinedGate::new("name".into(), vec![1, 2], vec![CalculatorFloat::from(0.6)])); "CallDefinedGate")]
fn test_conversion_operation_definition(input: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input.clone()).unwrap();
        let output = convert_pyany_to_operation(operation.bind(py)).unwrap();
        assert_eq!(input, output)
    })
}

// ---------------- Helper functions ---------------- //

fn reordering() -> HashMap<usize, usize> {
    let mut reordering: HashMap<usize, usize> = HashMap::new();
    reordering.insert(0, 1);
    reordering
}

fn statevector() -> Array1<Complex64> {
    let statevector: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    statevector
}

fn densitymatrix() -> Array2<Complex64> {
    let densitymatrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    densitymatrix
}

fn operators() -> Array2<f64> {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],];
    operators
}

fn create_circuit() -> Circuit {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    circuit
}
