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

//! Integration test for public API of bosonic operations

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

/// Test Squeezing inputs
#[test]
fn squeezing_inputs() {
    let op = Squeezing::new(1, 0.1.into(), 0.0.into());
    assert_eq!(op.mode(), &1_usize);
    assert_eq!(op.squeezing(), &CalculatorFloat::from(0.1));
    assert_eq!(op.phase(), &CalculatorFloat::from(0.0));
}

/// Test PhaseDisplacement inputs
#[test]
fn phasedisplacement_inputs() {
    let op = PhaseDisplacement::new(1, 0.1.into(), CalculatorFloat::PI);
    assert_eq!(op.mode(), &1_usize);
    assert_eq!(op.displacement(), &CalculatorFloat::from(0.1));
    assert_eq!(op.phase(), &CalculatorFloat::PI);
}

/// Test PhaseShift inputs
#[test]
fn phaseshift_inputs() {
    let op = PhaseShift::new(1, 0.1.into());
    assert_eq!(op.mode(), &1_usize);
    assert_eq!(op.phase(), &CalculatorFloat::from(0.1));
}

/// Test Squeezing inputs
#[test]
fn beamsplitter_inputs() {
    let op = BeamSplitter::new(0, 1, 0.5.into(), 0.1.into());
    assert_eq!(op.mode_0(), &0_usize);
    assert_eq!(op.mode_1(), &1_usize);
    assert_eq!(op.theta(), &CalculatorFloat::from(0.5));
    assert_eq!(op.phi(), &CalculatorFloat::from(0.1));
}

/// Test Squeezing inputs
#[test]
fn photondetection_inputs() {
    let op = PhotonDetection::new(1, "ro".into(), 0);
    assert_eq!(op.mode(), &1_usize);
    assert_eq!(op.readout(), &String::from("ro"));
    assert_eq!(op.readout_index(), &0_usize)
}

#[test_case(Operation::from(Squeezing::new(0, 0.5.into(), 0.0.into())))]
// #[test_case(Operation::from(PhaseDisplacement::new(0, 0.5.into(), 0.1.into())))]
#[test_case(Operation::from(PhaseShift::new(0, 0.5.into())))]
#[test_case(Operation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())))]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)))]
fn clone(op: Operation) {
    assert_eq!(op.clone(), op);
}

#[test_case(Operation::from(Squeezing::new(0, 0.5.into(), 0.0.into())), "Squeezing(Squeezing { mode: 0, squeezing: Float(0.5), phase: Float(0.0) })")]
// #[test_case(Operation::from(PhaseDisplacement::new(0, 0.5.into(), 0.1.into())), "PhaseDisplacement(PhaseDisplacement { mode: 0, displacement: Float(0.5), phase: Float(0.1) })")]
#[test_case(Operation::from(PhaseShift::new(0, 0.5.into())), "PhaseShift(PhaseShift { mode: 0, phase: Float(0.5) })")]
#[test_case(Operation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())), "BeamSplitter(BeamSplitter { mode_0: 0, mode_1: 1, theta: Float(0.1), phi: Float(0.5) })")]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)), "PhotonDetection(PhotonDetection { mode: 0, readout: \"ro\", readout_index: 0 })")]
fn debug(op: Operation, string: &str) {
    assert_eq!(format!("{:?}", op), string);
}

