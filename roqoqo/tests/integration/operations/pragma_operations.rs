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

#[cfg(feature = "serialize")]
use bincode::serialize;
use ndarray::{array, Array1, Array2};
use num_complex::Complex64;
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::prelude::RoqoqoError;
use roqoqo::Circuit;
#[cfg(feature = "serialize")]
use serde_test::{assert_tokens, Configure, Token};
use std::collections::{HashMap, HashSet};

/// Test PragmaSetNumberOfMeasurements inputs and involved qubits
#[test]
fn pragma_set_number_of_measurements_inputs_qubits() {
    let pragma = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));

    // Test inputs are correct
    assert_eq!(pragma.number_measurements(), &1_usize);
    assert_eq!(pragma.readout(), &String::from("ro"));

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::None);
}

/// Test PragmaSetNumberOfMeasurements standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_set_number_of_measurements_simple_traits() {
    let pragma = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));
    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaSetNumberOfMeasurements { number_measurements: 1, readout: \"ro\" }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));
    let pragma_1 = PragmaSetNumberOfMeasurements::new(1, String::from("ro1"));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaSetNumberOfMeasurements Operate trait
#[test]
fn pragma_set_number_of_measurements_operate_trait() {
    let pragma = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));

    // (1) Test tags function
    let tags: &[&str; 3] = &[
        "Operation",
        "PragmaOperation",
        "PragmaSetNumberOfMeasurements",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(
        pragma.hqslang(),
        String::from("PragmaSetNumberOfMeasurements")
    );

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaSetNumberOfMeasurements Substitute trait
#[test]
fn pragma_set_number_of_measurements_substitute_trait() {
    let pragma = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));
    let pragma_test = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));
    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaSetNumberOfMeasurements Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_set_number_of_measurements_serde_readable() {
    let pragma_serialization = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaSetNumberOfMeasurements",
                len: 2,
            },
            Token::Str("number_measurements"),
            Token::U64(1),
            Token::Str("readout"),
            Token::Str("ro"),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSetNumberOfMeasurements Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_set_number_of_measurements_serde_compact() {
    let pragma_serialization = PragmaSetNumberOfMeasurements::new(1, String::from("ro"));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaSetNumberOfMeasurements",
                len: 2,
            },
            Token::Str("number_measurements"),
            Token::U64(1),
            Token::Str("readout"),
            Token::Str("ro"),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSetStateVector inputs and involved qubits
