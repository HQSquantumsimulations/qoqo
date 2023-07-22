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

//! Integration test for public API of Define operations

#[cfg(feature = "json_schema")]
use jsonschema::{Draft, JSONSchema};
use qoqo_calculator::Calculator;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use schemars::schema_for;
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::HashMap;

/// Test DefinitionFloat inputs and involved qubits
#[test]
fn definition_float_inputs_qubits() {
    let def = DefinitionFloat::new(String::from("test"), 1, true);

    // Test inputs
    assert_eq!(def.name(), &String::from("test"));
    assert_eq!(def.length(), &1_usize);
    assert_eq!(def.is_output(), &true);

    // Test InvolveQubits trait
    assert_eq!(def.involved_qubits(), InvolvedQubits::None);
}

/// Test DefinitionFloat standard derived traits (Debug, Clone, PartialEq)
#[test]
fn definition_float_simple_traits() {
    let def = DefinitionFloat::new(String::from("test"), 1, true);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", def),
        "DefinitionFloat { name: \"test\", length: 1, is_output: true }"
    );

    // Test Clone trait
    assert_eq!(def.clone(), def);

    // Test PartialEq trait
    let def_0 = DefinitionFloat::new(String::from("test"), 1, true);
    let def_1 = DefinitionFloat::new(String::from("test"), 2, true);
    assert!(def_0 == def);
    assert!(def == def_0);
    assert!(def_1 != def);
    assert!(def != def_1);
}

/// Test DefinitionFloat Operate trait
#[test]
fn definition_float_operate_trait() {
    let def = DefinitionFloat::new(String::from("test"), 1, true);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Definition", "DefinitionFloat"];
    assert_eq!(def.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(def.hqslang(), String::from("DefinitionFloat"));

    // (3) Test is_parametrized function
    assert!(!def.is_parametrized());
}

/// Test DefinitionFloat Substitute trait
#[test]
fn definition_float_substitute_trait() {
    let def = DefinitionFloat::new(String::from("test"), 1, true);
    let def_test = DefinitionFloat::new(String::from("test"), 1, true);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0);
    let result = def_test.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(def, result);

    // (2) Remap qubits function
    let newqubit: usize = 2;
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, newqubit);
    qubit_mapping_test.insert(newqubit, 0);
    let result = def.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, def_test);
}

