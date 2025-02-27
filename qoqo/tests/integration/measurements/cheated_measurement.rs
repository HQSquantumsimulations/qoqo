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

//! Integration test for public API of Basis rotation measurement

use bincode::serialize;
use num_complex::Complex64;
use pyo3::prelude::*;
use pyo3::Python;
use qoqo::measurements::{CheatedInputWrapper, CheatedWrapper};
use qoqo::CircuitWrapper;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use roqoqo::{
    measurements::{Cheated, CheatedInput},
    Circuit,
};
use std::collections::HashMap;
use test_case::test_case;

#[test]
fn test_returning_circuits() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((2,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let circuits: Vec<CircuitWrapper> = br.call_method0("circuits").unwrap().extract().unwrap();
        for (index, b) in circuits.iter().enumerate() {
            assert_eq!(b, circs.get(index).unwrap());
        }
        let const_circuit: CircuitWrapper = br
            .call_method0("constant_circuit")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(CircuitWrapper::new(), const_circuit);
    })
}

/// Test evaluate() function for Cheated measurement
#[test_case(vec![vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]], 1.0, 0.0; "simple_state_diagonal")]
#[test_case(vec![vec![Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)]], -1.0, 0.0; "simple_state_2_diagonal")]
#[test_case(vec![vec![Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0), Complex64::new(0.0, std::f64::consts::FRAC_1_SQRT_2)]], 0.0, 1.0; "simple_state_off_diagonal")]
#[test_case(vec![vec![Complex64::new(0.5, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.5, 0.0)]], 0.0, 0.0; "density_matrix_diagonal")]
#[test_case(vec![vec![Complex64::new(0.5, 0.0), Complex64::new(0.0, -0.5), Complex64::new(0.0, 0.5), Complex64::new(0.5, 0.0)]], 0.0, 1.0; "density_matrix_off_diagonal")]
#[test_case(vec![vec![Complex64::new(1.0,0.0), Complex64::new(0.0,0.0)], vec![Complex64::new(0.0,0.0), Complex64::new(1.0,0.0)], vec![Complex64::new(1.0,0.0), Complex64::new(0.0,0.0)]], 1.0/3.0, 0.0; "simple_state_diagonal_averaginv")]
fn test_py03_evaluate_bool(
    register: Vec<Vec<Complex64>>,
    value_diagonal: f64,
    value_off_diagonal: f64,
) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((1,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(0.0, 0.0)),
            (0, 1, Complex64::new(0.0, -1.0)),
            (1, 0, Complex64::new(0.0, 1.0)),
            (1, 1, Complex64::new(0.0, 0.0)),
        ];
        let _ = input
            .call_method1(
                "add_operator_exp_val",
                ("test_off_diagonal", test_matrix, "ro"),
            )
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let mut measured_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();
        let _ = measured_registers.insert("ro".to_string(), register);

        let input1: HashMap<String, BitOutputRegister> =
            HashMap::<String, BitOutputRegister>::new();
        let input2: HashMap<String, FloatOutputRegister> =
            HashMap::<String, FloatOutputRegister>::new();

        let result = br
            .call_method1("evaluate", (input1, input2, measured_registers))
            .unwrap();

        let test_diagonal_py =
            f64::extract_bound(&result.get_item("test_diagonal").unwrap()).unwrap();
        let test_off_diagonal_py =
            f64::extract_bound(&result.get_item("test_off_diagonal").unwrap()).unwrap();

        assert!((test_diagonal_py - value_diagonal).abs() < 1e-10);
        assert!((test_off_diagonal_py - value_off_diagonal).abs() < 1e-10);
    })
}

/// Test evaluate failure
#[test]
fn test_py03_evaluate_error0() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let register = vec![vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
        ]];

        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(0.0, 0.0)),
            (0, 1, Complex64::new(0.0, -1.0)),
            (1, 0, Complex64::new(0.0, 1.0)),
            (1, 1, Complex64::new(0.0, 0.0)),
        ];
        let _ = input
            .call_method1(
                "add_operator_exp_val",
                ("test_off_diagonal", test_matrix, "ro"),
            )
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let mut measured_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();
        let _ = measured_registers.insert("ro".to_string(), register);
        let input1: HashMap<String, BitOutputRegister> =
            HashMap::<String, BitOutputRegister>::new();
        let input2: HashMap<String, FloatOutputRegister> =
            HashMap::<String, FloatOutputRegister>::new();

        let result = br.call_method1(
            "evaluate",
            (input1, input2.clone(), measured_registers.clone()),
        );
        assert!(result.is_err());

        let mut input1: HashMap<String, Vec<Vec<usize>>> = HashMap::new();
        input1.insert("ro".to_string(), vec![vec![0, 0, 0]]);
        input1.insert("ro".to_string(), vec![vec![1, 1, 1]]);
        let error = br.call_method1("evaluate", (input1, input2, measured_registers));
        assert!(error.is_err());
    })
}