#[test]
fn pragma_set_statevector_inputs_qubits() {
    let statevec: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    let pragma = PragmaSetStateVector::new(statevec.clone());

    // Test inputs are correct
    assert_eq!(pragma.statevector(), &statevec.clone());

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaSetStateVector standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_set_statevector_simple_traits() {
    let statevec: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    let pragma = PragmaSetStateVector::new(statevec.clone());

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaSetStateVector { statevector: [Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }], shape=[4], strides=[1], layout=CFcf (0xf), const ndim=1 }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaSetStateVector::new(statevec.clone());
    let pragma_1 = PragmaSetStateVector::new(statevec.clone() + 1.0);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaSetStateVector Operate trait
#[test]
fn pragma_set_statevector_operate_trait() {
    let statevec: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    let pragma = PragmaSetStateVector::new(statevec.clone());

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaSetStateVector"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaSetStateVector"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaSetStateVector Substitute trait
#[test]
fn pragma_set_statevector_substitute_trait() {
    let statevec: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    let pragma = PragmaSetStateVector::new(statevec.clone());
    let pragma_test = PragmaSetStateVector::new(statevec.clone());

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaSetStateVector Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_set_statevector_serde_readable() {
    let statevec: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    let pragma_serialization = PragmaSetStateVector::new(statevec.clone());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaSetStateVector",
                len: 1,
            },
            Token::Str("statevector"),
            Token::Struct {
                name: "Array",
                len: 3,
            },
            Token::Str("v"),
            Token::U8(1),
            Token::Str("dim"),
            Token::Tuple { len: 1 },
            Token::U64(4),
            Token::TupleEnd,
            Token::Str("data"),
            Token::Seq { len: Some(4) },
            Token::Tuple { len: 2 },
            Token::F64(1.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSetStateVector Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_set_statevector_serde_compact() {
    let statevec: Array1<Complex64> = array![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0)
    ];
    let pragma_serialization = PragmaSetStateVector::new(statevec.clone());
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaSetStateVector",
                len: 1,
            },
            Token::Str("statevector"),
            Token::Struct {
                name: "Array",
                len: 3,
            },
            Token::Str("v"),
            Token::U8(1),
            Token::Str("dim"),
            Token::Tuple { len: 1 },
            Token::U64(4),
            Token::TupleEnd,
            Token::Str("data"),
            Token::Seq { len: Some(4) },
            Token::Tuple { len: 2 },
            Token::F64(1.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSetDensityMatrix inputs and involved qubits
#[test]
fn pragma_set_density_matrix_inputs_qubits() {
    let matrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    let pragma = PragmaSetDensityMatrix::new(matrix.clone());

    // Test inputs are correct
    assert_eq!(pragma.density_matrix(), &matrix.clone());

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaSetDensityMatrix standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_set_density_matrix_simple_traits() {
    let matrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    let pragma = PragmaSetDensityMatrix::new(matrix.clone());

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaSetDensityMatrix { density_matrix: [[Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
 [Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2 }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaSetDensityMatrix::new(matrix.clone());
    let pragma_1 = PragmaSetDensityMatrix::new(matrix.clone() + 1.0);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaSetDensityMatrix Operate trait
#[test]
fn pragma_set_density_matrix_operate_trait() {
    let matrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    let pragma = PragmaSetDensityMatrix::new(matrix.clone());

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaSetDensityMatrix"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaSetDensityMatrix"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaSetDensityMatrix Substitute trait
#[test]
fn pragma_set_density_matrix_substitute_trait() {
    let matrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    let pragma = PragmaSetDensityMatrix::new(matrix.clone());
    let pragma_test = PragmaSetDensityMatrix::new(matrix.clone());

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaSetDensityMatrix Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_set_density_matrix_serde_readable() {
    let matrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    let pragma_serialization = PragmaSetDensityMatrix::new(matrix.clone());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaSetDensityMatrix",
                len: 1,
            },
            Token::Str("density_matrix"),
            Token::Struct {
                name: "Array",
                len: 3,
            },
            Token::Str("v"),
            Token::U8(1),
            Token::Str("dim"),
            Token::Tuple { len: 2 },
            Token::U64(2),
            Token::U64(2),
            Token::TupleEnd,
            Token::Str("data"),
            Token::Seq { len: Some(4) },
            Token::Tuple { len: 2 },
            Token::F64(1.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSetDensityMatrix Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_set_density_matrix_serde_compact() {
    let matrix: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    let pragma_serialization = PragmaSetDensityMatrix::new(matrix.clone());
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaSetDensityMatrix",
                len: 1,
            },
            Token::Str("density_matrix"),
            Token::Struct {
                name: "Array",
                len: 3,
            },
            Token::Str("v"),
            Token::U8(1),
            Token::Str("dim"),
            Token::Tuple { len: 2 },
            Token::U64(2),
            Token::U64(2),
            Token::TupleEnd,
            Token::Str("data"),
            Token::Seq { len: Some(4) },
            Token::Tuple { len: 2 },
            Token::F64(1.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::Tuple { len: 2 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaRepeatGate inputs and involved qubits
#[test]
fn pragma_repeat_gate_inputs_qubits() {
    let pragma = PragmaRepeatGate::new(3);

    // Test inputs are correct
    assert_eq!(pragma.repetition_coefficient(), &3_usize);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaRepeatGate standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_repeat_gate_simple_inputs() {
    let pragma = PragmaRepeatGate::new(3);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaRepeatGate { repetition_coefficient: 3 }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaRepeatGate::new(3);
    let pragma_1 = PragmaRepeatGate::new(4);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaRepeatGate Operate trait
#[test]
fn pragma_repeat_gate_operate_trait() {
    let pragma = PragmaRepeatGate::new(3);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaRepeatGate"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaRepeatGate"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaRepeatGate Substitute trait
#[test]
fn pragma_repeat_gate_substitute_trait() {
    let pragma = PragmaRepeatGate::new(3);
    let pragma_test = PragmaRepeatGate::new(3);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaRepeatGate Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_repeat_gate_serde_readable() {
    let pragma_serialization = PragmaRepeatGate::new(3);
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaRepeatGate",
                len: 1,
            },
            Token::Str("repetition_coefficient"),
            Token::U64(3),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaRepeatGate Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_repeat_gate_serde_compact() {
    let pragma_serialization = PragmaRepeatGate::new(3);
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaRepeatGate",
                len: 1,
            },
            Token::Str("repetition_coefficient"),
            Token::U64(3),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaOverrotation inputs and involved qubits
#[test]
fn pragma_overrotation_inputs_qubits() {
    let pragma = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);

    // Test inputs are correct
    assert_eq!(pragma.gate_hqslang(), &"RotateX".to_string());
    assert_eq!(pragma.qubits(), &vec![0]);
    assert_eq!(pragma.amplitude(), &0.03);
    assert_eq!(pragma.variance(), &0.001);

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaOverrotation standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_overrotation_simple_inputs() {
    let pragma = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaOverrotation { gate_hqslang: \"RotateX\", qubits: [0], amplitude: 0.03, variance: 0.001 }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);
    let pragma_1 = PragmaOverrotation::new("RotateX".to_string(), vec![1], 0.03, 0.001);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaOverrotation Operate trait
#[test]
fn pragma_overrotation_operate_trait() {
    let pragma = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "MultiQubitOperation",
        "PragmaOperation",
        "PragmaOverrotation",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaOverrotation"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaOverrotation Substitute trait
#[test]
fn pragma_overrotation_substitute_trait() {
    let pragma = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);

    // (1) Substitute parameters function
    let pragma_test = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test = PragmaOverrotation::new("RotateX".to_string(), vec![2], 0.03, 0.001);
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(2, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 2 }));
}

/// Test PragmaOverrotation Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_overrotation_serde_readable() {
    let pragma_serialization = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaOverrotation",
                len: 4,
            },
            Token::Str("gate_hqslang"),
            Token::Str("RotateX"),
            Token::Str("qubits"),
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Str("amplitude"),
            Token::F64(0.03),
            Token::Str("variance"),
            Token::F64(0.001),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaOverrotation Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_overrotation_serde_compact() {
    let pragma_serialization = PragmaOverrotation::new("RotateX".to_string(), vec![0], 0.03, 0.001);
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaOverrotation",
                len: 4,
            },
            Token::Str("gate_hqslang"),
            Token::Str("RotateX"),
            Token::Str("qubits"),
            Token::Seq { len: Some(1) },
            Token::U64(0),
            Token::SeqEnd,
            Token::Str("amplitude"),
            Token::F64(0.03),
            Token::Str("variance"),
            Token::F64(0.001),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaBoostNoise inputs and involved qubits
#[test]
fn pragma_boost_noise_inputs_qubits() {
    let pragma = PragmaBoostNoise::new(CalculatorFloat::from(0.003));

    // Test inputs are correct
    assert_eq!(pragma.noise_coefficient(), &CalculatorFloat::from(0.003));

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::None);
}

/// Test PragmaBoostNoise standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_boost_noise_simple_traits() {
    let pragma = PragmaBoostNoise::new(CalculatorFloat::from(0.003));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaBoostNoise { noise_coefficient: Float(0.003) }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaBoostNoise::new(CalculatorFloat::from(0.003));
    let pragma_1 = PragmaBoostNoise::new(CalculatorFloat::from(0.004));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaBoostNoise Operate trait
#[test]
fn pragma_boost_noise_operate_trait() {
    let pragma = PragmaBoostNoise::new(CalculatorFloat::from(0.003));

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaBoostNoise"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaBoostNoise"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaBoostNoise Substitute trait
#[test]
fn pragma_boost_noise_substitute_trait() {
    let pragma = PragmaBoostNoise::new(CalculatorFloat::from(0.003));
    let pragma_test = PragmaBoostNoise::new(CalculatorFloat::from("test"));

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.003);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaBoostNoise Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_boost_noise_serde_readable() {
    let pragma_serialization = PragmaBoostNoise::new(CalculatorFloat::from(0.003));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaBoostNoise",
                len: 1,
            },
            Token::Str("noise_coefficient"),
            Token::F64(0.003),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaBoostNoise Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_boost_noise_serde_compact() {
    let pragma_serialization = PragmaBoostNoise::new(CalculatorFloat::from(0.003));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaBoostNoise",
                len: 1,
            },
            Token::Str("noise_coefficient"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.003),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaStopParallelBlock inputs and involved qubits
#[test]
fn pragma_stop_inputs_qubits() {
    let pragma = PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // Test inputs are correct
    assert_eq!(pragma.qubits(), &vec![0, 1]);
    assert_eq!(pragma.execution_time(), &CalculatorFloat::from(0.0000001));

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaStopParallelBlock standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_stop_simple_traits() {
    let pragma = PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // Test Debug trait
    let string_comparison = (format!("{:?}", pragma)
        == "PragmaStopParallelBlock { qubits: [0, 1], execution_time: Float(0.0000001) }")
        || (format!("{:?}", pragma)
            == "PragmaStopParallelBlock { qubits: [0, 1], execution_time: Float(1e-7) }");

    assert!(string_comparison);

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001));
    let pragma_1 = PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000002));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaStopParallelBlock Operate trait
#[test]
fn pragma_stop_operate_trait() {
    let pragma = PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "MultiQubitOperation",
        "PragmaOperation",
        "PragmaStopParallelBlock",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaStopParallelBlock"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaStopParallelBlock Substitute trait
#[test]
fn pragma_stop_substitute_trait() {
    let pragma = PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // (1) Substitute parameters function
    let pragma_test = PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from("test"));
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0000001);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test = PragmaStopParallelBlock::new(vec![2, 1], CalculatorFloat::from(0.0000001));
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(2, 0);
    qubit_mapping_test.insert(1, 1);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let mut qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_err.insert(1, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 2 }));
}

/// Test PragmaStopParallelBlock Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_stop_serde_readable() {
    let pragma_serialization =
        PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaStopParallelBlock",
                len: 2,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::Str("execution_time"),
            Token::F64(0.0000001),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaStopParallelBlock Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_stop_serde_compact() {
    let pragma_serialization =
        PragmaStopParallelBlock::new(vec![0, 1], CalculatorFloat::from(0.0000001));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaStopParallelBlock",
                len: 2,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::Str("execution_time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.0000001),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGlobalPhase inputs and involved qubits
#[test]
fn pragma_global_phase_inputs_qubits() {
    let pragma = PragmaGlobalPhase::new(CalculatorFloat::from(0.05));

    // Test inputs are correct
    assert_eq!(pragma.phase(), &CalculatorFloat::from(0.05));

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::None);
}

/// Test PragmaGlobalPhase standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_global_phase_simple_traits() {
    let pragma = PragmaGlobalPhase::new(CalculatorFloat::from(0.05));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaGlobalPhase { phase: Float(0.05) }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaGlobalPhase::new(CalculatorFloat::from(0.05));
    let pragma_1 = PragmaGlobalPhase::new(CalculatorFloat::from(0.06));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaGlobalPhase Operate trait
#[test]
fn pragma_global_phase_operate_trait() {
    let pragma = PragmaGlobalPhase::new(CalculatorFloat::from(0.05));

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaGlobalPhase"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaGlobalPhase"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaGlobalPhase Substitute trait
#[test]
fn pragma_global_phase_substitute_trait() {
    let pragma = PragmaGlobalPhase::new(CalculatorFloat::from(0.05));
    let pragma_test = PragmaGlobalPhase::new(CalculatorFloat::from("test"));

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.05);
    let result_test = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(pragma, result_test);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, result_test);
}

/// Test PragmaGlobalPhase Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_global_phase_serde_readable() {
    let pragma_serialization = PragmaGlobalPhase::new(CalculatorFloat::from(0.05));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaGlobalPhase",
                len: 1,
            },
            Token::Str("phase"),
            Token::F64(0.05),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGlobalPhase Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_global_phase_serde_compact() {
    let pragma_serialization = PragmaGlobalPhase::new(CalculatorFloat::from(0.05));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaGlobalPhase",
                len: 1,
            },
            Token::Str("phase"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.05),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSleep inputs and involved qubits
#[test]
fn pragma_sleep_inputs_qubits() {
    let pragma = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // Test inputs are correct
    assert_eq!(pragma.qubits(), &vec![0, 1]);
    assert_eq!(pragma.sleep_time(), &CalculatorFloat::from(0.0000001));

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaSleep standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_sleep_simple_traits() {
    let pragma = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // Test Debug trait
    let string_comparison = (format!("{:?}", pragma)
        == "PragmaSleep { qubits: [0, 1], sleep_time: Float(0.0000001) }")
        || (format!("{:?}", pragma) == "PragmaSleep { qubits: [0, 1], sleep_time: Float(1e-7) }");

    assert!(string_comparison);

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001));
    let pragma_1 = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000002));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaSleep Operate trait
#[test]
fn pragma_sleep_operate_trait() {
    let pragma = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "MultiQubitOperation",
        "PragmaOperation",
        "PragmaSleep",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaSleep"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaSleep Substitute trait
#[test]
fn pragma_sleep_substitute_trait() {
    let pragma = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001));

    // (1) Substitute parameters function
    let pragma_test = PragmaSleep::new(vec![0, 1], CalculatorFloat::from("test"));
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.0000001);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test = PragmaSleep::new(vec![2, 1], CalculatorFloat::from(0.0000001));
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(2, 0);
    qubit_mapping_test.insert(1, 1);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let mut qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_err.insert(1, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 2 }));
}

/// Test PragmaSleep Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_sleep_serde_readable() {
    let pragma_serialization = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaSleep",
                len: 2,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::Str("sleep_time"),
            Token::F64(0.0000001),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSleep Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_sleep_serde_compact() {
    let pragma_serialization = PragmaSleep::new(vec![0, 1], CalculatorFloat::from(0.0000001));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaSleep",
                len: 2,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::Str("sleep_time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.0000001),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaActiveReset inputs and involved qubits
#[test]
fn pragma_active_reset_inputs_qubits() {
    let pragma = PragmaActiveReset::new(0);

    // Test inputs are correct
    assert_eq!(pragma.qubit(), &0_usize);

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaActiveReset standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_active_reset_simple_traits() {
    let pragma = PragmaActiveReset::new(0);

    // Test Debug trait
    assert_eq!(format!("{:?}", pragma), "PragmaActiveReset { qubit: 0 }");

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaActiveReset::new(0);
    let pragma_1 = PragmaActiveReset::new(1);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaActiveReset Operate trait
#[test]
fn pragma_active_reset_operate_trait() {
    let pragma = PragmaActiveReset::new(0);

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "SingleQubitOperation",
        "PragmaOperation",
        "PragmaActiveReset",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaActiveReset"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaActiveReset Substitute trait
#[test]
fn pragma_active_reset_substitute_trait() {
    let pragma = PragmaActiveReset::new(0);

    // (1) Substitute parameters function
    let pragma_test = PragmaActiveReset::new(0);
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.05);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test = PragmaActiveReset::new(2);
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(2, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 2 }));
}

/// Test PragmaActiveReset Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_active_reset_serde_readable() {
    let pragma_serialization = PragmaActiveReset::new(0);
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaActiveReset",
                len: 1,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaActiveReset Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_active_reset_serde_compact() {
    let pragma_serialization = PragmaActiveReset::new(0);
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaActiveReset",
                len: 1,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaStartDecompositionBlock inputs and involved qubits
#[test]
fn pragma_start_decomp_block_inputs_qubits() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 0);
    reordering.insert(1, 1);
    let pragma = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());

    // Test inputs are correct
    assert_eq!(pragma.qubits(), &vec![0, 1]);
    assert_eq!(pragma.reordering_dictionary(), &reordering);

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaStartDecompositionBlock standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_start_decomp_block_simple_traits() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 1);
    let pragma = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaStartDecompositionBlock { qubits: [0, 1], reordering_dictionary: {0: 1} }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());
    let pragma_1 = PragmaStartDecompositionBlock::new(vec![0, 2], reordering.clone());
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaStartDecompositionBlock Operate trait
#[test]
fn pragma_start_decomp_block_operate_trait() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 0);
    reordering.insert(1, 1);
    let pragma = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "MultiQubitOperation",
        "PragmaOperation",
        "PragmaStartDecompositionBlock",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(
        pragma.hqslang(),
        String::from("PragmaStartDecompositionBlock")
    );

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaStartDecompositionBlock Substitute trait
#[test]
fn pragma_start_decomp_block_substitute_trait() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 0);
    reordering.insert(1, 1);
    let pragma = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());

    // (1) Substitute parameters function
    let pragma_test = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.05);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut reordering_test = HashMap::new();
    reordering_test.insert(1, 1);
    reordering_test.insert(0, 0);
    let pragma_test = PragmaStartDecompositionBlock::new(vec![1, 0], reordering_test);
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 1);
    qubit_mapping_test.insert(1, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaStartDecompositionBlock Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_start_decomp_block_serde_readable() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 1);
    let pragma_serialization = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaStartDecompositionBlock",
                len: 2,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::Str("reordering_dictionary"),
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::U64(1),
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaStartDecompositionBlock Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_start_decomp_block_serde_compact() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 1);
    let pragma_serialization = PragmaStartDecompositionBlock::new(vec![0, 1], reordering.clone());
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaStartDecompositionBlock",
                len: 2,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::Str("reordering_dictionary"),
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::U64(1),
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaStopDecompositionBlock inputs and involved qubits
#[test]
fn pragma_stop_decomp_block_inputs_qubits() {
    let pragma = PragmaStopDecompositionBlock::new(vec![0, 1]);

    // Test inputs are correct
    assert_eq!(pragma.qubits(), &vec![0, 1]);

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    qubits.insert(1);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaStopDecompositionBlock standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_stop_decomp_block_simple_traits() {
    let pragma = PragmaStopDecompositionBlock::new(vec![0, 1]);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaStopDecompositionBlock { qubits: [0, 1] }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaStopDecompositionBlock::new(vec![0, 1]);
    let pragma_1 = PragmaStopDecompositionBlock::new(vec![0, 2]);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaStopDecompositionBlock Operate trait
#[test]
fn pragma_stop_decomp_block_operate_trait() {
    let pragma = PragmaStopDecompositionBlock::new(vec![0, 1]);

    // (1) Test tags function
    let tags: &[&str; 4] = &[
        "Operation",
        "MultiQubitOperation",
        "PragmaOperation",
        "PragmaStopDecompositionBlock",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(
        pragma.hqslang(),
        String::from("PragmaStopDecompositionBlock")
    );

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaStopDecompositionBlock Substitute trait
#[test]
fn pragma_stop_decomp_block_substitute_trait() {
    let pragma = PragmaStopDecompositionBlock::new(vec![0, 1]);

    // (1) Substitute parameters function
    let pragma_test = PragmaStopDecompositionBlock::new(vec![0, 1]);
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.05);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test = PragmaStopDecompositionBlock::new(vec![0, 2]);
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 0);
    qubit_mapping_test.insert(2, 1);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let mut qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_err.insert(1, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 0 }));
}

