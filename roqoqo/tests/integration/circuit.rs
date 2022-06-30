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
// use jsonschema::{Draft, JSONSchema};
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::{AsVec, Circuit};
// #[cfg(feature = "serialize")]
// use schemars::schema_for;
use std::collections::{HashMap, HashSet};
#[cfg(feature = "overrotate")]
use std::convert::TryInto;
use std::iter::FromIterator;
use test_case::test_case;

/// Basic functional test
#[test]
fn it_works() {
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    circuit.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));
    assert!(circuit.get(0) == circuit.get(0));
}

#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
fn add_definitions(definition: Operation) {
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    circuit.add_operation(definition.clone());
    assert!(*circuit.get(0).unwrap() == definition);
    assert!(*circuit.get(1).unwrap() == RotateZ::new(0, CalculatorFloat::from(0.0)).into());
}

/// Test get function
#[test]
fn get_op() {
    let definition = Operation::from(DefinitionBit::new(String::from("ro"), 1, false));
    let operation = Operation::from(PauliX::new(0));
    let mut circuit = Circuit::new();
    circuit.add_operation(definition.clone());
    circuit.add_operation(operation.clone());

    assert_eq!(circuit.get(0).unwrap(), &definition);
    assert_eq!(circuit.get(1).unwrap(), &operation);
}

/// Test get_mut function
#[test]
fn get_mut_op() {
    let definition = Operation::from(DefinitionBit::new(String::from("ro"), 1, false));
    let operation = Operation::from(PauliX::new(0));
    let mut circuit = Circuit::new();
    circuit.add_operation(definition.clone());
    circuit.add_operation(operation.clone());

    assert_eq!(circuit.get_mut(0).unwrap(), &definition);
    assert_eq!(circuit.get_mut(1).unwrap(), &operation);
}

/// Test iter and next functions
#[test]
fn simple_iter() {
    let mut circuit = Circuit::new();
    circuit.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    circuit.add_operation(DefinitionBit::new(String::from("ro"), 1, false));
    circuit.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));
    let mut circuit_iter = circuit.iter();

    assert!(*circuit.get(0).unwrap() == *circuit_iter.next().unwrap());
    assert!(*circuit.get(1).unwrap() == *circuit_iter.next().unwrap());
    assert!(*circuit.get(2).unwrap() == *circuit_iter.next().unwrap());
}

/// Test is_parametrized function
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("test"))), true; "is")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0.5))), false; "is_not")]
fn is_parametrized(operation: Operation, parametrized: bool) {
    let mut circuit = Circuit::new();
    circuit.add_operation(operation);

    match parametrized {
        true => assert!(circuit.is_parametrized()),
        false => assert!(!circuit.is_parametrized()),
    }
}

/// Test len function
#[test]
fn length() {
    let mut circuit = Circuit::new();
    assert_eq!(circuit.len(), 0_usize);
    circuit.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    assert_eq!(circuit.len(), 1_usize);
    circuit.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    assert_eq!(circuit.len(), 2_usize);
    circuit.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));
    assert_eq!(circuit.len(), 3_usize);
}

/// Test is_empty function
#[test]
fn is_empty() {
    let mut circuit = Circuit::new();
    assert!(circuit.is_empty());
    circuit.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    assert!(!circuit.is_empty());
    circuit.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    assert!(!circuit.is_empty());
}

/// Test involved qubits
#[test_case(Operation::from(PragmaBoostNoise::new(CalculatorFloat::from(0.0))), InvolvedQubits::None; "none")]
#[test_case(Operation::from(PragmaGetOccupationProbability::new(String::from("ro"), None)), InvolvedQubits::All; "all")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0.5))), InvolvedQubits::Set([0].iter().cloned().collect()); "set_0")]
fn involved_qubits(operation: Operation, qubits: InvolvedQubits) {
    let mut circuit = Circuit::new();
    circuit.add_operation(operation);

    assert_eq!(circuit.involved_qubits(), qubits)
}