#[test_case(Operation::from(Squeezing::new(0, 0.5.into(), 0.0.into())), Operation::from(Squeezing::new(0, 0.5.into(), 0.0.into())), Operation::from(Squeezing::new(1, 0.5.into(), 0.0.into())))]
// #[test_case(Operation::from(PhaseDisplacement::new(0, 0.5.into(), 0.1.into())), Operation::from(PhaseDisplacement::new(0, 0.5.into(), 0.1.into())), Operation::from(PhaseDisplacement::new(1, 0.5.into(), 0.1.into())))]
#[test_case(Operation::from(PhaseShift::new(0, 0.5.into())), Operation::from(PhaseShift::new(0, 0.5.into())), Operation::from(PhaseShift::new(1, 0.5.into())))]
#[test_case(Operation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())), Operation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())), Operation::from(BeamSplitter::new(1, 2, 0.1.into(), 0.5.into())))]
#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)), Operation::from(PhotonDetection::new(0, "ro".into(), 0)), Operation::from(PhotonDetection::new(1, "ro".into(), 0)))]
fn partial_eq(op: Operation, op_0: Operation, op_1: Operation) {
    assert!(op_0 == op);
    assert!(op == op_0);
    assert!(op_1 != op);
    assert!(op != op_1);
}

#[test_case(ModeGateOperation::from(Squeezing::new(0, 0.1.into(), 0.0.into())), InvolvedQubits::None, InvolvedClassical::None, InvolvedModes::Set(HashSet::from([0_usize])))]
// #[test_case(ModeGateOperation::from(PhaseDisplacement::new(0, 0.1.into(), 0.1.into())), InvolvedQubits::None, InvolvedClassical::None, InvolvedModes::Set(HashSet::from([0_usize])))]
#[test_case(ModeGateOperation::from(PhaseShift::new(0, 0.1.into())), InvolvedQubits::None, InvolvedClassical::None, InvolvedModes::Set(HashSet::from([0_usize])))]
#[test_case(ModeGateOperation::from(BeamSplitter::new(0, 1, 0.5.into(), 0.1.into())), InvolvedQubits::None, InvolvedClassical::None, InvolvedModes::Set(HashSet::from([0_usize, 1_usize])))]
fn involved_qubits_classical_modes(
    op: ModeGateOperation,
    qubits: InvolvedQubits,
    classical: InvolvedClassical,
    modes: InvolvedModes,
) {
    assert_eq!(op.involved_qubits(), qubits);
    assert_eq!(op.involved_classical(), classical);
    assert_eq!(op.involved_modes(), modes);
}

#[test_case(SingleModeOperation::from(PhotonDetection::new(0, "ro".into(), 0)), InvolvedQubits::None, InvolvedClassical::Set(HashSet::from([("ro".into(), 0_usize)])), InvolvedModes::Set(HashSet::from([0_usize])))]
fn involved_qubits_classical_modes_measurement(
    op: SingleModeOperation,
    qubits: InvolvedQubits,
    classical: InvolvedClassical,
    modes: InvolvedModes,
) {
    assert_eq!(op.involved_qubits(), qubits);
    assert_eq!(op.involved_classical(), classical);
    assert_eq!(op.involved_modes(), modes);
}

#[test_case(ModeGateOperation::from(Squeezing::new(2, "test".into(), "test1".into())), ModeGateOperation::from(Squeezing::new(0, 0.1.into(), 0.5.into())))]
// #[test_case(ModeGateOperation::from(PhaseDisplacement::new(2, "test".into(), "test1".into())), ModeGateOperation::from(PhaseDisplacement::new(0, 0.1.into(), 0.5.into())))]
#[test_case(ModeGateOperation::from(PhaseShift::new(2, "test".into())), ModeGateOperation::from(PhaseShift::new(0, 0.1.into())))]
#[test_case(ModeGateOperation::from(BeamSplitter::new(2, 0, "test".into(), "test1".into())), ModeGateOperation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())))]
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

