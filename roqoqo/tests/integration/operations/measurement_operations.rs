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
//
//! Integration test for public API of Measurement operations

use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::Circuit;
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{HashMap, HashSet};

/// Test MeasureQubit inputs, involved qubits and OperateSingleQubit qubit
#[test]
fn measure_qubit_inputs_qubits() {
    let measure = MeasureQubit::new(0, String::from("ro"), 1);

    // Test inputs are correct
    assert_eq!(measure.qubit(), &0_usize);
    assert_eq!(measure.readout(), &String::from("ro"));
    assert_eq!(measure.readout_index(), &1_usize);

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(measure.involved_qubits(), InvolvedQubits::Set(qubits));

    // Test OperateSingleQubit trait
    assert_eq!(measure.qubit(), &0_usize);
}

/// Test MeasureQubit standard derived traits (Debug, Clone, PartialEq)
#[test]
fn measure_qubit_simple_traits() {
    let measure = MeasureQubit::new(0, String::from("ro"), 1);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", measure),
        "MeasureQubit { qubit: 0, readout: \"ro\", readout_index: 1 }"
    );

    // Test Clone trait
    assert_eq!(measure.clone(), measure);

    // Test PartialEq trait
    let measure_0 = MeasureQubit::new(0, String::from("ro"), 1);
    let measure_1 = MeasureQubit::new(0, String::from("ro1"), 1);
    assert!(measure_0 == measure);
    assert!(measure == measure_0);
    assert!(measure_1 != measure);
    assert!(measure != measure_1);
}

/// Test MeasureQubit Operate trait
#[test]
fn measure_qubit_operate_trait() {
    let measure = MeasureQubit::new(0, String::from("ro"), 1);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "Measurement", "MeasureQubit"];
    assert_eq!(measure.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(measure.hqslang(), String::from("MeasureQubit"));

    // (3) Test is_parametrized function
    assert!(!measure.is_parametrized());
}

/// Test MeasureQubit Substitute trait
#[test]
fn measure_qubit_substitute_trait() {
    let measure = MeasureQubit::new(0, String::from("ro"), 1);

    // (1) Substitute parameters function
    let measure_test = MeasureQubit::new(0, String::from("ro"), 1);
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = measure_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(measure, result);

    // (2) Remap qubits function
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 2);
    qubit_mapping.insert(2, 0);
    let result = measure.remap_qubits(&qubit_mapping).unwrap();
    // comparison of gates
    let test_meas = MeasureQubit::new(2, String::from("ro"), 1);
    assert_eq!(result, test_meas);
}

/// Test MeasureQubit Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn measure_qubit_serde_readable() {
    let measure_serialization = MeasureQubit::new(0, String::from("ro"), 1);
    assert_tokens(
        &measure_serialization.readable(),
        &[
            Token::Struct {
                name: "MeasureQubit",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("readout_index"),
            Token::U64(1),
            Token::StructEnd,
        ],
    );
}

