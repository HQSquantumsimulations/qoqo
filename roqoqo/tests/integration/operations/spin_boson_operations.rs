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
#[test]
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

#[test_case(
    SingleModeOperation::from(QuantumRabi::new(1, 0, "test".into())),
    SingleModeOperation::from(QuantumRabi::new(2, 3, 1.5.into()))
)]
#[test_case(
    SingleModeOperation::from(LongitudinalCoupling::new(1, 0, "test".into())),
    SingleModeOperation::from(LongitudinalCoupling::new(2, 3, 1.5.into()))
)]
#[test_case(
    SingleModeOperation::from(JaynesCummings::new(1, 0, "test".into())),
    SingleModeOperation::from(JaynesCummings::new(2, 3, 1.5.into()))
)]
fn substitute_subsitutemodes(op: SingleModeOperation, op_test: SingleModeOperation) {
    let mut mapping_test: HashMap<usize, usize> = HashMap::new();
    mapping_test.insert(0, 3);
    mapping_test.insert(1, 2);
    mapping_test.insert(2, 3);
    mapping_test.insert(3, 0);

    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 1.5);

    // (1) Substitute parameters function
    let result = op.substitute_parameters(&substitution_dict).unwrap();

    // (2) Remap modes function
    let result = result.remap_modes(&mapping_test).unwrap();

    // (3) Remap qubits function
    let result = result.remap_qubits(&mapping_test).unwrap();
    assert_eq!(result, op_test);
}

#[test_case(
    Operation::from(QuantumRabi::new(2, 3, 1.5.into())),
    Operation::from(QuantumRabi::new(1, 0, "test".into())),
    "QuantumRabi"
)]
#[test_case(
    Operation::from(LongitudinalCoupling::new(2, 3, 1.5.into())),
    Operation::from(LongitudinalCoupling::new(1, 0, "test".into())),
    "LongitudinalCoupling"
)]
#[test_case(
    Operation::from(JaynesCummings::new(2, 3, 1.5.into())),
    Operation::from(JaynesCummings::new(1, 0, "test".into())),
    "JaynesCummings"
)]
fn operate_one_mode_one_spin(op: Operation, op_param: Operation, name: &str) {
    // (1) Test tags function
    let tags: &[&str; 7] = &[
        "Operation",
        "GateOperation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "SingleQubitGateOperation",
        "SingleQubitGate",
        name,
    ];
    assert_eq!(op.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(op.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!op.is_parametrized());
    assert!(op_param.is_parametrized());
}

#[test_case(SingleModeOperation::from(QuantumRabi::new(0, 0, 1.0.into())))]
#[test_case(SingleModeOperation::from(LongitudinalCoupling::new(0, 0, 1.0.into())))]
#[test_case(SingleModeOperation::from(JaynesCummings::new(0, 0, 1.0.into())))]
fn single_mode_op(op: SingleModeOperation) {
    assert_eq!(op.mode(), &0_usize);
}

#[test_case(SingleQubitOperation::from(QuantumRabi::new(0, 0, 1.0.into())))]
#[test_case(SingleQubitOperation::from(LongitudinalCoupling::new(0, 0, 1.0.into())))]
#[test_case(SingleQubitOperation::from(JaynesCummings::new(0, 0, 1.0.into())))]
fn single_qubit_op(op: SingleQubitOperation) {
    assert_eq!(op.qubit(), &0_usize);
}

#[cfg(feature = "serialize")]
#[test]
fn quantumrabi_serde() {
    let op = QuantumRabi::new(0, 0, 1.0.into());

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "QuantumRabi",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("theta"),
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "QuantumRabi",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("theta"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "serialize")]
#[test]
fn longitudinal_coupling_serde() {
    let op = LongitudinalCoupling::new(0, 0, 1.0.into());

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "LongitudinalCoupling",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("theta"),
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "LongitudinalCoupling",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("theta"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "serialize")]
#[test]
fn jaynes_cummings_serde() {
    let op = JaynesCummings::new(0, 0, 1.0.into());

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "JaynesCummings",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("theta"),
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "JaynesCummings",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("theta"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn quantumrabi_json_schema() {
    let def = QuantumRabi::new(0, 0, 1.0.into());
    // Serialize
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(QuantumRabi);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

#[cfg(feature = "json_schema")]
#[test]
fn longitudinal_coupling_json_schema() {
    let def = LongitudinalCoupling::new(0, 0, 1.0.into());
    // Serialize
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(LongitudinalCoupling);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

#[cfg(feature = "json_schema")]
#[test]
fn jaynes_cummings_json_schema() {
    let def = JaynesCummings::new(0, 0, 1.0.into());
    // Serialize
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(JaynesCummings);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