/// Test PragmaStopDecompositionBlock Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_stop_decomp_block_serde_readable() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 1);
    let pragma_serialization = PragmaStopDecompositionBlock::new(vec![0, 1]);
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaStopDecompositionBlock",
                len: 1,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaStopDecompositionBlock Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_stop_decomp_block_serde_compact() {
    let mut reordering = HashMap::new();
    reordering.insert(0, 1);
    let pragma_serialization = PragmaStopDecompositionBlock::new(vec![0, 1]);
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaStopDecompositionBlock",
                len: 1,
            },
            Token::Str("qubits"),
            Token::Seq { len: Some(2) },
            Token::U64(0),
            Token::U64(1),
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDamping inputs and involved qubits
#[test]
fn pragma_damping_inputs_qubits() {
    let pragma = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // Test inputs are correct
    assert_eq!(pragma.qubit(), &0_usize);
    assert_eq!(pragma.gate_time(), &CalculatorFloat::from(0.005));
    assert_eq!(pragma.rate(), &CalculatorFloat::from(0.02));

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaDamping standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_damping_simple_traits() {
    let pragma = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaDamping { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    let pragma_1 = PragmaDamping::new(0, CalculatorFloat::from(0.006), CalculatorFloat::from(0.02));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaDamping Operate trait
#[test]
fn pragma_damping_operate_trait() {
    let pragma = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Test tags function
    let tags: &[&str; 6] = &[
        "Operation",
        "SingleQubitOperation",
        "PragmaOperation",
        "PragmaNoiseOperation",
        "PragmaNoiseProbaOperation",
        "PragmaDamping",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaDamping"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaDamping Substitute trait
#[test]
fn pragma_damping_substitute_trait() {
    let pragma = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Substitute parameters function
    let pragma_test = PragmaDamping::new(
        0,
        CalculatorFloat::from("test"),
        CalculatorFloat::from(0.02),
    );
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.005);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test =
        PragmaDamping::new(1, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(1, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 1 }));
}

/// Test PragmaDamping OperatePragmaNoise and OperatePragmaNoiseProba trait
#[test]
fn pragma_damping_pragmanoise_trait() {
    let pragma = PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Superoperator function
    let superop_prob: f64 = f64::try_from(pragma.probability()).unwrap();
    let superop_sqrt: f64 = (1.0 - superop_prob).sqrt();
    let superop: Array2<f64> = array![
        [1.0, 0.0, 0.0, superop_prob],
        [0.0, superop_sqrt, 0.0, 0.0],
        [0.0, 0.0, superop_sqrt, 0.0],
        [0.0, 0.0, 0.0, 1.0 - superop_prob],
    ];
    assert_eq!(superop, pragma.superoperator().unwrap());

    // (2) Probability function
    let proba_pre_exp: f64 = -2.0 * 0.005 * 0.02;
    let proba = CalculatorFloat::from(0.5 * (1.0 - proba_pre_exp.exp()));
    assert_eq!(proba, pragma.probability());

    // (3) PowerCF function
    let pragma_test = PragmaDamping::new(
        0,
        CalculatorFloat::from(0.005 * 1.5),
        CalculatorFloat::from(0.02),
    );
    assert_eq!(pragma_test, pragma.powercf(CalculatorFloat::from(1.5)));
}

/// Test PragmaDamping Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_damping_serde_readable() {
    let pragma_serialization =
        PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaDamping",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::F64(0.005),
            Token::Str("rate"),
            Token::F64(0.02),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDamping Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_damping_serde_compact() {
    let pragma_serialization =
        PragmaDamping::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaDamping",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.005),
            Token::Str("rate"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.02),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDepolarising inputs and involved qubits
#[test]
fn pragma_depolarising_inputs_qubits() {
    let pragma =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // Test inputs are correct
    assert_eq!(pragma.qubit(), &0_usize);
    assert_eq!(pragma.gate_time(), &CalculatorFloat::from(0.005));
    assert_eq!(pragma.rate(), &CalculatorFloat::from(0.02));

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaDepolarising standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_depolarising_simple_traits() {
    let pragma =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaDepolarising { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    let pragma_1 =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.006), CalculatorFloat::from(0.02));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaDepolarising Operate trait
#[test]
fn pragma_depolarising_operate_trait() {
    let pragma =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Test tags function
    let tags: &[&str; 6] = &[
        "Operation",
        "SingleQubitOperation",
        "PragmaOperation",
        "PragmaNoiseOperation",
        "PragmaNoiseProbaOperation",
        "PragmaDepolarising",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaDepolarising"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaDepolarising Substitute trait
#[test]
fn pragma_depolarising_substitute_trait() {
    let pragma =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Substitute parameters function
    let pragma_test = PragmaDepolarising::new(
        0,
        CalculatorFloat::from("test"),
        CalculatorFloat::from(0.02),
    );
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.005);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test =
        PragmaDepolarising::new(1, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(1, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 1 }));
}

/// Test PragmaDepolarising OperatePragmaNoise and OperatePragmaNoiseProba trait
#[test]
fn pragma_depolarising_pragmanoise_trait() {
    let pragma =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Superoperator function
    let superop_pre_exp: f64 = -1.0 * 0.005 * 0.02;
    let superop_prob: f64 = 0.75 * (1.0 - superop_pre_exp.exp());
    let superop_proba1: f64 = 1.0 - (2.0 / 3.0) * superop_prob.clone();
    let superop_proba2: f64 = 1.0 - (4.0 / 3.0) * superop_prob.clone();
    let superop_proba3: f64 = (2.0 / 3.0) * superop_prob.clone();
    let superop: Array2<f64> = array![
        [superop_proba1, 0.0, 0.0, superop_proba3],
        [0.0, superop_proba2, 0.0, 0.0],
        [0.0, 0.0, superop_proba2, 0.0],
        [superop_proba3, 0.0, 0.0, superop_proba1],
    ];
    assert_eq!(superop, pragma.superoperator().unwrap());

    // (2) Probability function
    let proba_pre_exp: f64 = -1.0 * 0.005 * 0.02;
    let proba = CalculatorFloat::from(0.75 * (1.0 - proba_pre_exp.exp()));
    assert_eq!(proba, pragma.probability());

    // (3) PowerCF function
    let pragma_test = PragmaDepolarising::new(
        0,
        CalculatorFloat::from(0.005 * 1.5),
        CalculatorFloat::from(0.02),
    );
    assert_eq!(pragma_test, pragma.powercf(CalculatorFloat::from(1.5)));
}

/// Test PragmaDepolarising Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_depolarising_serde_readable() {
    let pragma_serialization =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaDepolarising",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::F64(0.005),
            Token::Str("rate"),
            Token::F64(0.02),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDepolarising Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_depolarising_serde_compact() {
    let pragma_serialization =
        PragmaDepolarising::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaDepolarising",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.005),
            Token::Str("rate"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.02),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDephasing inputs and involved qubits
#[test]
fn pragma_dephasing_inputs_qubits() {
    let pragma = PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // Test inputs are correct
    assert_eq!(pragma.qubit(), &0_usize);
    assert_eq!(pragma.gate_time(), &CalculatorFloat::from(0.005));
    assert_eq!(pragma.rate(), &CalculatorFloat::from(0.02));

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaDephasing standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_dephasing_simple_traits() {
    let pragma = PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaDephasing { qubit: 0, gate_time: Float(0.005), rate: Float(0.02) }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 =
        PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    let pragma_1 =
        PragmaDephasing::new(0, CalculatorFloat::from(0.006), CalculatorFloat::from(0.02));
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaDephasing Operate trait
#[test]
fn pragma_dephasing_operate_trait() {
    let pragma = PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Test tags function
    let tags: &[&str; 6] = &[
        "Operation",
        "SingleQubitOperation",
        "PragmaOperation",
        "PragmaNoiseOperation",
        "PragmaNoiseProbaOperation",
        "PragmaDephasing",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaDephasing"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaDephasing Substitute trait
#[test]
fn pragma_dephasing_substitute_trait() {
    let pragma = PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Substitute parameters function
    let pragma_test = PragmaDephasing::new(
        0,
        CalculatorFloat::from("test"),
        CalculatorFloat::from(0.02),
    );
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.005);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test =
        PragmaDephasing::new(1, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(1, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 1 }));
}

/// Test PragmaDephasing OperatePragmaNoise and OperatePragmaNoiseProba trait
#[test]
fn pragma_dephasing_pragmanoise_trait() {
    let pragma = PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));

    // (1) Superoperator function
    let superop_pre_exp: f64 = -2.0 * 0.005 * 0.02;
    let superop_prob: f64 = 0.5 * (1.0 - superop_pre_exp.exp());
    let superop_proba1: f64 = 1.0 - 2.0 * superop_prob;
    let superop: Array2<f64> = array![
        [1.0, 0.0, 0.0, 0.0],
        [0.0, superop_proba1, 0.0, 0.0],
        [0.0, 0.0, superop_proba1, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    assert_eq!(superop, pragma.superoperator().unwrap());

    // (2) Probability function
    let proba_pre_exp: f64 = -2.0 * 0.005 * 0.02;
    let proba = CalculatorFloat::from(0.5 * (1.0 - proba_pre_exp.exp()));
    assert_eq!(proba, pragma.probability());

    // (3) PowerCF function
    let pragma_test = PragmaDephasing::new(
        0,
        CalculatorFloat::from(0.005 * 1.5),
        CalculatorFloat::from(0.02),
    );
    assert_eq!(pragma_test, pragma.powercf(CalculatorFloat::from(1.5)));
}

/// Test PragmaDephasing Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_dephasing_serde_readable() {
    let pragma_serialization =
        PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaDephasing",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::F64(0.005),
            Token::Str("rate"),
            Token::F64(0.02),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDephasing Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_dephasing_serde_compact() {
    let pragma_serialization =
        PragmaDephasing::new(0, CalculatorFloat::from(0.005), CalculatorFloat::from(0.02));
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaDephasing",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.005),
            Token::Str("rate"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.02),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaRandomNoise inputs and involved qubits
#[test]
fn pragma_random_noise_inputs_qubits() {
    let pragma = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );

    // Test inputs are correct
    assert_eq!(pragma.qubit(), &0_usize);
    assert_eq!(pragma.gate_time(), &CalculatorFloat::from(0.005));
    assert_eq!(pragma.depolarising_rate(), &CalculatorFloat::from(0.02));
    assert_eq!(pragma.dephasing_rate(), &CalculatorFloat::from(0.01));

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaRandomNoise standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_random_noise_simple_traits() {
    let pragma = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaRandomNoise { qubit: 0, gate_time: Float(0.005), depolarising_rate: Float(0.02), dephasing_rate: Float(0.01) }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );
    let pragma_1 = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.006),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaRandomNoise Operate trait
#[test]
fn pragma_random_noise_operate_trait() {
    let pragma = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );

    // (1) Test tags function
    let tags: &[&str; 6] = &[
        "Operation",
        "SingleQubitOperation",
        "PragmaOperation",
        "PragmaNoiseOperation",
        "PragmaNoiseProbaOperation",
        "PragmaRandomNoise",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaRandomNoise"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaRandomNoise Substitute trait
#[test]
fn pragma_random_noise_substitute_trait() {
    let pragma = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );

    // (1) Substitute parameters function
    let pragma_test = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from("test"),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.005);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test = PragmaRandomNoise::new(
        1,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(1, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 1 }));
}

/// Test PragmaRandomNoise OperatePragmaNoise and OperatePragmaNoiseProba trait
#[test]
fn pragma_random_noise_pragmanoise_trait() {
    let pragma = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );

    // (1) Superoperator function
    let superop_pre_exp: f64 = -2.0 * 0.005 * 0.01;
    let superop_prob: f64 = 0.5 * (1.0 - superop_pre_exp.exp());
    let superop_proba1: f64 = 1.0 - 2.0 * superop_prob;
    let superop: Array2<f64> = array![
        [1.0, 0.0, 0.0, 0.0],
        [0.0, superop_proba1, 0.0, 0.0],
        [0.0, 0.0, superop_proba1, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    assert_eq!(superop, pragma.superoperator().unwrap());

    // (2) Probability function
    let float_proba: f64 = 0.005 * (0.02 / 2.0 + 0.02 / 4.0 + 0.01);
    let proba = CalculatorFloat::from(float_proba);
    assert_eq!(proba, pragma.probability());

    // (3) PowerCF function
    let pragma_test = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005 * 1.5),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );
    assert_eq!(pragma_test, pragma.powercf(CalculatorFloat::from(1.5)));
}

/// Test PragmaRandomNoise Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_random_noise_serde_readable() {
    let pragma_serialization = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaRandomNoise",
                len: 4,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::F64(0.005),
            Token::Str("depolarising_rate"),
            Token::F64(0.02),
            Token::Str("dephasing_rate"),
            Token::F64(0.01),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaRandomNoise Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_random_noise_serde_compact() {
    let pragma_serialization = PragmaRandomNoise::new(
        0,
        CalculatorFloat::from(0.005),
        CalculatorFloat::from(0.02),
        CalculatorFloat::from(0.01),
    );
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaRandomNoise",
                len: 4,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.005),
            Token::Str("depolarising_rate"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.02),
            Token::Str("dephasing_rate"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.01),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGeneralNoise inputs and involved qubits
#[test]
fn pragma_general_noise_inputs_qubits() {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0],];
    let pragma = PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators.clone());

    // Test inputs are correct
    assert_eq!(pragma.qubit(), &0_usize);
    assert_eq!(pragma.gate_time(), &CalculatorFloat::from(0.005));
    assert_eq!(pragma.rates(), &operators);

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaGeneralNoise standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_general_noise_simple_traits() {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0],];
    let pragma = PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators.clone());

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaGeneralNoise { qubit: 0, gate_time: Float(0.005), rates: [[1.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 1.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2 }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators.clone());
    let pragma_1 = PragmaGeneralNoise::new(0, CalculatorFloat::from(0.006), operators.clone());
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaGeneralNoise Operate trait
#[test]
fn pragma_general_noise_operate_trait() {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],];
    let pragma = PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators.clone());

    // (1) Test tags function
    let tags: &[&str; 5] = &[
        "Operation",
        "SingleQubitOperation",
        "PragmaOperation",
        "PragmaNoiseOperation",
        "PragmaGeneralNoise",
    ];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaGeneralNoise"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaGeneralNoise Substitute trait
