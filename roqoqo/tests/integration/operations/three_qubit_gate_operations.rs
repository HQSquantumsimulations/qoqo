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

//! Integration test for public API of three qubit gate operations

use super::convert_matrix;
#[cfg(feature = "json_schema")]
use jsonschema::{Draft, Validator};
use nalgebra::DMatrix;
use ndarray::Array2;
use num_complex::Complex64;
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::RoqoqoError::QubitMappingError;
use roqoqo::{operations::*, Circuit, RoqoqoError};
#[cfg(feature = "json_schema")]
use schemars::schema_for;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

// helper function to convert a complex 8x8 matrix to a matrix with real absolute values
fn convert_normsqr(customarray: DMatrix<Complex64>) -> Vec<f64> {
    let mut overall_vec: Vec<[f64; 8]> = Vec::new();
    for i in [0, 8, 16, 24, 32, 40, 48, 56].iter() {
        let mut this_vec: Vec<f64> = Vec::new();
        for j in 0..8 {
            this_vec.push(customarray[i + j].norm());
        }
        let this_vec_to_array: [f64; 8] = this_vec.try_into().unwrap();
        overall_vec.push(this_vec_to_array);
    }
    overall_vec.concat()
}

#[test]
fn test_circuit_controlledcontrolledpauliz() {
    let op = ControlledControlledPauliZ::new(0, 1, 2);
    let c = op.circuit();

    let mut circuit = Circuit::new();
    circuit += ControlledPhaseShift::new(1, 2, CalculatorFloat::FRAC_PI_2);
    circuit += CNOT::new(0, 1);
    circuit += ControlledPhaseShift::new(1, 2, -CalculatorFloat::FRAC_PI_2);
    circuit += CNOT::new(0, 1);
    circuit += ControlledPhaseShift::new(0, 2, CalculatorFloat::FRAC_PI_2);

    assert_eq!(c, circuit);
}

#[test]
fn test_circuit_controlledcontrolledphaseshift() {
    let op = ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::PI);
    let c = op.circuit();

    let mut circuit = Circuit::new();
    circuit += ControlledPhaseShift::new(1, 2, CalculatorFloat::PI / 2.0);
    circuit += CNOT::new(0, 1);
    circuit += ControlledPhaseShift::new(1, 2, -CalculatorFloat::PI / 2.0);
    circuit += CNOT::new(0, 1);
    circuit += ControlledPhaseShift::new(0, 2, CalculatorFloat::PI / 2.0);

    assert_eq!(c, circuit);
}

#[test]
fn test_circuit_toffoli() {
    let op = Toffoli::new(0, 1, 2);
    let c = op.circuit();

    let mut circuit = Circuit::new();
    circuit += Hadamard::new(2);
    circuit += CNOT::new(1, 2);
    circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
    circuit += CNOT::new(0, 2);
    circuit += TGate::new(2);
    circuit += CNOT::new(1, 2);
    circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
    circuit += CNOT::new(0, 2);
    circuit += TGate::new(1);
    circuit += TGate::new(2);
    circuit += Hadamard::new(2);
    circuit += CNOT::new(0, 1);
    circuit += TGate::new(0);
    circuit += RotateZ::new(1, -CalculatorFloat::FRAC_PI_4);
    circuit += CNOT::new(0, 1);

    assert_eq!(c, circuit);
}

#[test]
fn test_circuit_controlledswap() {
    let op = ControlledSWAP::new(0, 1, 2);
    let c = op.circuit();

    let mut circuit = Circuit::new();
    circuit += CNOT::new(2, 1);
    circuit += Hadamard::new(2);
    circuit += CNOT::new(1, 2);
    circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
    circuit += CNOT::new(0, 2);
    circuit += TGate::new(2);
    circuit += CNOT::new(1, 2);
    circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
    circuit += CNOT::new(0, 2);
    circuit += TGate::new(1);
    circuit += TGate::new(2);
    circuit += Hadamard::new(2);
    circuit += CNOT::new(0, 1);
    circuit += TGate::new(0);
    circuit += RotateZ::new(1, -CalculatorFloat::FRAC_PI_4);
    circuit += CNOT::new(0, 1);
    circuit += CNOT::new(2, 1);

    assert_eq!(c, circuit);
}