#[test_case(SingleModeOperation::from(PhotonDetection::new(2, "ro".into(), 0)), SingleModeOperation::from(PhotonDetection::new(0, "ro".into(), 0)))]
fn substitute_subsitutemodes_measurement(op: SingleModeOperation, op_test: SingleModeOperation) {
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

#[test_case(Operation::from(Squeezing::new(0, 0.1.into(), 0.0.into())), Operation::from(Squeezing::new(0, "param".into(), 0.0.into())), "Squeezing")]
// #[test_case(Operation::from(PhaseDisplacement::new(0, 0.1.into(), 0.5.into())), Operation::from(PhaseDisplacement::new(0, "param".into(), "param2".into())), "PhaseDisplacement")]
#[test_case(Operation::from(PhaseShift::new(0, 0.1.into())), Operation::from(PhaseShift::new(0, "param".into())), "PhaseShift")]
fn operate_one_mode(op: Operation, op_param: Operation, name: &str) {
    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        name,
    ];
    assert_eq!(op.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(op.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!op.is_parametrized());
    assert!(op_param.is_parametrized());
}

#[test_case(Operation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.1.into())), Operation::from(BeamSplitter::new(0, 1, 1.0.into(), "param".into())), "BeamSplitter")]
fn operate_two_modes(op: Operation, op_param: Operation, name: &str) {
    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "ModeGateOperation",
        "TwoModeGateOperation",
        name,
    ];
    assert_eq!(op.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(op.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!op.is_parametrized());
    assert!(op_param.is_parametrized());
}

#[test_case(Operation::from(PhotonDetection::new(0, "ro".into(), 0)), "PhotonDetection")]
fn operate_measurement(op: Operation, name: &str) {
    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Measurement", name];
    assert_eq!(op.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(op.hqslang(), String::from(name));

    // (3) Test is_parametrized function
    assert!(!op.is_parametrized());
}

#[test_case(SingleModeOperation::from(Squeezing::new(0, 0.5.into(), 0.0.into())))]
// #[test_case(SingleModeOperation::from(PhaseDisplacement::new(0, 0.5.into(), 0.1.into())))]
#[test_case(SingleModeOperation::from(PhaseShift::new(0, 0.5.into())))]
#[test_case(SingleModeOperation::from(PhotonDetection::new(0, "ro".into(), 0)))]
fn single_mode_op(op: SingleModeOperation) {
    assert_eq!(op.mode(), &0_usize);
}
#[test_case(SingleModeGateOperation::from(Squeezing::new(0, 0.5.into(), 0.0.into())))]
// #[test_case(SingleModeGateOperation::from(PhaseDisplacement::new(0, 0.5.into(), 0.1.into())))]
#[test_case(SingleModeGateOperation::from(PhaseShift::new(0, 0.5.into())))]
fn single_mode_gate_op(op: SingleModeGateOperation) {
    assert_eq!(op.mode(), &0_usize);
}

#[test_case(TwoModeOperation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())))]
fn two_mode_op(op: TwoModeOperation) {
    assert_eq!(op.mode_0(), &0_usize);
    assert_eq!(op.mode_1(), &1_usize);
}
#[test_case(TwoModeGateOperation::from(BeamSplitter::new(0, 1, 0.1.into(), 0.5.into())))]
fn two_mode_gate_op(op: TwoModeGateOperation) {
    assert_eq!(op.mode_0(), &0_usize);
    assert_eq!(op.mode_1(), &1_usize);
}

#[cfg(feature = "serialize")]
#[test]
fn squeezing_serde() {
    let op = Squeezing::new(0, 0.1.into(), 0.0.into());

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "Squeezing",
                len: 3,
            },
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("squeezing"),
            Token::F64(0.1),
            Token::Str("phase"),
            Token::F64(0.0),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "Squeezing",
                len: 3,
            },
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("squeezing"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.1),
            Token::Str("phase"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.0),
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "serialize")]
#[test]
fn phaseshift_serde() {
    let op = PhaseShift::new(0, 0.1.into());

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "PhaseShift",
                len: 2,
            },
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("phase"),
            Token::F64(0.1),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "PhaseShift",
                len: 2,
            },
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("phase"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.1),
            Token::StructEnd,
        ],
    );
}

// #[cfg(feature = "serialize")]
// #[test]
// fn phasedisplacement_serde() {
//     let op = PhaseDisplacement::new(0, 0.1.into(), 0.1.into());

