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
use pyo3::prelude::*;
use pyo3::Python;
use qoqo::measurements::{CheatedPauliZProductInputWrapper, CheatedPauliZProductWrapper};
use qoqo::CircuitWrapper;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use roqoqo::{
    measurements::{CheatedPauliZProduct, CheatedPauliZProductInput},
    Circuit,
};
use std::collections::HashMap;

#[test]
fn test_returning_circuits() {
    Python::with_gil(|py| {
        pyo3::prepare_freethreaded_python();

        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
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

/// Test evaluate() function for CheatedPauliZProduct measurement
#[test]
fn test_py03_evaluate_bool() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_0",))
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_1",))
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_2",))
            .unwrap();

        let mut linear_map: HashMap<usize, f64> = HashMap::new();
        linear_map.insert(0, 3.0);
        let _ = input
            .call_method1("add_linear_exp_val", ("single_pp_val", linear_map))
            .unwrap();
        let mut linear_map: HashMap<usize, f64> = HashMap::new();
        linear_map.insert(0, 4.0);
        linear_map.insert(1, 5.0);
        linear_map.insert(2, 6.0);
        let _ = input
            .call_method1("add_linear_exp_val", ("multi_pp_val", linear_map))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let mut measured_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
        let _ = measured_registers.insert("ro_pauli_product_0".to_string(), vec![vec![1.0]]);
        let _ = measured_registers.insert("ro_pauli_product_1".to_string(), vec![vec![0.0]]);
        let _ = measured_registers.insert("ro_pauli_product_2".to_string(), vec![vec![-0.5]]);

        let bit_register: HashMap<String, BitOutputRegister> = HashMap::new();
        let complex_register: HashMap<String, ComplexOutputRegister> = HashMap::new();
        let result = br
            .call_method1(
                "evaluate",
                (bit_register, measured_registers, complex_register),
            )
            .unwrap();
        let two_qubit_exp_val_py = f64::extract(result.get_item("single_pp_val").unwrap()).unwrap();
        let two_pp_exp_val_py = f64::extract(result.get_item("multi_pp_val").unwrap()).unwrap();
        assert_eq!(two_qubit_exp_val_py, 3.0);
        assert_eq!(two_pp_exp_val_py, 1.0);
    })
}

/// Test evaluate() function for CheatedPauliZProduct measurement with symbolic parameters
#[test]
fn test_evaluate_symbolic() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_0",))
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_1",))
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_2",))
            .unwrap();

        let symbolic_pystring =
            "sin(3.0 * pauli_product_0) + sin(-1.0 * pauli_product_1)".to_string();
        let _ = input
            .call_method1("add_symbolic_exp_val", ("single_pp_val", symbolic_pystring))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let mut measured_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
        let _ = measured_registers.insert("ro_pauli_product_0".to_string(), vec![vec![1.0]]);
        let _ = measured_registers.insert("ro_pauli_product_1".to_string(), vec![vec![-1.0]]);
        let _ = measured_registers.insert("ro_pauli_product_2".to_string(), vec![vec![-0.5]]);

        let bit_register: HashMap<String, BitOutputRegister> = HashMap::new();
        let complex_register: HashMap<String, ComplexOutputRegister> = HashMap::new();
        let result = br
            .call_method1(
                "evaluate",
                (bit_register, measured_registers, complex_register),
            )
            .unwrap();
        let single_pp_val_py = f64::extract(result.get_item("single_pp_val").unwrap()).unwrap();
        assert!((single_pp_val_py - (3.0_f64.sin() + 1.0_f64.sin())).abs() < f64::EPSILON);
    })
}

/// Test evaluate failure
#[test]
fn test_py03_evaluate_error0() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_0",))
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_1",))
            .unwrap();
        let _ = input
            .call_method1("add_pauliz_product", ("ro_pauli_product_2",))
            .unwrap();

        let symbolic_pystring =
            "sin(3.0 * pauli_product_0) + sin(-1.0 * pauli_product_3)".to_string();
        let _ = input
            .call_method1("add_symbolic_exp_val", ("single_pp_val", symbolic_pystring))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let input2: HashMap<String, FloatOutputRegister> =
            HashMap::<String, FloatOutputRegister>::new();
        let input3: HashMap<String, ComplexOutputRegister> =
            HashMap::<String, ComplexOutputRegister>::new();

        let result = br.call_method1("evaluate", (vec![0], input2.clone(), input3.clone()));
        assert!(result.is_err());

        let mut input1: HashMap<String, Vec<Vec<usize>>> = HashMap::new();
        input1.insert("ro".to_string(), vec![vec![0]]);
        let error = br.call_method1("evaluate", (input1, input2, input3));
        assert!(error.is_err());
    })
}