#[test]
fn test_circuit_phaseshiftedcontrolledcontrolledz() {
    let op = PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2);
    let c = op.circuit();

    let mut circuit = Circuit::new();
    circuit += PhaseShiftedControlledPhase::new(
        1,
        2,
        CalculatorFloat::FRAC_PI_2,
        CalculatorFloat::FRAC_PI_2,
    );
    circuit += CNOT::new(0, 1);
    circuit += PhaseShiftedControlledPhase::new(
        1,
        2,
        -CalculatorFloat::FRAC_PI_2,
        CalculatorFloat::FRAC_PI_2,
    );
    circuit += CNOT::new(0, 1);
    circuit += PhaseShiftedControlledPhase::new(
        0,
        2,
        CalculatorFloat::FRAC_PI_2,
        CalculatorFloat::FRAC_PI_2,
    );

    assert_eq!(c, circuit);
}

#[test]
fn test_circuit_phaseshiftedcontrolledcontrolledphase() {
    let op = PhaseShiftedControlledControlledPhase::new(
        0,
        1,
        2,
        CalculatorFloat::FRAC_PI_2,
        CalculatorFloat::PI,
    );
    let c = op.circuit();

    let mut circuit = Circuit::new();
    circuit +=
        PhaseShiftedControlledPhase::new(1, 2, CalculatorFloat::FRAC_PI_4, CalculatorFloat::PI);
    circuit += CNOT::new(0, 1);
    circuit +=
        PhaseShiftedControlledPhase::new(1, 2, -CalculatorFloat::FRAC_PI_4, CalculatorFloat::PI);
    circuit += CNOT::new(0, 1);
    circuit +=
        PhaseShiftedControlledPhase::new(0, 2, CalculatorFloat::FRAC_PI_4, CalculatorFloat::PI);

    assert_eq!(c, circuit);
}

//
// Test Unitary Matrix for ThreeQubit Gates
//

// Test unitary matrix for ThreeQubitGate Operations
#[test_case(GateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(GateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(GateOperation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(GateOperation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
#[test_case(GateOperation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(GateOperation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_three_qubit_gate_unitarity(gate: GateOperation) {
    let result: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    let result_array: Array2<Complex64> = result.unwrap();
    // check unitarity with nalgebra
    // convert ndarray into nalgebra matrix
    let result_matrix: DMatrix<Complex64> = convert_matrix(result_array);
    // calculate matrix product A*A_dagger
    let product = result_matrix.clone() * result_matrix.adjoint();
    // convert complex matrix product into real matrix by taking the absolute value of the complex number, which should be sufficient if the matrix is unitary.
    let matrix_norm: DMatrix<f64> =
        DMatrix::<f64>::from_vec(8, 8, convert_normsqr(product).to_vec());
    let epsilon = 1e-12;
    assert!(matrix_norm.is_identity(epsilon));
}

//
// Test 'Derive' for ThreeQubitGate Operations
//

/// Test clone function for ThreeQubitGate Operations
#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_twoqubitgates_clone(gate1: Operation) {
    #[allow(clippy::redundant_clone)]
    let gate2 = gate1.clone();
    assert_eq!(gate2, gate1);
}

#[test_case(ThreeQubitGateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(ThreeQubitGateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(ThreeQubitGateOperation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(ThreeQubitGateOperation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(ThreeQubitGateOperation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_qubits_threequbitgates(gate: ThreeQubitGateOperation) {
    let control_0: Option<&usize> = gate.control_0();
    assert_eq!(control_0, Some(&0));
    let control_1: Option<&usize> = gate.control_1();
    assert_eq!(control_1, Some(&1));
    let target: Option<&usize> = gate.target();
    assert_eq!(target, Some(&2));
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    qubits.insert(2);
    let test_qubits: InvolvedQubits = InvolvedQubits::Set(qubits);
    assert_eq!(gate.involved_qubits(), test_qubits);
}