#[test]
fn pragma_general_noise_substitute_trait() {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],];
    let pragma = PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators.clone());

    // (1) Substitute parameters function
    let pragma_test = PragmaGeneralNoise::new(0, CalculatorFloat::from("test"), operators.clone());
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("test", 0.005);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let pragma_test = PragmaGeneralNoise::new(1, CalculatorFloat::from(0.005), operators.clone());
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(1, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    let qubit_mapping_err: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_err);
    assert_eq!(result, Err(RoqoqoError::QubitMappingError { qubit: 1 }));
}

/// Test PragmaGeneralNoise Operate trait
#[test]
fn pragma_general_noise_pragmanoise_trait() {
    let rates: Array2<f64> = array![[0.3, 0.0, 0.1], [0.7, 0.0, 0.0], [0.0, 0.8, 0.2]]; // add check for >= eigenvalues
    let pragma = PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), rates.clone());

    // matrix exponential using numpy:
    let test_exponential = array![
        [
            1.00000004e+00,
            3.13603590e-05,
            4.48130265e-03,
            5.97459826e-03
        ],
        [
            1.14712981e-02,
            9.95012493e-01,
            2.86751104e-06,
            2.49363597e-03
        ],
        [
            2.44347666e-05,
            1.39441142e-02,
            9.97004509e-01,
            1.74528238e-05
        ],
        [
            -3.25322253e-08,
            -2.78464407e-05,
            -3.97707102e-03,
            9.91536000e-01
        ]
    ];

    let result: Array2<f64> = test_exponential - pragma.superoperator().unwrap().t();
    for item in result.iter() {
        assert!(item.abs() <= 0.0001);
    }
}