/// Test definitions and operations functions
#[test]
fn definitions_operations() {
    let mut circuit = Circuit::new();
    assert_eq!(circuit.definitions(), &Vec::new());
    assert_eq!(circuit.operations(), &Vec::new());

    let definition = DefinitionFloat::new(String::from("ro"), 1, false);
    let operation = RotateZ::new(0, CalculatorFloat::from(0.0));
    circuit.add_operation(definition.clone());
    circuit.add_operation(operation.clone());

    assert_eq!(circuit.definitions(), &vec![Operation::from(definition)]);
    assert_eq!(circuit.operations(), &vec![Operation::from(operation)]);
}

/// Test substitute_parameters function using Calculator
#[test]
fn substitute_params_calculator() {
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateX::new(0, CalculatorFloat::from(0.5)));

    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(RotateX::new(0, CalculatorFloat::from("test")));

    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.5);
    let result = circuit_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, circuit)
}

/// Test substitute_parameters function using InputSymbolic
#[test]
fn substitute_params_input_symbolic() {
    let mut circuit = Circuit::new();
    circuit.add_operation(InputSymbolic::new("test".to_string(), 0.5));
    circuit.add_operation(RotateX::new(0, CalculatorFloat::from(0.5)));

    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(InputSymbolic::new("test".to_string(), 0.5));
    circuit_test.add_operation(RotateX::new(0, CalculatorFloat::from("test")));

    let mut substitution_dict: Calculator = Calculator::new();
    let result = circuit_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, circuit)
}

/// Test remap_qubits function
#[test]
fn remap_qbits() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));

    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(PauliX::new(2));

    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(2, 0);
    qubit_mapping_test.insert(0, 2);
    let result = circuit_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, circuit)
}

/// Test count_occurences function
#[test]
fn count_occurences() {
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    circuit.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));
    circuit.add_operation(RotateX::new(1, CalculatorFloat::from(1.0)));
    circuit.add_operation(RotateX::new(1, CalculatorFloat::from(1.0)));
    circuit.add_operation(RotateX::new(1, CalculatorFloat::from(1.0)));
    circuit.add_operation(RotateX::new(1, CalculatorFloat::from(1.0)));
    assert!(circuit.count_occurences(&["RotateX"]) == 4);
    assert!(circuit.count_occurences(&["RotateZ"]) == 2);
    assert!(circuit.count_occurences(&["Rotation"]) == 6);
    assert!(circuit.count_occurences(&["Definition"]) == 0);
}

/// Test get_operation_types function
#[test]
fn test_get_operation_types() {
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    circuit.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));
    circuit.add_operation(RotateX::new(1, CalculatorFloat::from(1.0)));
    circuit.add_operation(RotateX::new(1, CalculatorFloat::from(1.0)));
    circuit.add_operation(RotateX::new(1, CalculatorFloat::from(1.0)));
    let mut test: HashSet<&str> = HashSet::new();
    let _ = test.insert("RotateZ");
    let _ = test.insert("RotateX");
    assert!(circuit.get_operation_types() == test);
    circuit.add_operation(DefinitionBit::new("a".to_string(), 1, false));
    let _ = test.insert("DefinitionBit");
    assert!(circuit.get_operation_types() == test);
}

/// Test indexing function
#[test]
fn index_access() {
    let mut circuit = Circuit::new();
    circuit.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    circuit.add_operation(RotateZ::new(20, CalculatorFloat::from("theta")));

    let comparison_def = Operation::from(DefinitionFloat::new(String::from("ro"), 1, false));
    let comparison_op = Operation::from(RotateZ::new(20, CalculatorFloat::from("theta")));

    assert!(circuit[0] == comparison_def);
    assert!(circuit[1] == comparison_op);
}

/// Test mutable indexing function
#[test]
fn mutable_index_access() {
    let mut circuit = Circuit::new();
    circuit.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    circuit.add_operation(RotateZ::new(20, CalculatorFloat::from("theta")));

    circuit[0] = Operation::from(DefinitionFloat::new(String::from("ro"), 1, true));
    circuit[1] = Operation::from(RotateZ::new(10, CalculatorFloat::from("theta")));
    let comparison_def = Operation::from(DefinitionFloat::new(String::from("ro"), 1, true));
    let comparison_op = Operation::from(RotateZ::new(10, CalculatorFloat::from("theta")));

    assert!(circuit[0] == comparison_def);
    assert!(circuit[1] == comparison_op);
}