#[test_case(ThreeQubitGateOperation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
fn test_qubits_threequbitgates_cswap(gate: ThreeQubitGateOperation) {
    let control: Option<&usize> = gate.control();
    assert_eq!(control, Some(&0));
    let target_0: Option<&usize> = gate.target_0();
    assert_eq!(target_0, Some(&1));
    let target_1: Option<&usize> = gate.target_1();
    assert_eq!(target_1, Some(&2));
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    qubits.insert(2);
    let test_qubits: InvolvedQubits = InvolvedQubits::Set(qubits);
    assert_eq!(gate.involved_qubits(), test_qubits);
    assert!(gate.control_0().is_none());
    assert!(gate.control_1().is_none());
    assert!(gate.target().is_none());
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_is_parametrized_false(gate: Operation) {
    let bool_parameter = gate.is_parametrized();
    assert!(!bool_parameter);
}

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("x"))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("x"))); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("y"), CalculatorFloat::from("z"))); "PhaseShiftedControlledControlledPhase")]
fn test_is_parametrized_true(gate: Operation) {
    let bool_parameter = gate.is_parametrized();
    assert!(bool_parameter);
}

#[test_case("ControlledControlledPauliZ", Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case("ControlledControlledPhaseShift", Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case("Toffoli", Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case("ControlledSWAP", Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
#[test_case("PhaseShiftedControlledControlledZ", Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case("PhaseShiftedControlledControlledPhase", Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_threequbitgateoperations_hqslang(name: &'static str, gate: Operation) {
    assert!(!gate.hqslang().is_empty());
    assert_eq!(gate.hqslang(), name);
}

#[test_case(
    GateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)),
    GateOperation::from(ControlledControlledPauliZ::new(1, 2, 0)); "ControlledControlledPauliZ")]
#[test_case(
    GateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))),
    GateOperation::from(ControlledControlledPhaseShift::new(1, 2, 0, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(
    GateOperation::from(Toffoli::new(0, 1, 2)),
    GateOperation::from(Toffoli::new(1, 2, 0)); "Toffoli")]
#[test_case(
    GateOperation::from(ControlledSWAP::new(0, 1, 2)),
    GateOperation::from(ControlledSWAP::new(1, 2, 0)); "ControlledSWAP")]
#[test_case(
    GateOperation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)),
    GateOperation::from(PhaseShiftedControlledControlledZ::new(1, 2, 0, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(
    GateOperation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)),
    GateOperation::from(PhaseShiftedControlledControlledPhase::new(1, 2, 0, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn remap_qubits_result(gate: GateOperation, test_gate: GateOperation) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    qubit_mapping.insert(1, 2);
    qubit_mapping.insert(2, 0);
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(&result, &Ok(test_gate));
}

#[test_case(GateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(GateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(GateOperation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(GateOperation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
#[test_case(GateOperation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(GateOperation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn remap_qubits_error0(gate: GateOperation) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(1, 0);
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(result, Err(QubitMappingError { qubit: 0 }));
}

#[test_case(GateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(GateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(GateOperation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(GateOperation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
#[test_case(GateOperation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(GateOperation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn remap_qubits_error1(gate: GateOperation) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 2);
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(result, Err(QubitMappingError { qubit: 2 }));
}

#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "ControlledControlledPauliZ",
        ],
    Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "Rotation",
        "ControlledControlledPhaseShift",
        ],
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "Toffoli",
        ],
    Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "ControlledSWAP",
        ],
    Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "PhaseShiftedControlledControlledZ",
        ],
    Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "PhaseShiftedControlledControlledPhase",
        ],
    Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
pub fn test_tags(tags: Vec<&str>, gate: Operation) {
    let range = 0..tags.len();
    for i in range {
        assert_eq!(gate.tags()[i], tags[i]);
    }
}

#[test_case(
    "ControlledControlledPauliZ(ControlledControlledPauliZ { control_0: 1, control_1: 0, target: 2 })",
    Operation::from(ControlledControlledPauliZ::new(1, 0, 2)); "ControlledControlledPauliZ")]
#[test_case(
    "ControlledControlledPhaseShift(ControlledControlledPhaseShift { control_0: 1, control_1: 0, target: 2, theta: Float(-1.0) })",
    Operation::from(ControlledControlledPhaseShift::new(1, 0, 2, CalculatorFloat::from(-1.0))); "ControlledControlledPhaseShift")]
#[test_case(
    "Toffoli(Toffoli { control_0: 1, control_1: 0, target: 2 })",
    Operation::from(Toffoli::new(1, 0, 2)); "Toffoli")]