/// Test copy
#[test]
fn test_pyo3_copy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();
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
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();
        let br_wrapper = br.extract::<CheatedPauliZProductWrapper>().unwrap();

        let br_clone = br_wrapper.clone();
        assert_eq!(format!("{:?}", br_wrapper), format!("{:?}", br_clone));

        let debug_string = "RefCell { value: CheatedPauliZProductWrapper { internal: CheatedPauliZProduct { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }], input: CheatedPauliZProductInput { measured_exp_vals: {}, pauli_product_keys: {\"ro\": 0} } } } }";
        assert_eq!(format!("{:?}", br), debug_string);

        let debug_input = input;
        let debug_input_string = "RefCell { value: CheatedPauliZProductInputWrapper { internal: CheatedPauliZProductInput { measured_exp_vals: {}, pauli_product_keys: {\"ro\": 0} } } }";
        assert_eq!(format!("{:?}", input), debug_input_string);
        assert_eq!(
            CheatedPauliZProductInputWrapper::default().internal,
            CheatedPauliZProductInputWrapper::new().internal
        );

        let mut linear_map: HashMap<usize, f64> = HashMap::new();
        linear_map.insert(0, 3.0);
        let _ = debug_input
            .call_method1("add_linear_exp_val", ("single_pp_val", linear_map.clone()))
            .unwrap();
        linear_map.insert(0, 5.0);
        let error = debug_input.call_method1("add_linear_exp_val", ("single_pp_val", linear_map));
        assert!(error.is_err());

        let symbolic_pystring = "sin(3.0 * pauli_product_1)".to_string();
        let error =
            debug_input.call_method1("add_symbolic_exp_val", ("single_pp_val", symbolic_pystring));
        assert!(error.is_err());
    })
}

/// Test _internal_to_bincode function
#[test]
fn test_internal_to_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let mut roqoqo_bri = CheatedPauliZProductInput::new();
        roqoqo_bri.add_pauliz_product("ro".to_string());
        let circs: Vec<Circuit> = vec![Circuit::new()];
        let roqoqo_br = CheatedPauliZProduct {
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
        assert_eq!(serialised.0, "CheatedPauliZProduct");
        assert_eq!(serialised.1, comparison_serialised);
    })
}

/// Test to_json and from_json functions
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let deserialised = new_br
            .call_method1("from_json", (serialised,))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
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
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let mut map: HashMap<String, f64> = HashMap::<String, f64>::new();
        map.insert("theta".to_string(), 0.0);
        let br_sub = br
            .call_method1("substitute_parameters", (map,))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let br_wrapper = br.extract::<CheatedPauliZProductWrapper>().unwrap();
        let br_sub_wrapper = br_sub.extract::<CheatedPauliZProductWrapper>().unwrap();
        assert_ne!(format!("{:?}", br_wrapper), format!("{:?}", br_sub_wrapper));
    })
}

/// Test substitute_parameters returning an error
#[test]
fn test_substitute_parameters_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let map: HashMap<String, f64> = HashMap::<String, f64>::new();
        let br_sub = br.call_method1("substitute_parameters", (map,));
        assert!(br_sub.is_err());
    })
}

/// Test measurement_type()
#[test]
fn test_measurement_type() {
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let measurement_type = br.call_method0("measurement_type").unwrap();
        assert_eq!(measurement_type.to_string(), "CheatedPauliZProduct");
    })
}

/// Test input()
#[test]
fn test_return_input() {
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let input = input_type
            .call0()
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductWrapper>>()
            .unwrap();

        let input_returned = br
            .call_method0("input")
            .unwrap()
            .cast_as::<PyCell<CheatedPauliZProductInputWrapper>>()
            .unwrap();

        assert_eq!(format!("{:?}", input_returned), format!("{:?}", input));
    })
}