/// Test copy
#[test]
fn test_pyo3_copy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();
        let br_clone = br;

        let circuits: Vec<CircuitWrapper> = br.call_method0("circuits").unwrap().extract().unwrap();
        let circuits_clone: Vec<CircuitWrapper> = br_clone
            .call_method0("circuits")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(circuits, circuits_clone);

        let const_circuit: CircuitWrapper = br
            .call_method0("constant_circuit")
            .unwrap()
            .extract()
            .unwrap();
        let const_circuit_clone: CircuitWrapper = br_clone
            .call_method0("constant_circuit")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(const_circuit, const_circuit_clone);
    })
}

/// Test debug and clone
#[test]
fn test_pyo3_debug() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();
        let br_wrapper = br.extract::<CheatedWrapper>().unwrap();

        #[allow(clippy::redundant_clone)]
        let br_clone = br_wrapper.clone();
        assert_eq!(format!("{:?}", br_wrapper), format!("{:?}", br_clone));

        let debug_string = "CheatedWrapper { internal: Cheated { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }], input: CheatedInput { measured_operators: {\"test_diagonal\": ([(0, 0, Complex { re: 1.0, im: 0.0 }), (0, 1, Complex { re: 0.0, im: 0.0 }), (1, 0, Complex { re: 0.0, im: 0.0 }), (1, 1, Complex { re: -1.0, im: 0.0 })], \"ro\")}, number_qubits: 3 } } }";
        assert_eq!(format!("{:?}", br.borrow()), debug_string);

        let debug_input_string = "CheatedInputWrapper { internal: CheatedInput { measured_operators: {\"test_diagonal\": ([(0, 0, Complex { re: 1.0, im: 0.0 }), (0, 1, Complex { re: 0.0, im: 0.0 }), (1, 0, Complex { re: 0.0, im: 0.0 }), (1, 1, Complex { re: -1.0, im: 0.0 })], \"ro\")}, number_qubits: 3 } }";
        assert_eq!(format!("{:?}", input.borrow()), debug_input_string);

        let debug_input = input;
        let error = debug_input.call_method1(
            "add_operator_exp_val",
            (
                "test_diagonal",
                vec![(0, 0, Complex64::new(0.0, 0.0))],
                "ro",
            ),
        );
        assert!(error.is_err());
    })
}

/// Test _internal_to_bincode function
#[test]
fn test_internal_to_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1(
                "add_operator_exp_val",
                ("test_diagonal", test_matrix.clone(), "ro"),
            )
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let mut roqoqo_bri = CheatedInput::new(3);
        roqoqo_bri
            .add_operator_exp_val("test_diagonal".to_string(), test_matrix, "ro".to_string())
            .unwrap();
        let circs: Vec<Circuit> = vec![Circuit::new()];
        let roqoqo_br = Cheated {
            constant_circuit: Some(Circuit::new()),
            circuits: circs,
            input: roqoqo_bri,
        };
        let comparison_serialised = serialize(&roqoqo_br).unwrap();

        let serialised: (String, Vec<u8>) = br
            .call_method0("_internal_to_bincode")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(serialised.0, "Cheated");
        assert_eq!(serialised.1, comparison_serialised);
    })
}

/// Test to_bincode and from_bincode functions
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let serialised = input.call_method0("to_bincode").unwrap();
        let new_input = input;
        let binding = new_input
            .call_method1("from_bincode", (&serialised,))
            .unwrap();
        let deserialised = binding.downcast::<CheatedInputWrapper>().unwrap();
        assert_eq!(format!("{:?}", input), format!("{:?}", deserialised));

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let new_br = br;

        let serialised = br.call_method0("to_bincode").unwrap();
        let binding = new_br.call_method1("from_bincode", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<CheatedWrapper>().unwrap();
        assert_eq!(
            format!("{:?}", br.as_ref()),
            format!("{:?}", deserialised.as_ref())
        );

        let deserialised_error =
            new_br.call_method1("from_bincode", (bincode::serialize("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new_br.call_method1("from_bincode", (bincode::serialize(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());
    })
}

/// Test to_json and from_json functions
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let serialised = input.call_method0("to_json").unwrap();
        let new_input = input;
        let binding = new_input.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<CheatedInputWrapper>().unwrap();
        assert_eq!(format!("{:?}", input), format!("{:?}", deserialised));

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let binding = new_br.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<CheatedWrapper>().unwrap();
        assert_eq!(
            format!("{:?}", br.as_ref()),
            format!("{:?}", deserialised.as_ref())
        );

        let deserialised_error =
            new_br.call_method1("from_json", (serde_json::to_string("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new_br.call_method1("from_json", (serde_json::to_string(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());
    })
}

/// Test substitute_parameters
#[test]
fn test_substitute_parameters() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let mut map: HashMap<String, f64> = HashMap::<String, f64>::new();
        map.insert("theta".to_string(), 0.0);
        let binding = br.call_method1("substitute_parameters", (map,)).unwrap();
        let br_sub = binding.downcast::<CheatedWrapper>().unwrap();

        let br_wrapper = br.extract::<CheatedWrapper>().unwrap();
        let br_sub_wrapper = br_sub.extract::<CheatedWrapper>().unwrap();
        assert_ne!(format!("{:?}", br_wrapper), format!("{:?}", br_sub_wrapper));
    })
}