/// Test into_iter and from_iter functions
#[test]
fn into_iter_from_iter() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    circuit.add_operation(PauliZ::new(1));

    let circuit_to = circuit.clone().into_iter();
    assert!(*circuit.get(0).unwrap() == circuit_to.clone().next().unwrap());
    assert!(*circuit.get(1).unwrap() == circuit_to.clone().nth(1).unwrap());

    assert_eq!(
        format!("{:?}", circuit_to),
        "OperationIterator { definition_iter: IntoIter([]), operation_iter: IntoIter([PauliX(PauliX { qubit: 0 }), PauliZ(PauliZ { qubit: 1 })]) }"
    );

    let circuit_from = Circuit::from_iter(circuit_to);
    assert_eq!(circuit, circuit_from)
}

/// Test extend function for the Iterator form of Circuit
#[test]
fn extend_iter() {
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    circuit.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));

    let circuit_to = circuit.clone().into_iter();
    assert!(*circuit.get(0).unwrap() == circuit_to.clone().next().unwrap());
    assert!(*circuit.get(1).unwrap() == circuit_to.clone().nth(1).unwrap());

    let mut circuit_from = Circuit::new();
    circuit_from.extend(circuit_to);
    assert_eq!(circuit, circuit_from)
}

/// Test default function
#[test]
fn default() {
    assert_eq!(Circuit::new(), Circuit::default());
}

/// Test AsVec trait
#[test]
fn as_vec() {
    let mut circuit = Circuit::new();
    let definition = DefinitionFloat::new(String::from("ro"), 1, false);
    let rotatez0 = RotateZ::new(0, CalculatorFloat::from(0.0));
    let rotatex1 = RotateX::new(1, CalculatorFloat::from(1.0));
    let pauliz0 = PauliZ::new(0);
    let paulix1 = PauliX::new(1);
    circuit.add_operation(definition.clone());
    circuit.add_operation(rotatez0.clone());
    circuit.add_operation(rotatex1.clone());
    circuit.add_operation(pauliz0.clone());
    circuit.add_operation(paulix1.clone());

    let vec_ops = vec![
        Operation::from(definition),
        Operation::from(rotatez0),
        Operation::from(rotatex1),
        Operation::from(pauliz0),
        Operation::from(paulix1),
    ];

    // Range
    assert_eq!(circuit.as_vec(0..5), None);
    assert_eq!(circuit.as_vec(0..1).unwrap(), vec_ops[0..1]);
    assert_eq!(circuit.as_vec(0..4).unwrap(), vec_ops[0..4]);
    assert_eq!(circuit.as_vec(1..3).unwrap(), vec_ops[1..3]);

    // RangeTo
    assert_eq!(circuit.as_vec(..5), None);
    assert_eq!(circuit.as_vec(..1).unwrap(), vec_ops[0..1]);
    assert_eq!(circuit.as_vec(..2).unwrap(), vec_ops[..2]);

    // RangeFrom
    assert_eq!(circuit.as_vec(0..).unwrap(), vec_ops[0..]);
    assert_eq!(circuit.as_vec(3..).unwrap(), vec_ops[3..]);
}

/// Test add_operation function
#[test]
fn add_op_circuit() {
    let mut circuit_one = Circuit::new();
    circuit_one.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));

    let operation = Operation::from(RotateZ::new(1, CalculatorFloat::from(1.0)));
    let circuit_plus_op = circuit_one.clone() + operation;

    let mut circuit_two = Circuit::new();
    circuit_two.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));
    let circuit_plus_circuit = circuit_one.clone() + circuit_two.clone();
    let circuit_plus_ref_circuit = circuit_one.clone() + &circuit_two;

    let mut circuit_overall = Circuit::new();
    circuit_overall.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    circuit_overall.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));

    assert_eq!(circuit_plus_op, circuit_overall);
    assert_eq!(circuit_plus_circuit, circuit_overall);
    assert_eq!(circuit_plus_ref_circuit, circuit_overall);
}