/// Test DefinitionFloat Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn definition_float_serde_readable() {
    let def = DefinitionFloat::new(String::from("test"), 1, true);

    assert_tokens(
        &def.readable(),
        &[
            Token::Struct {
                name: "DefinitionFloat",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionFloat Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn definition_float_serde_compact() {
    let def = DefinitionFloat::new(String::from("test"), 1, true);

    assert_tokens(
        &def.compact(),
        &[
            Token::Struct {
                name: "DefinitionFloat",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionComplex inputs and involved qubits
#[test]
fn definition_complex_inputs_qubits() {
    let def = DefinitionComplex::new(String::from("test"), 1, true);

    // Test inputs
    assert_eq!(def.name(), &String::from("test"));
    assert_eq!(def.length(), &1_usize);
    assert_eq!(def.is_output(), &true);

    // Test InvolveQubits trait
    assert_eq!(def.involved_qubits(), InvolvedQubits::None);
}

/// Test DefinitionComplex standard derived traits (Debug, Clone, PartialEq)
#[test]
fn definition_complex_simple_traits() {
    let def = DefinitionComplex::new(String::from("test"), 1, true);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", def),
        "DefinitionComplex { name: \"test\", length: 1, is_output: true }"
    );

    // Test Clone trait
    assert_eq!(def.clone(), def);

    // Test PartialEq trait
    let def_0 = DefinitionComplex::new(String::from("test"), 1, true);
    let def_1 = DefinitionComplex::new(String::from("test"), 2, true);
    assert!(def_0 == def);
    assert!(def == def_0);
    assert!(def_1 != def);
    assert!(def != def_1);
}

/// Test DefinitionComplex Operate trait
#[test]
fn definition_complex_operate_trait() {
    let def = DefinitionComplex::new(String::from("test"), 1, true);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Definition", "DefinitionComplex"];
    assert_eq!(def.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(def.hqslang(), String::from("DefinitionComplex"));

    // (3) Test is_parametrized function
    assert!(!def.is_parametrized());
}

/// Test DefinitionComplex Substitute trait
#[test]
fn definition_complex_substitute_trait() {
    let def = DefinitionComplex::new(String::from("test"), 1, true);
    let def_test = DefinitionComplex::new(String::from("test"), 1, true);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0);
    let result = def_test.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(def, result);

    // (2) Remap qubits function
    let newqubit: usize = 2;
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, newqubit);
    qubit_mapping_test.insert(newqubit, 0);
    let result = def.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, def_test);
}

/// Test DefinitionComplex Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn definition_complex_serde_readable() {
    let def = DefinitionComplex::new(String::from("test"), 1, true);

    assert_tokens(
        &def.readable(),
        &[
            Token::Struct {
                name: "DefinitionComplex",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionComplex Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn definition_complex_serde_compact() {
    let def = DefinitionComplex::new(String::from("test"), 1, true);

    assert_tokens(
        &def.compact(),
        &[
            Token::Struct {
                name: "DefinitionComplex",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionUsize inputs and involved qubits
#[test]
fn definition_usize_inputs_qubits() {
    let def = DefinitionUsize::new(String::from("test"), 1, true);

    // Test inputs are correct
    assert_eq!(def.name(), &String::from("test"));
    assert_eq!(def.length(), &1_usize);
    assert_eq!(def.is_output(), &true);

    // Test InvolveQubits trait
    assert_eq!(def.involved_qubits(), InvolvedQubits::None);
}

/// Test DefinitionUsize standard derived traits (Debug, Clone, PartialEq)
#[test]
fn definition_usize_simple_traits() {
    let def = DefinitionUsize::new(String::from("test"), 1, true);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", def),
        "DefinitionUsize { name: \"test\", length: 1, is_output: true }"
    );

    // Test Clone trait
    assert_eq!(def.clone(), def);

    // Test PartialEq trait
    let def_0 = DefinitionUsize::new(String::from("test"), 1, true);
    let def_1 = DefinitionUsize::new(String::from("test"), 2, true);
    assert!(def_0 == def);
    assert!(def == def_0);
    assert!(def_1 != def);
    assert!(def != def_1);
}

/// Test DefinitionUsize Operate trait
#[test]
fn definition_usize_operate_trait() {
    let def = DefinitionUsize::new(String::from("test"), 1, true);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Definition", "DefinitionUsize"];
    assert_eq!(def.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(def.hqslang(), String::from("DefinitionUsize"));

    // (3) Test is_parametrized function
    assert!(!def.is_parametrized());
}

/// Test DefinitionUsize Substitute trait
#[test]
fn definition_usize_substitute_trait() {
    let def = DefinitionUsize::new(String::from("test"), 1, true);
    let def_test = DefinitionUsize::new(String::from("test"), 1, true);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0);
    let result = def_test.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(def, result);

    // (2) Remap qubits function
    let newqubit: usize = 2;
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, newqubit);
    qubit_mapping_test.insert(newqubit, 0);
    let result = def.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, def_test);
}

/// Test DefinitionFloat Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn definition_usize_serde_readable() {
    let def = DefinitionUsize::new(String::from("test"), 1, true);

    assert_tokens(
        &def.readable(),
        &[
            Token::Struct {
                name: "DefinitionUsize",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionFloat Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn definition_usize_serde_compact() {
    let def = DefinitionUsize::new(String::from("test"), 1, true);

    assert_tokens(
        &def.compact(),
        &[
            Token::Struct {
                name: "DefinitionUsize",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionBit inputs and involved qubits
#[test]
fn definition_bit_inputs_qubits() {
    let def = DefinitionBit::new(String::from("test"), 1, false);

    // Test inputs are correct
    assert_eq!(def.name(), &String::from("test"));
    assert_eq!(def.length(), &1_usize);
    assert_eq!(def.is_output(), &false);

    // Test InvolveQubits trait
    assert_eq!(def.involved_qubits(), InvolvedQubits::None);
}

/// Test DefinitionBit standard derived traits (Debug, Clone, PartialEq)
#[test]
fn definition_bit_simple_traits() {
    let def = DefinitionBit::new(String::from("test"), 1, false);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", def),
        "DefinitionBit { name: \"test\", length: 1, is_output: false }"
    );

    // Test Clone trait
    assert_eq!(def.clone(), def);

    // Test PartialEq trait
    let def_0 = DefinitionBit::new(String::from("test"), 1, false);
    let def_1 = DefinitionBit::new(String::from("test"), 2, false);
    assert!(def_0 == def);
    assert!(def == def_0);
    assert!(def_1 != def);
    assert!(def != def_1);
}

/// Test DefinitionBit Operate trait
#[test]
fn definition_bit_operate_trait() {
    let def = DefinitionBit::new(String::from("test"), 1, false);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Definition", "DefinitionBit"];
    assert_eq!(def.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(def.hqslang(), String::from("DefinitionBit"));

    // (3) Test is_parametrized function
    assert!(!def.is_parametrized());
}

/// Test DefinitionBit Substitute trait
#[test]
fn definition_bit_substitute_trait() {
    let def = DefinitionBit::new(String::from("test"), 1, false);
    let def_test = DefinitionBit::new(String::from("test"), 1, false);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0);
    let result = def_test.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(def, result);

    // (2) Remap qubits function
    let newqubit: usize = 2;
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, newqubit);
    qubit_mapping_test.insert(newqubit, 0);
    let result = def.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, def_test);
}

/// Test DefinitionBit Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn definition_bit_serde_readable() {
    let def = DefinitionBit::new(String::from("test"), 1, false);

    assert_tokens(
        &def.readable(),
        &[
            Token::Struct {
                name: "DefinitionBit",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(false),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionBit Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn definition_bit_serde_compact() {
    let def = DefinitionBit::new(String::from("test"), 1, false);

    assert_tokens(
        &def.compact(),
        &[
            Token::Struct {
                name: "DefinitionBit",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("length"),
            Token::U64(1),
            Token::Str("is_output"),
            Token::Bool(false),
            Token::StructEnd,
        ],
    );
}

/// Test InputSymbolic inputs and involved qubits
#[test]
fn input_symbolic_inputs_qubits() {
    let def = InputSymbolic::new(String::from("test"), 1.0);

    // Test inputs are correct
    assert_eq!(def.name(), &String::from("test"));
    assert_eq!(def.input(), &1.0);

    // Test InvolveQubits trait
    assert_eq!(def.involved_qubits(), InvolvedQubits::None);
}

/// Test InputSymbolic standard derived traits (Debug, Clone, PartialEq)
#[test]
fn input_symbolic_simple_traits() {
    let def = InputSymbolic::new(String::from("test"), 1.0);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", def),
        "InputSymbolic { name: \"test\", input: 1.0 }"
    );

    // Test Clone trait
    assert_eq!(def.clone(), def);

    // Test PartialEq trait
    let def_0 = InputSymbolic::new(String::from("test"), 1.0);
    let def_1 = InputSymbolic::new(String::from("test"), 2.0);
    assert!(def_0 == def);
    assert!(def == def_0);
    assert!(def_1 != def);
    assert!(def != def_1);
}

/// Test InputSymbolic Operate trait
#[test]
fn input_symbolic_operate_trait() {
    let def = InputSymbolic::new(String::from("test"), 1.0);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Definition", "InputSymbolic"];
    assert_eq!(def.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(def.hqslang(), String::from("InputSymbolic"));

    // (3) Test is_parametrized function
    assert!(!def.is_parametrized());
}

/// Test InputSymbolic Substitute trait
#[test]
fn input_symbolic_substitute_trait() {
    let def = InputSymbolic::new(String::from("test"), 1.0);
    let def_test = InputSymbolic::new(String::from("test"), 1.0);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0);
    let result = def_test.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(def, result);

    // (2) Remap qubits function
    let newqubit: usize = 2;
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, newqubit);
    qubit_mapping_test.insert(newqubit, 0);
    let result = def.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, def_test);
}

/// Test InputSymbolic Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn input_symbolic_serde_readable() {
    let def = InputSymbolic::new(String::from("test"), 1.0);

    assert_tokens(
        &def.readable(),
        &[
            Token::Struct {
                name: "InputSymbolic",
                len: 2,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("input"),
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );
}

/// Test InputSymbolic Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn input_symbolic_serde_compact() {
    let def = InputSymbolic::new(String::from("test"), 1.0);

    assert_tokens(
        &def.compact(),
        &[
            Token::Struct {
                name: "InputSymbolic",
                len: 2,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("input"),
            Token::F64(1.0),
            Token::StructEnd,
        ],
    );
}

/// Test InputBit inputs and involved qubits
#[test]
fn input_bit_inputs_qubits() {
    let def = InputBit::new(String::from("test"), 1, false);

    // Test inputs are correct
    assert_eq!(def.name(), &String::from("test"));
    assert_eq!(def.index(), &1);
    assert_eq!(def.value(), &false);

    // Test InvolveQubits trait
    assert_eq!(def.involved_qubits(), InvolvedQubits::None);
}

/// Test InputBit standard derived traits (Debug, Clone, PartialEq)
#[test]
fn input_bit_simple_traits() {
    let def = InputBit::new(String::from("test"), 1, true);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", def),
        "InputBit { name: \"test\", index: 1, value: true }"
    );

    // Test Clone trait
    assert_eq!(def.clone(), def);

    // Test PartialEq trait
    let def_0 = InputBit::new(String::from("test"), 1, true);
    let def_1 = InputBit::new(String::from("test"), 2, false);
    assert!(def_0 == def);
    assert!(def == def_0);
    assert!(def_1 != def);
    assert!(def != def_1);
}

/// Test InputBit Operate trait
#[test]
fn input_bit_operate_trait() {
    let def = InputBit::new(String::from("test"), 1, false);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Definition", "InputBit"];
    assert_eq!(def.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(def.hqslang(), String::from("InputBit"));

    // (3) Test is_parametrized function
    assert!(!def.is_parametrized());
}

/// Test InputBit Substitute trait
#[test]
fn input_bit_substitute_trait() {
    let def = InputBit::new(String::from("test"), 1, true);
    let def_test = InputBit::new(String::from("test"), 1, true);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0);
    let result = def_test.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(def, result);

    // (2) Remap qubits function
    let newqubit: usize = 2;
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, newqubit);
    qubit_mapping_test.insert(newqubit, 0);
    let result = def.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, def_test);
}

/// Test InputBit Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn input_bit_serde_readable() {
    let def = InputBit::new(String::from("test"), 1, true);

    assert_tokens(
        &def.readable(),
        &[
            Token::Struct {
                name: "InputBit",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("index"),
            Token::U64(1),
            Token::Str("value"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test InputBit Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn input_bit_serde_compact() {
    let def = InputBit::new(String::from("test"), 1, true);

    assert_tokens(
        &def.compact(),
        &[
            Token::Struct {
                name: "InputBit",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("test"),
            Token::Str("index"),
            Token::U64(1),
            Token::Str("value"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

/// Test DefinitionFloat JsonSchema trait
#[cfg(feature = "json_schema")]
#[test]
pub fn definition_float_json_schema() {
    let def = DefinitionFloat::new(String::from("test"), 1, true);
    // Serialize Circuit
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(DefinitionFloat);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

/// Test DefinitionComplex JsonSchema trait
#[cfg(feature = "json_schema")]
#[test]
pub fn definition_complex_json_schema() {
    let def = DefinitionComplex::new(String::from("test"), 1, true);
    // Serialize Circuit
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(DefinitionComplex);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

/// Test DefinitionUsize JsonSchema trait
#[cfg(feature = "json_schema")]
#[test]
pub fn definition_usize_json_schema() {
    let def = DefinitionUsize::new(String::from("test"), 1, true);
    // Serialize Circuit
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(DefinitionUsize);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

/// Test DefinitionBit JsonSchema trait
#[cfg(feature = "json_schema")]
#[test]
pub fn definition_bit_json_schema() {
    let def = DefinitionBit::new(String::from("test"), 1, true);
    // Serialize Circuit
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(DefinitionBit);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}

/// Test InputBit JsonSchema trait
#[cfg(feature = "json_schema")]
#[test]
pub fn definition_input_bit_json_schema() {
    let def = InputBit::new(String::from("test"), 1, true);
    // Serialize Circuit
    let test_json = serde_json::to_string(&def).unwrap();
    let test_value: serde_json::Value = serde_json::from_str(&test_json).unwrap();

    // Create JSONSchema
    let test_schema = schema_for!(InputBit);
    let schema = serde_json::to_string(&test_schema).unwrap();
    let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .unwrap();

    let validation_result = compiled_schema.validate(&test_value);
    assert!(validation_result.is_ok());
}