/// Test substitute_parameters returning an error
#[test]
fn test_substitute_parameters_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let map: HashMap<String, f64> = HashMap::<String, f64>::new();
        let br_sub = br.call_method1("substitute_parameters", (map,));
        assert!(br_sub.is_err());
    })
}

/// Test measurement_type()
#[test]
fn test_measurement_type() {
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let measurement_type = br.call_method0("measurement_type").unwrap();
        assert_eq!(measurement_type.to_string(), "Cheated");
    })
}

/// Test input()
#[test]
fn test_return_input() {
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();

        let binding = br.call_method0("input").unwrap();
        let input_returned = binding.downcast::<CheatedInputWrapper>().unwrap();

        assert_eq!(format!("{:?}", input_returned), format!("{:?}", input));
    })
}

#[test]
fn test_pyo3_format_repr() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let format_repr = "Cheated { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }], input: CheatedInput { measured_operators: {\"test_diagonal\": ([(0, 0, Complex { re: 1.0, im: 0.0 }), (0, 1, Complex { re: 0.0, im: 0.0 }), (1, 0, Complex { re: 0.0, im: 0.0 }), (1, 1, Complex { re: -1.0, im: 0.0 })], \"ro\")}, number_qubits: 3 } }";
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();
        let to_format = br.call_method1("__format__", ("",)).unwrap();
        let format_op: String = String::extract_bound(&to_format).unwrap();
        let to_repr = br.call_method0("__repr__").unwrap();
        let repr_op: String = String::extract_bound(&to_repr).unwrap();
        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    })
}

#[test]
fn test_pyo3_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedWrapper>().unwrap();
        let copy_op = br.call_method0("__copy__").unwrap();
        let deepcopy_op = br.call_method1("__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = br;

        let comparison_copy = bool::extract_bound(
            &copy_op
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_op
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

#[test]
fn test_pyo3_richcmp() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br_one = binding.downcast::<CheatedWrapper>().unwrap();
        let arg: Option<CircuitWrapper> = None;
        let binding = br_type.call1((arg, circs, input)).unwrap();
        let br_two = binding.downcast::<CheatedWrapper>().unwrap();
        let comparison =
            bool::extract_bound(&br_one.call_method1("__eq__", (br_two,)).unwrap()).unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract_bound(&br_one.call_method1("__ne__", (br_two,)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = br_one.call_method1("__ge__", (br_two,));
        assert!(comparison.is_err());
    })
}

/// Test json_schema function
#[cfg(feature = "json_schema")]
#[test]
fn test_pyo3_json_schema() {
    let rust_schema_input =
        serde_json::to_string_pretty(&schemars::schema_for!(CheatedInput)).unwrap();
    let rust_schema = serde_json::to_string_pretty(&schemars::schema_for!(Cheated)).unwrap();
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let binding = input_type.call1((3,)).unwrap();
        let input = binding.downcast::<CheatedInputWrapper>().unwrap();
        let test_matrix = vec![
            (0, 0, Complex64::new(1.0, 0.0)),
            (0, 1, Complex64::new(0.0, 0.0)),
            (1, 0, Complex64::new(0.0, 0.0)),
            (1, 1, Complex64::new(-1.0, 0.0)),
        ];
        let _ = input
            .call_method1("add_operator_exp_val", ("test_diagonal", test_matrix, "ro"))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedWrapper>();
        #[allow(clippy::redundant_clone)]
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br_one = binding.downcast::<CheatedWrapper>().unwrap();

        let schema_input: String =
            String::extract_bound(&input.call_method0("json_schema").unwrap()).unwrap();
        let schema: String =
            String::extract_bound(&br_one.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema_input, rust_schema_input);
        assert_eq!(schema, rust_schema);

        let current_version_string_input =
            String::extract_bound(&input.call_method0("current_version").unwrap()).unwrap();
        let current_version_string =
            String::extract_bound(&br_one.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string_input =
            String::extract_bound(&input.call_method0("min_supported_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&br_one.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(current_version_string_input, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.0.0");
        assert_eq!(minimum_supported_version_string_input, "1.0.0");
    });
}