/// Test += function
#[test]
fn add_assign_op_circuit() {
    let mut circuit_one = Circuit::new();
    circuit_one.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    let mut circuit_two = circuit_one.clone();
    let mut circuit_three = circuit_one.clone();

    let operation = Operation::from(RotateZ::new(1, CalculatorFloat::from(1.0)));
    circuit_one += operation;

    let mut circuit_to_add = Circuit::new();
    circuit_to_add.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));
    circuit_two += circuit_to_add.clone();
    circuit_three += &circuit_to_add;

    let mut circuit_overall = Circuit::new();
    circuit_overall.add_operation(RotateZ::new(0, CalculatorFloat::from(0.0)));
    circuit_overall.add_operation(RotateZ::new(1, CalculatorFloat::from(1.0)));

    assert_eq!(circuit_one, circuit_overall);
    assert_eq!(circuit_two, circuit_overall);
    assert_eq!(circuit_three, circuit_overall);
}

/// Test Debug, Clone, Display and PartialEq traits
#[test]
fn simple_traits() {
    let mut circuit = Circuit::new();
    circuit.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    circuit.add_operation(PauliZ::new(0));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", circuit),
        "Circuit { definitions: [DefinitionFloat(DefinitionFloat { name: \"ro\", length: 1, is_output: false })], operations: [PauliZ(PauliZ { qubit: 0 })], _roqoqo_version: RoqoqoVersion }"
    );

    // Test Display trait
    assert_eq!(
        format!("{}", circuit),
        "DefinitionFloat(DefinitionFloat { name: \"ro\", length: 1, is_output: false })\nPauliZ(PauliZ { qubit: 0 })\n"
    );

    // Test Clone trait
    assert_eq!(circuit.clone(), circuit);

    // Test PartialEq trait
    let mut circuit_0 = Circuit::new();
    circuit_0.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    circuit_0.add_operation(PauliZ::new(0));
    let mut circuit_1 = Circuit::new();
    circuit_1.add_operation(DefinitionFloat::new(String::from("ro"), 1, false));
    circuit_1.add_operation(PauliZ::new(1));
    assert!(circuit_0 == circuit);
    assert!(circuit == circuit_0);
    assert!(circuit_1 != circuit);
    assert!(circuit != circuit_1);
}

/// Test overrotate circuit
#[test]
#[cfg(feature = "overrotate")]
fn test_overrotate() {
    let mut circuit = Circuit::new();
    circuit += PragmaOverrotation::new("RotateY".to_string(), vec![1], 20.0, 30.0);
    circuit += RotateX::new(0, 0.0.into());
    circuit += RotateY::new(0, 1.0.into());
    circuit += RotateY::new(1, 2.0.into());
    circuit += RotateY::new(1, 3.0.into());
    let circuit_overrotated = circuit.overrotate().unwrap();

    assert_eq!(circuit_overrotated[0], circuit[1]);
    assert_eq!(circuit_overrotated[1], circuit[2]);
    assert_eq!(circuit_overrotated[3], circuit[4]);
    assert_eq!(circuit_overrotated[2].hqslang(), "RotateY");
    let t: RotateY = circuit_overrotated[2].clone().try_into().unwrap();
    assert_eq!(t.qubit(), &1);
    assert_ne!(t.theta(), &2.0.into());
}

// #[cfg(feature = "json_schema")]
// #[test]
// fn test_basis_rotation_json() {
//     let mut circuit = Circuit::new();
//     circuit += RotateX::new(0, "theta".into());

//     // Serialize Circuit
//     let test_json = serde_json::to_string(&circuit).unwrap();
//     let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

//     // Create JSONSchema
//     let test_schema = schema_for!(Circuit);
//     let schema = serde_json::to_string(&test_schema).unwrap();
//     let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
//     let compiled_schema = JSONSchema::options()
//         .with_draft(Draft::Draft7)
//         .compile(&schema_value)
//         .unwrap();

//     let validation_result = compiled_schema.validate(&test_value);
//     assert!(validation_result.is_ok());
// }
