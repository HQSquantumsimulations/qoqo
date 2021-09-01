// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use qoqo::operations::*;
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use roqoqo::Circuit;
use std::collections::HashMap;
use std::f64::consts::PI;
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
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(CNOT::new(0, 1)); "CNOT")]
#[test_case(Operation::from(SWAP::new(0, 1)); "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)); "ISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)); "FSwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)); "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)); "InvSqrtISwap")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)); "XY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_4)); "ControlledPhaseShift")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)); "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)); "ControlledPauliZ")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)); "MolmerSorensenXX")]
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
fn test_conversion(input: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input.clone()).unwrap();
    let output = convert_pyany_to_operation(operation.as_ref(py)).unwrap();
    assert_eq!(input, output)
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
