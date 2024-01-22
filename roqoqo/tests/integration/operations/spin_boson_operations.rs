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

//! Integration test for public API of spin-boson operations

#[cfg(feature = "json_schema")]
use jsonschema::{Draft, JSONSchema};
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use schemars::schema_for;
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{HashMap, HashSet};
use test_case::test_case;

/// Test inputs
fn inputs() {
    let op = QuantumRabi::new(4, 0, 1.5.into());
    assert_eq!(op.qubit(), &4_usize);
    assert_eq!(op.mode(), &0_usize);
    assert_eq!(op.theta(), &CalculatorFloat::from(1.5));

    let op = LongitudinalCoupling::new(4, 0, 1.5.into());
    assert_eq!(op.qubit(), &4_usize);
    assert_eq!(op.mode(), &0_usize);
    assert_eq!(op.theta(), &CalculatorFloat::from(1.5));

    let op = JaynesCummings::new(4, 0, 1.5.into());
    assert_eq!(op.qubit(), &4_usize);
    assert_eq!(op.mode(), &0_usize);
    assert_eq!(op.theta(), &CalculatorFloat::from(1.5));
}

#[test_case(Operation::from(QuantumRabi::new(4, 0, 1.5.into())))]
#[test_case(Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())))]
#[test_case(Operation::from(JaynesCummings::new(4, 0, 1.5.into())))]
fn clone(op: Operation) {
    assert_eq!(op.clone(), op);
}

#[test_case(
    Operation::from(QuantumRabi::new(4, 0, 1.5.into())),
    "QuantumRabi(QuantumRabi { qubit: 4, mode: 0, theta: Float(1.5) })"  
)]
#[test_case(
    Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())),
   "LongitudinalCoupling(LongitudinalCoupling { qubit: 4, mode: 0, theta: Float(1.5) })"  
)]
#[test_case(
    Operation::from(JaynesCummings::new(4, 0, 1.5.into())),
   "JaynesCummings(JaynesCummings { qubit: 4, mode: 0, theta: Float(1.5) })"  
)]
fn debug(op: Operation, string: &str) {
    assert_eq!(format!("{:?}", op), string);
}

#[test_case(
    Operation::from(QuantumRabi::new(4, 0, 1.5.into())),
    Operation::from(QuantumRabi::new(4, 0, 1.5.into())),
    Operation::from(QuantumRabi::new(2, 1, 1.0.into()))
)]
#[test_case(
    Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())),
    Operation::from(LongitudinalCoupling::new(4, 0, 1.5.into())),
    Operation::from(LongitudinalCoupling::new(2, 1, 1.0.into()))
)]
#[test_case(
    Operation::from(JaynesCummings::new(4, 0, 1.5.into())),
    Operation::from(JaynesCummings::new(4, 0, 1.5.into())),
    Operation::from(JaynesCummings::new(2, 1, 1.0.into()))
)]
fn partial_eq(op: Operation, op_0: Operation, op_1: Operation) {
    assert!(op_0 == op);
    assert!(op == op_0);
    assert!(op_1 != op);
    assert!(op != op_1);
}

#[test_case(
    SingleModeOperation::from(QuantumRabi::new(4, 0, 1.5.into())),
    InvolvedQubits::Set(HashSet::from([4_usize])),
    InvolvedClassical::None,
    InvolvedModes::Set(HashSet::from([0_usize]))
)]
#[test_case(
    SingleModeOperation::from(LongitudinalCoupling::new(4, 0, 1.5.into())),
    InvolvedQubits::Set(HashSet::from([4_usize])),
    InvolvedClassical::None,
    InvolvedModes::Set(HashSet::from([0_usize]))
)]
#[test_case(
    SingleModeOperation::from(JaynesCummings::new(4, 0, 1.5.into())),
    InvolvedQubits::Set(HashSet::from([4_usize])),
    InvolvedClassical::None,
    InvolvedModes::Set(HashSet::from([0_usize]))
)]
fn involved_qubits_classical_modes(
    op: SingleModeOperation,
    qubits: InvolvedQubits,
    classical: InvolvedClassical,
    modes: InvolvedModes,
) {
    assert_eq!(op.involved_qubits(), qubits);
    assert_eq!(op.involved_classical(), classical);
    assert_eq!(op.involved_modes(), modes);
}

fn substitute_subsitutemodes(op: ModeGateOperation, op_test: ModeGateOperation) {
    let mut mapping_test: HashMap<usize, usize> = HashMap::new();
    mapping_test.insert(0, 1);
    mapping_test.insert(1, 2);
    mapping_test.insert(2, 0);

    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.1);
    substitution_dict.set_variable("test1", 0.5);

    // (1) Substitute parameters function
    let result = op.substitute_parameters(&substitution_dict).unwrap();

    // (2) Remap modes function
    let result = result.remap_modes(&mapping_test).unwrap();
    assert_eq!(result, op_test);

    // (3) Remap qubits function
    let result = result.remap_qubits(&mapping_test).unwrap();
    assert_eq!(result, op_test);
}
