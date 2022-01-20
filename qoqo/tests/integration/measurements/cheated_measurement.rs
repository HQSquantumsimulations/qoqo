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

//! Integration test for public API of Basis rotation measurement

use bincode::serialize;
use num_complex::Complex64;
use pyo3::prelude::*;
use pyo3::Python;
use qoqo::measurements::{CheatedInputWrapper, CheatedWrapper};
use qoqo::CircuitWrapper;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
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
        let input = input_type
            .call1((2,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

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
        let input = input_type
            .call1((1,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

        let mut measured_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();
        let _ = measured_registers.insert("ro".to_string(), register);

        let input1: HashMap<String, BitOutputRegister> =
            HashMap::<String, BitOutputRegister>::new();
        let input2: HashMap<String, FloatOutputRegister> =
            HashMap::<String, FloatOutputRegister>::new();

        let result = br
            .call_method1("evaluate", (input1, input2, measured_registers))
            .unwrap();

        let test_diagonal_py = f64::extract(result.get_item("test_diagonal").unwrap()).unwrap();
        let test_off_diagonal_py =
            f64::extract(result.get_item("test_off_diagonal").unwrap()).unwrap();

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
        let input = input_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

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
        let input = input_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();
        let br_clone = &(*br);

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
        let input = input_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();
        let br_wrapper = br.extract::<CheatedWrapper>().unwrap();

        let br_clone = br_wrapper.clone();
        assert_eq!(format!("{:?}", br_wrapper), format!("{:?}", br_clone));

        let debug_string = "RefCell { value: CheatedWrapper { internal: Cheated { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }], input: CheatedInput { measured_operators: {\"test_diagonal\": ([(0, 0, Complex { re: 1.0, im: 0.0 }), (0, 1, Complex { re: 0.0, im: 0.0 }), (1, 0, Complex { re: 0.0, im: 0.0 }), (1, 1, Complex { re: -1.0, im: 0.0 })], \"ro\")}, number_qubits: 3 } } } }";
        assert_eq!(format!("{:?}", br), debug_string);

        let debug_input_string = "RefCell { value: CheatedInputWrapper { internal: CheatedInput { measured_operators: {\"test_diagonal\": ([(0, 0, Complex { re: 1.0, im: 0.0 }), (0, 1, Complex { re: 0.0, im: 0.0 }), (1, 0, Complex { re: 0.0, im: 0.0 }), (1, 1, Complex { re: -1.0, im: 0.0 })], \"ro\")}, number_qubits: 3 } } }";
        assert_eq!(format!("{:?}", input), debug_input_string);

        let debug_input = &(*input);
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
        let input = input_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

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

        let serialised: (&str, Vec<u8>) = br
            .call_method0("_internal_to_bincode")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(serialised.0, "Cheated");
        assert_eq!(serialised.1, comparison_serialised);
    })
}

/// Test to_json and from_json functions
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedInputWrapper>();
        let input = input_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

        let new_br = &(*br);
        let serialised = br.call_method0("to_json").unwrap();
        let deserialised = new_br
            .call_method1("from_json", (serialised,))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();
        assert_eq!(format!("{:?}", br), format!("{:?}", deserialised));

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
        let input = input_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

        let mut map: HashMap<String, f64> = HashMap::<String, f64>::new();
        map.insert("theta".to_string(), 0.0);
        let br_sub = br
            .call_method1("substitute_parameters", (map,))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

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
        let input = input_type
            .call1((3,))
            .unwrap()
            .cast_as::<PyCell<CheatedInputWrapper>>()
            .unwrap();
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
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedWrapper>>()
            .unwrap();

        let map: HashMap<String, f64> = HashMap::<String, f64>::new();
        let br_sub = br.call_method1("substitute_parameters", (map,));
        assert!(br_sub.is_err());
    })
}
