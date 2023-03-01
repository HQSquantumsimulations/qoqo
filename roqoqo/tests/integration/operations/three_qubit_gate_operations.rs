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

//! Integration test for public API of three qubit gate operations

use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::RoqoqoError::QubitMappingError;

use std::collections::{HashMap, HashSet};
use test_case::test_case;

#[test]
fn test_circuit() {
    todo!()
}

//
// Test Unitary Matrix for ThreeQubit Gates
//

// Test unitary matrix for ThreeQubitGate Operations
#[test_case(GateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(GateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_three_qubit_gate_unitarity(_gate: GateOperation) {
    todo!()
}

//
// Test 'Derive' for ThreeQubitGate Operations
//

/// Test clone function for ThreeQubitGate Operations
#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_twoqubitgates_clone(gate1: Operation) {
    let gate2 = gate1.clone();
    assert_eq!(gate2, gate1);
}

#[test_case(ThreeQubitGateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(ThreeQubitGateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_qubits_threequbitgates(gate: ThreeQubitGateOperation) {
    let control_0: &usize = gate.control_0();
    assert_eq!(control_0, &0);
    let control_1: &usize = gate.control_1();
    assert_eq!(control_1, &1);
    let target: &usize = gate.target();
    assert_eq!(target, &2);
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    qubits.insert(2);
    let test_qubits: InvolvedQubits = InvolvedQubits::Set(qubits);
    assert_eq!(gate.involved_qubits(), test_qubits);
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_is_parametrized_false(gate: Operation) {
    let bool_parameter = gate.is_parametrized();
    assert!(!bool_parameter);
}

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("x"))); "ControlledControlledPhaseShift")]
fn test_is_parametrized_true(gate: Operation) {
    let bool_parameter = gate.is_parametrized();
    assert!(bool_parameter);
}

#[test_case("ControlledControlledPauliZ", Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case("ControlledControlledPhaseShift", Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
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
fn remap_qubits_error0(gate: GateOperation) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(1, 0);
    let result = gate.remap_qubits(&qubit_mapping);
    assert_eq!(result, Err(QubitMappingError { qubit: 0 }));
}

#[test_case(GateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(GateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
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
fn test_three_qubitgates_debug(message: &'static str, gate: Operation) {
    assert_eq!(format!("{:?}", gate), message);
}

#[test_case(
    Operation::from(ControlledControlledPauliZ::new(0, 1, 2)),
    Operation::from(ControlledControlledPauliZ::new(1, 0, 2)); "ControlledControlledPauliZ")]
#[test_case(
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))),
    Operation::from(ControlledControlledPhaseShift::new(1, 0, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_twoqubitgates_partialeq(gate1: Operation, gate2: Operation) {
    assert!(gate1 == gate1.clone());
    assert_eq!(gate1, gate1.clone());
    assert_ne!(gate2, gate1);
    assert_ne!(gate1, gate2);
}

#[test_case(
    Rotation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::PI)),
    Rotation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::PI * 1.5)); "ControlledControlledPhaseShift")]
fn test_rotate_powercf(gate: Rotation, gate2: Rotation) {
    let power_gate = gate.powercf(CalculatorFloat::from(1.5));
    assert_eq!(power_gate, gate2);
    assert_eq!(power_gate.theta(), gate2.theta());
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_ineffective_substitute_parameters(gate: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, gate);
}

#[test_case(
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("theta"))),
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::ZERO)); "ControlledControlledPhaseShift")]
fn test_substitute_parameters(gate: Operation, gate2: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 0.0);
    let result = gate.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, gate2);
}

#[test_case(
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("theta"))); "ControlledControlledPhaseShift")]
fn test_substitute_parameters_error(gate: Operation) {
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("error", 0.0);
    let result = gate.substitute_parameters(&substitution_dict);
    assert!(result.is_err());
}

#[test]
fn test_inputs_controlledcontrolledpauliz() {
    let gate = ControlledControlledPauliZ::new(0, 1, 2);
    assert_eq!(gate.control_0(), &0);
    assert_eq!(gate.control_1(), &1);
    assert_eq!(gate.target(), &2);
}

#[test]
fn test_inputs_controlledcontrolledphaseshift() {
    let gate = ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2));
    assert_eq!(gate.control_0(), &0);
    assert_eq!(gate.control_1(), &1);
    assert_eq!(gate.target(), &2);
    assert_eq!(gate.theta(), &CalculatorFloat::from(0.2));
}