/// Test MeasureQubit Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn measure_qubit_serde_compact() {
    let measure_serialization = MeasureQubit::new(0, String::from("ro"), 1);
    assert_tokens(
        &measure_serialization.compact(),
        &[
            Token::Struct {
                name: "MeasureQubit",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("readout_index"),
            Token::U64(1),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetStateVector inputs and involved qubits
#[test]
fn pragma_get_statevector_inputs_qubits() {
    let pragma = PragmaGetStateVector::new(String::from("ro"), None);
    // Test inputs are correct
    assert_eq!(pragma.readout(), &String::from("ro"));
    assert_eq!(pragma.circuit(), &None);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaGetStateVector standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_get_statevector_simple_traits() {
    let pragma = PragmaGetStateVector::new(String::from("ro"), None);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaGetStateVector { readout: \"ro\", circuit: None }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaGetStateVector::new(String::from("ro"), None);
    let pragma_1 = PragmaGetStateVector::new(String::from("ro1"), None);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaGetStateVector Operate trait
#[test]
fn pragma_get_statevector_operate_trait() {
    let pragma = PragmaGetStateVector::new(String::from("ro"), None);

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "Measurement",
        "PragmaOperation",
        "PragmaGetStateVector",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaGetStateVector"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaGetStateVector Substitute trait
#[test]
fn pragma_get_statevector_substitute_trait() {
    let pragma = PragmaGetStateVector::new(String::from("ro"), None);

    // (1) Substitute parameters function
    // Works
    let pragma_test = PragmaGetStateVector::new(String::from("ro"), None);
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(pragma, result);
    // // Error
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateX::new(2, CalculatorFloat::from("theta")));
    // let pragma_test = PragmaGetStateVector::new(String::from("ro"), Some(circuit));
    // let mut calc = Calculator::new();
    // let gate_substituted = pragma_test.substitute_parameters(&mut calc);
    // assert!(gate_substituted.is_err());

    // (2) Remap qubits function with an empty circuit
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    qubit_mapping_test.insert(2, 0);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaGetStateVector::new(String::from("ro"), None);
    assert_eq!(result, test_gate);

    // (3) Remap qubits function with an non-empty circuit
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(2));
    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(PauliX::new(0));
    let pragma = PragmaGetStateVector::new(String::from("ro"), Some(circuit_test));
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaGetStateVector::new(String::from("ro"), Some(circuit));
    assert_eq!(result, test_gate)
}

/// Test PragmaGetStateVector Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_statevector_serde_readable() {
    let pragma_serialization = PragmaGetStateVector::new(String::from("ro"), None);
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaGetStateVector",
                len: 2,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::None,
            // Token::Struct {
            //     name: "Circuit",
            //     len: 2,
            // },
            // Token::Str("definitions"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::Str("operations"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetStateVector Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_statevector_serde_compact() {
    let pragma_serialization = PragmaGetStateVector::new(String::from("ro"), None);
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaGetStateVector",
                len: 2,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::None,
            // Token::Struct {
            //     name: "Circuit",
            //     len: 2,
            // },
            // Token::Str("definitions"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::Str("operations"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetDensityMatrix inputs and involved qubits
#[test]
fn pragma_get_density_matrix_inputs_qubits() {
    let pragma = PragmaGetDensityMatrix::new(String::from("ro"), None);

    // Test inputs are correct
    assert_eq!(pragma.readout(), &String::from("ro"));
    assert_eq!(pragma.circuit(), &None);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaGetDensityMatrix standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_get_density_matrix_simple_traits() {
    let pragma = PragmaGetDensityMatrix::new(String::from("ro"), None);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaGetDensityMatrix { readout: \"ro\", circuit: None }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaGetDensityMatrix::new(String::from("ro"), None);
    let pragma_1 = PragmaGetDensityMatrix::new(String::from("ro1"), None);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaGetDensityMatrix Operate trait
#[test]
fn pragma_get_density_matrix_operate_trait() {
    let pragma = PragmaGetDensityMatrix::new(String::from("ro"), None);

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "Measurement",
        "PragmaOperation",
        "PragmaGetDensityMatrix",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaGetDensityMatrix"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaGetDensityMatrix Substitute trait
#[test]
fn pragma_get_density_matrix_substitute_trait() {
    let pragma = PragmaGetDensityMatrix::new(String::from("ro"), None);

    // (1) Substitute parameters function
    let pragma_test = PragmaGetDensityMatrix::new(String::from("ro"), None);
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap(); // add something that is remapped -> check that remap is correctly called
    assert_eq!(pragma, result);
    // // Error
    // let mut circuit = Circuit::new();
    // circuit.add_operation(RotateX::new(2, "theta".into()));
    // let pragma_test = PragmaGetStateVector::new(String::from("ro"), Some(circuit));
    // let mut calc = Calculator::new();
    // let gate_substituted = pragma_test.substitute_parameters(&mut calc);
    // assert!(gate_substituted.is_err());

    // (2) Remap qubits function with an empty circuit
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    qubit_mapping_test.insert(2, 0);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaGetDensityMatrix::new(String::from("ro"), None);
    assert_eq!(result, test_gate);

    // (3) Remap qubits function with an non-empty circuit
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(2));
    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(PauliX::new(0));
    let pragma = PragmaGetDensityMatrix::new(String::from("ro"), Some(circuit_test));
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaGetDensityMatrix::new(String::from("ro"), Some(circuit));
    assert_eq!(result, test_gate)
}

/// Test PragmaGetDensityMatrix Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_density_matrix_serde_readable() {
    let pragma_serialization = PragmaGetDensityMatrix::new(String::from("ro"), None);

    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaGetDensityMatrix",
                len: 2,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::None,
            // Token::Struct {
            //     name: "Circuit",
            //     len: 2,
            // },
            // Token::Str("definitions"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::Str("operations"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetDensityMatrix Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_density_matrix_serde_compact() {
    let pragma_serialization = PragmaGetDensityMatrix::new(String::from("ro"), None);

    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaGetDensityMatrix",
                len: 2,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::None,
            // Token::Struct {
            //     name: "Circuit",
            //     len: 2,
            // },
            // Token::Str("definitions"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::Str("operations"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetOccupationProbability inputs and involved qubits
#[test]
fn pragma_get_occupation_probability_inputs_qubits() {
    let pragma = PragmaGetOccupationProbability::new(String::from("ro"), None);

    // Test inputs are correct
    assert_eq!(pragma.readout(), &String::from("ro"));
    assert_eq!(pragma.circuit(), &None);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaGetOccupationProbability standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_get_occupation_probability_simple_traits() {
    let pragma = PragmaGetOccupationProbability::new(String::from("ro"), None);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaGetOccupationProbability { readout: \"ro\", circuit: None }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaGetOccupationProbability::new(String::from("ro"), None);
    let pragma_1 = PragmaGetOccupationProbability::new(String::from("ro1"), None);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaGetOccupationProbability Operate trait
#[test]
fn pragma_get_occupation_probability_operate_trait() {
    let pragma = PragmaGetOccupationProbability::new(String::from("ro"), None);

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "Measurement",
        "PragmaOperation",
        "PragmaGetOccupationProbability",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(
        pragma.hqslang(),
        String::from("PragmaGetOccupationProbability")
    );

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaGetOccupationProbability Substitute trait
#[test]
fn pragma_get_occupation_probability_substitute_trait() {
    let pragma = PragmaGetOccupationProbability::new(String::from("ro"), None);

    // (1) Substitute parameters function
    let pragma_test = PragmaGetOccupationProbability::new(String::from("ro"), None);
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(pragma, result);
    // // Error
    // let mut circuit = Circuit::new();
    // circuit.add_operation(RotateX::new(2, "theta".into()));
    // let pragma_test = PragmaGetStateVector::new(String::from("ro"), Some(circuit));
    // let mut calc = Calculator::new();
    // let gate_substituted = pragma_test.substitute_parameters(&mut calc);
    // assert!(gate_substituted.is_err());

    // (2) Remap qubits function with an empty circuit
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    qubit_mapping_test.insert(2, 0);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaGetOccupationProbability::new(String::from("ro"), None);
    assert_eq!(result, test_gate);

    // (3) Remap qubits function with an non-empty circuit
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(2));
    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(PauliX::new(0));
    let pragma = PragmaGetOccupationProbability::new(String::from("ro"), Some(circuit_test));
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaGetOccupationProbability::new(String::from("ro"), Some(circuit));
    assert_eq!(result, test_gate)
}

/// Test PragmaGetOccupationProbability Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_occupation_probability_serde_readable() {
    let pragma_serialization = PragmaGetOccupationProbability::new(String::from("ro"), None);
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaGetOccupationProbability",
                len: 2,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::None,
            // Token::Struct {
            //     name: "Circuit",
            //     len: 2,
            // },
            // Token::Str("definitions"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::Str("operations"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetOccupationProbability Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_occupation_probability_serde_compact() {
    let pragma_serialization = PragmaGetOccupationProbability::new(String::from("ro"), None);
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaGetOccupationProbability",
                len: 2,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::None,
            // Token::Struct {
            //     name: "Circuit",
            //     len: 2,
            // },
            // Token::Str("definitions"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::Str("operations"),
            // Token::Seq { len: Some(0) },
            // Token::SeqEnd,
            // Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetPauliProduct inputs and involved qubits
#[test]
fn pragma_get_pauli_product_inputs_qubits() {
    let mut qubit_paulis: HashMap<usize, usize> = HashMap::new();
    qubit_paulis.insert(0, 1);
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    let pragma =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), circuit.clone());

    // Test inputs are correct
    assert_eq!(pragma.qubit_paulis(), &qubit_paulis.clone());
    assert_eq!(pragma.readout(), &String::from("ro"));
    assert_eq!(pragma.circuit(), &circuit.clone());

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaGetPauliProduct standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_get_pauli_product_simple_traits() {
    let mut qubit_paulis: HashMap<usize, usize> = HashMap::new();
    qubit_paulis.insert(0, 1);
    let pragma =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), Circuit::default());

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaGetPauliProduct { qubit_paulis: {0: 1}, readout: \"ro\", circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), Circuit::default());
    let pragma_1 = PragmaGetPauliProduct::new(
        qubit_paulis.clone(),
        String::from("ro1"),
        Circuit::default(),
    );
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaGetPauliProduct Operate trait
#[test]
fn pragma_get_pauli_product_operate_trait() {
    let mut qubit_paulis: HashMap<usize, usize> = HashMap::new();
    qubit_paulis.insert(0, 1);
    let pragma =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), Circuit::default());

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "Measurement",
        "PragmaOperation",
        "PragmaGetPauliProduct",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaGetPauliProduct"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaGetPauliProduct Substitute trait
#[test]
fn pragma_get_pauli_product_substitute_trait() {
    let mut qubit_paulis: HashMap<usize, usize> = HashMap::new();
    qubit_paulis.insert(0, 1);
    let pragma =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), Circuit::default());

    // (1) Substitute parameters function
    let pragma_test =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), Circuit::default());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(pragma, result);

    // (2) Remap qubits function with an empty circuit
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    qubit_mapping_test.insert(2, 0);
    let mut new_qubit_paulis: HashMap<usize, usize> = HashMap::new();
    new_qubit_paulis.insert(2, 1);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaGetPauliProduct::new(
        new_qubit_paulis.clone(),
        String::from("ro"),
        Circuit::default(),
    );
    assert_eq!(result, test_gate);

    // (3) Remap qubits function with an non-empty circuit
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(2));
    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(PauliX::new(0));
    let pragma = PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), circuit_test);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate =
        PragmaGetPauliProduct::new(new_qubit_paulis.clone(), String::from("ro"), circuit);
    assert_eq!(result, test_gate)
}

/// Test PragmaGetPauliProduct Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_pauli_product_serde_readable() {
    let mut qubit_paulis: HashMap<usize, usize> = HashMap::new();
    qubit_paulis.insert(0, 1);
    let pragma_serialization =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), Circuit::default());
    use roqoqo::ROQOQO_VERSION;
    use std::str::FromStr;
    let mut rsplit = ROQOQO_VERSION.split('.').take(2);
    let major_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Major version is not unsigned integer.");
    let minor_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Minor version is not unsigned integer.");

    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaGetPauliProduct",
                len: 3,
            },
            Token::Str("qubit_paulis"),
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::U64(1),
            Token::MapEnd,
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::Struct {
                name: "Circuit",
                len: 3,
            },
            Token::Str("definitions"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::Str("operations"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::Str("_roqoqo_version"),
            Token::Struct {
                name: "RoqoqoVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(major_version),
            Token::Str("minor_version"),
            Token::U32(minor_version),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGetPauliProduct Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_get_pauli_product_serde_compact() {
    let mut qubit_paulis: HashMap<usize, usize> = HashMap::new();
    qubit_paulis.insert(0, 1);
    let pragma_serialization =
        PragmaGetPauliProduct::new(qubit_paulis.clone(), String::from("ro"), Circuit::default());
    use roqoqo::ROQOQO_VERSION;
    use std::str::FromStr;
    let mut rsplit = ROQOQO_VERSION.split('.').take(2);
    let major_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Major version is not unsigned integer.");
    let minor_version = u32::from_str(
        rsplit
            .next()
            .expect("Internal error: Version not conforming to semver"),
    )
    .expect("Internal error: Minor version is not unsigned integer.");

    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaGetPauliProduct",
                len: 3,
            },
            Token::Str("qubit_paulis"),
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::U64(1),
            Token::MapEnd,
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("circuit"),
            Token::Struct {
                name: "Circuit",
                len: 3,
            },
            Token::Str("definitions"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::Str("operations"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::Str("_roqoqo_version"),
            Token::Struct {
                name: "RoqoqoVersionSerializable",
                len: 2,
            },
            Token::Str("major_version"),
            Token::U32(major_version),
            Token::Str("minor_version"),
            Token::U32(minor_version),
            Token::StructEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaRepeatedMeasurement inputs and involved qubits
#[test]
fn pragma_repeated_measurement_inputs_qubits() {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let pragma = PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));

    // Test inputs are correct
    assert_eq!(pragma.readout(), &String::from("ro"));
    assert_eq!(pragma.qubit_mapping(), &Some(qubit_mapping));
    assert_eq!(pragma.number_measurements(), &2_usize);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaRepeatedMeasurement standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_repeated_measurement_simple_traits() {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let pragma = PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaRepeatedMeasurement { readout: \"ro\", number_measurements: 2, qubit_mapping: Some({0: 1}) }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 =
        PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));
    let pragma_1 =
        PragmaRepeatedMeasurement::new(String::from("ro1"), 2, Some(qubit_mapping.clone()));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaRepeatedMeasurement Operate trait
#[test]
fn pragma_repeated_measurement_operate_trait() {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let pragma = PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "Measurement",
        "PragmaOperation",
        "PragmaRepeatedMeasurement",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaRepeatedMeasurement"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaRepeatedMeasurement Substitute trait
#[test]
fn pragma_repeated_measurement_substitute_trait() {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let pragma = PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));

    // (1) Substitute parameters function
    let pragma_test =
        PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(pragma, result);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    qubit_mapping_test.insert(2, 0);
    let mut new_qubit_mapping: HashMap<usize, usize> = HashMap::new();
    new_qubit_mapping.insert(2, 1);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate =
        PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(new_qubit_mapping.clone()));
    assert_eq!(result, test_gate);
}

/// Test PragmaRepeatedMeasurement Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_repeated_measurement_serde_readable() {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let pragma_serialization =
        PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));

    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaRepeatedMeasurement",
                len: 3,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("number_measurements"),
            Token::U64(2),
            Token::Str("qubit_mapping"),
            Token::Some,
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::U64(1),
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaRepeatedMeasurement Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_repeated_measurement_serde_compact() {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let pragma_serialization =
        PragmaRepeatedMeasurement::new(String::from("ro"), 2, Some(qubit_mapping.clone()));

    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaRepeatedMeasurement",
                len: 3,
            },
            Token::Str("readout"),
            Token::Str("ro"),
            Token::Str("number_measurements"),
            Token::U64(2),
            Token::Str("qubit_mapping"),
            Token::Some,
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::U64(1),
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}