#[test_case(
    "ControlledSWAP(ControlledSWAP { control: 1, target_0: 0, target_1: 2 })",
    Operation::from(ControlledSWAP::new(1, 0, 2)); "ControlledSWAP")]
#[test_case(
    "PhaseShiftedControlledControlledZ(PhaseShiftedControlledControlledZ { control_0: 1, control_1: 0, target: 2, phi: Float(3.141592653589793) })",
    Operation::from(PhaseShiftedControlledControlledZ::new(1, 0, 2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledZ")]
#[test_case(
    "PhaseShiftedControlledControlledPhase(PhaseShiftedControlledControlledPhase { control_0: 1, control_1: 0, target: 2, theta: Float(3.141592653589793), phi: Float(1.5707963267948966) })",
    Operation::from(PhaseShiftedControlledControlledPhase::new(1, 0, 2, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledPhase")]
fn test_three_qubitgates_debug(message: &'static str, gate: Operation) {
    assert_eq!(format!("{:?}", gate), message);
}

#[test_case(
    Operation::from(ControlledControlledPauliZ::new(0, 1, 2)),
    Operation::from(ControlledControlledPauliZ::new(1, 0, 2)); "ControlledControlledPauliZ")]
#[test_case(
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))),
    Operation::from(ControlledControlledPhaseShift::new(1, 0, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(
    Operation::from(Toffoli::new(0, 1, 2)),
    Operation::from(Toffoli::new(1, 0, 2)); "Toffoli")]
#[test_case(
    Operation::from(ControlledSWAP::new(0, 1, 2)),
    Operation::from(ControlledSWAP::new(1, 2, 0)); "ControlledSWAP")]
#[test_case(
    Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::PI)),
    Operation::from(PhaseShiftedControlledControlledZ::new(1, 2, 0, CalculatorFloat::PI)); "PhaseShiftedControlledControlledZ")]
#[test_case(
    Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::PI, CalculatorFloat::PI)),
    Operation::from(PhaseShiftedControlledControlledPhase::new(1, 2, 0, CalculatorFloat::PI, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_threequbitgates_partialeq(gate1: Operation, gate2: Operation) {
    assert!(gate1 == gate1.clone());
    assert_eq!(gate1, gate1.clone());
    assert_ne!(gate2, gate1);
    assert_ne!(gate1, gate2);
}

#[test_case(
    Rotation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::PI)),
    Rotation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::PI * 1.5)); "ControlledControlledPhaseShift")]
#[test_case(
    Rotation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)),
    Rotation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2 * 1.5, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_rotate_powercf(gate: Rotation, gate2: Rotation) {
    let power_gate = gate.powercf(CalculatorFloat::from(1.5));
    assert_eq!(power_gate, gate2);
    assert_eq!(power_gate.theta(), gate2.theta());
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSwap")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_ineffective_substitute_parameters(gate: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, gate);
}

#[test_case(
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("theta"))),
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::ZERO)); "ControlledControlledPhaseShift")]
#[test_case(
    Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("theta"))),
    Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::ZERO)); "PhaseShiftedControlledControlledZ")]
#[test_case(
    Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("theta"), CalculatorFloat::from("theta"))),
    Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::ZERO, CalculatorFloat::ZERO)); "PhaseShiftedControlledControlledPhase")]
fn test_substitute_parameters(gate: Operation, gate2: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, gate2);
}

#[test_case(
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("theta"))); "ControlledControlledPhaseShift")]
#[test_case(
    Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledZ")]
#[test_case(
    Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("theta"), CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledPhase")]
fn test_substitute_parameters_error(gate: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("error", 0.0);
    let result = gate.substitute_parameters(&substitution_dict);
    assert!(result.is_err());
}

#[test]
fn test_inputs_controlledcontrolledpauliz() {
    let gate = ControlledControlledPauliZ::new(0, 1, 2);
    assert_eq!(gate.control_0(), Some(&0));
    assert_eq!(gate.control_1(), Some(&1));
    assert_eq!(gate.target(), Some(&2));
    assert!(gate.control().is_none());
    assert!(gate.target_0().is_none());
    assert!(gate.target_1().is_none());
}