//     assert_tokens(
//         &op.clone().readable(),
//         &[
//             Token::Struct {
//                 name: "PhaseDisplacement",
//                 len: 3,
//             },
//             Token::Str("mode"),
//             Token::U64(0),
//             Token::Str("displacement"),
//             Token::F64(0.1),
//             Token::Str("phase"),
//             Token::F64(0.1),
//             Token::StructEnd,
//         ],
//     );

//     assert_tokens(
//         &op.compact(),
//         &[
//             Token::Struct {
//                 name: "PhaseDisplacement",
//                 len: 3,
//             },
//             Token::Str("mode"),
//             Token::U64(0),
//             Token::Str("displacement"),
//             Token::NewtypeVariant {
//                 name: "CalculatorFloat",
//                 variant: "Float",
//             },
//             Token::F64(0.1),
//             Token::Str("phase"),
//             Token::NewtypeVariant {
//                 name: "CalculatorFloat",
//                 variant: "Float",
//             },
//             Token::F64(0.1),
//             Token::StructEnd,
//         ],
//     );
// }

#[cfg(feature = "serialize")]
#[test]
fn beamsplitter_serde() {
    let op = BeamSplitter::new(0, 1, 0.1.into(), 0.1.into());

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "BeamSplitter",
                len: 4,
            },
            Token::Str("mode_0"),
            Token::U64(0),
            Token::Str("mode_1"),
            Token::U64(1),
            Token::Str("theta"),
            Token::F64(0.1),
            Token::Str("phi"),
            Token::F64(0.1),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "BeamSplitter",
                len: 4,
            },
            Token::Str("mode_0"),
            Token::U64(0),
            Token::Str("mode_1"),
            Token::U64(1),
            Token::Str("theta"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.1),
            Token::Str("phi"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.1),
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "serialize")]
#[test]
fn photondetection_serde() {
    let op = PhotonDetection::new(0, "ro".into(), 0);

    assert_tokens(
        &op.clone().readable(),
        &[
            Token::Struct {
                name: "PhotonDetection",
                len: 3,
            },
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("readout_index"),
            Token::U64(0),
            Token::StructEnd,
        ],
    );

    assert_tokens(
        &op.compact(),
        &[
            Token::Struct {
                name: "PhotonDetection",
                len: 3,
            },
            Token::Str("mode"),
            Token::U64(0),
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("readout_index"),
            Token::U64(0),
            Token::StructEnd,
        ],
    );
}

#[cfg(feature = "json_schema")]
#[test]
fn squeezing_json_schema() {
    let def = Squeezing::new(0, 0.1.into(), 0.0.into());
    // Serialize
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(Squeezing);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

// #[cfg(feature = "json_schema")]
// #[test]
// fn phasedisplacement_json_schema() {
//     let def = PhaseDisplacement::new(0, 0.1.into(), 0.1.into());
//     // Serialize
//     let test_json = serde_json::to_string(&def).unwrap();
//     let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

//     // Create JSONSchema
//     let test_schema = schema_for!(PhaseDisplacement);
//     let schema = serde_json::to_string(&test_schema).unwrap();
//     let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
//     let compiled_schema = JSONSchema::options()
//         .with_draft(Draft::Draft7)
//         .compile(&schema_value)
//         .unwrap();

//     let validation_result = compiled_schema.validate(&test_value);
//     assert!(validation_result.is_ok());
// }

#[cfg(feature = "json_schema")]
#[test]
fn phaseshift_json_schema() {
    let def = PhaseShift::new(0, 0.1.into());
    // Serialize
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(PhaseShift);
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
fn beamsplitter_json_schema() {
    let def = BeamSplitter::new(0, 1, 0.3.into(), 0.4.into());
    // Serialize
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(BeamSplitter);
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
fn photondetection_json_schema() {
    let def = PhotonDetection::new(0, "test".to_string(), 0);
    // Serialize
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(PhotonDetection);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