/// Test PragmaGeneralNoise Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_general_noise_serde_readable() {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],];
    let pragma_serialization =
        PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators.clone());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaGeneralNoise",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::F64(0.005),
            Token::Str("rates"),
            Token::Struct {
                name: "Array",
                len: 3,
            },
            Token::Str("v"),
            Token::U8(1),
            Token::Str("dim"),
            Token::Tuple { len: 2 },
            Token::U64(3),
            Token::U64(3),
            Token::TupleEnd,
            Token::Str("data"),
            Token::Seq { len: Some(9) },
            Token::F64(1.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(1.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(1.0),
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaGeneralNoise Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_general_noise_serde_compact() {
    let operators: Array2<f64> = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],];
    let pragma_serialization =
        PragmaGeneralNoise::new(0, CalculatorFloat::from(0.005), operators.clone());
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaGeneralNoise",
                len: 3,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::Str("gate_time"),
            Token::NewtypeVariant {
                name: "CalculatorFloat",
                variant: "Float",
            },
            Token::F64(0.005),
            Token::Str("rates"),
            Token::Struct {
                name: "Array",
                len: 3,
            },
            Token::Str("v"),
            Token::U8(1),
            Token::Str("dim"),
            Token::Tuple { len: 2 },
            Token::U64(3),
            Token::U64(3),
            Token::TupleEnd,
            Token::Str("data"),
            Token::Seq { len: Some(9) },
            Token::F64(1.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(1.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(1.0),
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaConditional inputs and involved qubits
#[test]
fn pragma_conditional_inputs_qubits() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    let pragma = PragmaConditional::new(String::from("ro"), 1, circuit.clone());

    // Test inputs are correct
    assert_eq!(pragma.condition_register(), &String::from("ro"));
    assert_eq!(pragma.condition_index(), &1_usize);
    assert_eq!(pragma.circuit(), &circuit.clone());

    // Test InvolveQubits trait
    let mut qubits: HashSet<usize> = HashSet::new();
    qubits.insert(0);
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::Set(qubits));
}

/// Test PragmaConditional standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_conditional_simple_traits() {
    let pragma = PragmaConditional::new(String::from("ro"), 1, Circuit::default());

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaConditional { condition_register: \"ro\", condition_index: 1, circuit: Circuit { definitions: [], operations: [] } }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaConditional::new(String::from("ro"), 1, Circuit::default());
    let pragma_1 = PragmaConditional::new(String::from("ro1"), 1, Circuit::default());
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaConditional Operate trait
#[test]
fn pragma_conditional_operate_trait() {
    let pragma = PragmaConditional::new(String::from("ro"), 1, Circuit::default());

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaConditional"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaConditional"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaConditional Substitute trait
#[test]
fn pragma_conditional_substitute_trait() {
    let pragma = PragmaConditional::new(String::from("ro"), 1, Circuit::default());
    let pragma_test = PragmaConditional::new(String::from("ro"), 1, Circuit::default());

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(pragma, result);

    // (2) Remap qubits function with an empty circuit
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let mut new_qubit_paulis: HashMap<usize, usize> = HashMap::new();
    new_qubit_paulis.insert(2, 1);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);

    // (3) Remap qubits function with an non-empty circuit
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(2));
    let mut circuit_test = Circuit::new();
    circuit_test.add_operation(PauliX::new(0));
    let pragma = PragmaConditional::new(String::from("ro"), 1, circuit_test);
    let result = pragma.remap_qubits(&qubit_mapping_test).unwrap();
    let test_gate = PragmaConditional::new(String::from("ro"), 1, circuit);
    assert_eq!(result, test_gate);
}

/// Test PragmaConditional Serialization and Deserialization traits (readable)
#[cfg(feature = "serialize")]
#[test]
fn pragma_conditional_serde_readable() {
    let pragma_serialization = PragmaConditional::new(String::from("ro"), 1, Circuit::default());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaConditional",
                len: 3,
            },
            Token::Str("condition_register"),
            Token::Str("ro"),
            Token::Str("condition_index"),
            Token::U64(1),
            Token::Str("circuit"),
            Token::Struct {
                name: "Circuit",
                len: 2,
            },
            Token::Str("definitions"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::Str("operations"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaConditional Serialization and Deserialization traits (compact)
#[cfg(feature = "serialize")]
#[test]
fn pragma_conditional_serde_compact() {
    let pragma_serialization = PragmaConditional::new(String::from("ro"), 1, Circuit::default());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaConditional",
                len: 3,
            },
            Token::Str("condition_register"),
            Token::Str("ro"),
            Token::Str("condition_index"),
            Token::U64(1),
            Token::Str("circuit"),
            Token::Struct {
                name: "Circuit",
                len: 2,
            },
            Token::Str("definitions"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::Str("operations"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaChangeDevice inputs and involved qubits
#[test]
#[cfg(feature = "serialize")]
fn pragma_change_device_inputs_qubits() {
    // This is not a change device pragma, but for testing purposes it can be used
    let wrapped: Operation = PragmaActiveReset::new(0).into();
    let pragma = PragmaChangeDevice::new(&wrapped).unwrap();

    // Test inputs are correct
    assert_eq!(pragma.wrapped_hqslang, String::from("PragmaActiveReset"));
    let tags: &[&str; 4] = &[
        "Operation",
        "SingleQubitOperation",
        "PragmaOperation",
        "PragmaActiveReset",
    ];
    assert_eq!(pragma.wrapped_tags, tags);
    assert_eq!(pragma.wrapped_operation, serialize(&wrapped).unwrap());

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaConditional standard derived traits (Debug, Clone, PartialEq)
#[test]
#[cfg(feature = "serialize")]
fn pragma_change_device_simple_traits() {
    let wrapped: Operation = PragmaActiveReset::new(0).into();
    let pragma = PragmaChangeDevice::new(&wrapped).unwrap();
    let wrapped_0: Operation = PragmaActiveReset::new(0).into();
    let pragma_0 = PragmaChangeDevice::new(&wrapped_0).unwrap();

    let wrapped_1: Operation = PragmaActiveReset::new(1).into();
    let pragma_1 = PragmaChangeDevice::new(&wrapped_1).unwrap();
    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaConditional Operate trait
#[test]
#[cfg(feature = "serialize")]
fn pragma_change_device_operate_trait() {
    let wrapped: Operation = PragmaActiveReset::new(0).into();
    let pragma = PragmaChangeDevice::new(&wrapped).unwrap();

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaChangeDevice"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaChangeDevice"));

    // (3) Test is_parametrized function
    assert_eq!(pragma.is_parametrized(), false);
}

/// Test PragmaConditional Substitute trait
#[test]
#[cfg(feature = "serialize")]
fn pragma_change_device_substitute_trait() {
    let wrapped: Operation = PragmaActiveReset::new(0).into();
    let pragma = PragmaChangeDevice::new(&wrapped).unwrap();
    let pragma_test = PragmaChangeDevice::new(&wrapped).unwrap();
    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    assert_eq!(pragma, result);

    // (2) Remap qubits with a remapping
    // This is not supported yet and should throw an error
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    let mut new_qubit_paulis: HashMap<usize, usize> = HashMap::new();
    new_qubit_paulis.insert(2, 1);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).is_err();
    assert!(result);
}