#[test]
fn test_inputs_controlledcontrolledphaseshift() {
    let gate = ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2));
    assert_eq!(gate.control_0(), Some(&0));
    assert_eq!(gate.control_1(), Some(&1));
    assert_eq!(gate.target(), Some(&2));
    assert_eq!(gate.theta(), &CalculatorFloat::from(0.2));
    assert!(gate.control().is_none());
    assert!(gate.target_0().is_none());
    assert!(gate.target_1().is_none());
}

#[test]
fn test_inputs_toffoli() {
    let gate = Toffoli::new(0, 1, 2);
    assert_eq!(gate.control_0(), Some(&0));
    assert_eq!(gate.control_1(), Some(&1));
    assert_eq!(gate.target(), Some(&2));
    assert!(gate.control().is_none());
    assert!(gate.target_0().is_none());
    assert!(gate.target_1().is_none());
}

#[test]
fn test_inputs_cswap() {
    let gate = ControlledSWAP::new(0, 1, 2);
    assert_eq!(gate.control(), Some(&0));
    assert_eq!(gate.target_0(), Some(&1));
    assert_eq!(gate.target_1(), Some(&2));
    assert!(gate.control_0().is_none());
    assert!(gate.control_1().is_none());
    assert!(gate.target().is_none());
}

#[test]
fn test_inputs_phaseshiftedccz() {
    let gate = PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::PI);
    assert!(gate.control().is_none());
    assert_eq!(gate.control_0(), Some(&0));
    assert_eq!(gate.control_1(), Some(&1));
    assert_eq!(gate.target(), Some(&2));
    assert!(gate.target_0().is_none());
    assert!(gate.target_1().is_none());
}

#[test]
fn test_inputs_phaseshiftedccps() {
    let gate = PhaseShiftedControlledControlledPhase::new(
        0,
        1,
        2,
        CalculatorFloat::PI,
        CalculatorFloat::PI,
    );
    assert!(gate.control().is_none());
    assert_eq!(gate.control_0(), Some(&0));
    assert_eq!(gate.control_1(), Some(&1));
    assert_eq!(gate.target(), Some(&2));
    assert!(gate.target_0().is_none());
    assert!(gate.target_1().is_none());
}

/// Test JsonSchema trait
#[cfg(feature = "json_schema")]
#[test_case(ThreeQubitGateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(ThreeQubitGateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(ThreeQubitGateOperation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(ThreeQubitGateOperation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(
    ThreeQubitGateOperation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledZ")]
#[test_case(
    ThreeQubitGateOperation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("theta"), CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledPhase")]
pub fn test_json_schema_three_qubit_gate_operations(gate: ThreeQubitGateOperation) {
    // Serialize
    let test_json = match gate.clone() {
        ThreeQubitGateOperation::ControlledControlledPauliZ(op) => {
            serde_json::to_string(&op).unwrap()
        }
        ThreeQubitGateOperation::ControlledControlledPhaseShift(op) => {
            serde_json::to_string(&op).unwrap()
        }
        ThreeQubitGateOperation::Toffoli(op) => serde_json::to_string(&op).unwrap(),
        ThreeQubitGateOperation::ControlledSWAP(op) => serde_json::to_string(&op).unwrap(),
        ThreeQubitGateOperation::PhaseShiftedControlledControlledZ(op) => {
            serde_json::to_string(&op).unwrap()
        }
        ThreeQubitGateOperation::PhaseShiftedControlledControlledPhase(op) => {
            serde_json::to_string(&op).unwrap()
        }
        _ => unreachable!(),
    };
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = match gate {
        ThreeQubitGateOperation::ControlledControlledPauliZ(_) => {
            schema_for!(ControlledControlledPauliZ)
        }
        ThreeQubitGateOperation::ControlledControlledPhaseShift(_) => {
            schema_for!(ControlledControlledPhaseShift)
        }
        ThreeQubitGateOperation::Toffoli(_) => schema_for!(Toffoli),
        ThreeQubitGateOperation::ControlledSWAP(_) => schema_for!(ControlledSWAP),
        ThreeQubitGateOperation::PhaseShiftedControlledControlledZ(_) => {
            schema_for!(PhaseShiftedControlledControlledZ)
        }
        ThreeQubitGateOperation::PhaseShiftedControlledControlledPhase(_) => {
            schema_for!(PhaseShiftedControlledControlledPhase)
        }
        _ => unreachable!(),
    };
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
