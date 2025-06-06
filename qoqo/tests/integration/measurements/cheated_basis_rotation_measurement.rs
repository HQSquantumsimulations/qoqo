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
use pyo3::prelude::*;
use pyo3::Python;
use qoqo::measurements::{CheatedPauliZProductInputWrapper, CheatedPauliZProductWrapper};
use qoqo::CircuitWrapper;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
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
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

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
        let two_qubit_exp_val_py =
            f64::extract_bound(&result.get_item("single_pp_val").unwrap()).unwrap();
        let two_pp_exp_val_py =
            f64::extract_bound(&result.get_item("multi_pp_val").unwrap()).unwrap();
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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
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
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

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
        let single_pp_val_py =
            f64::extract_bound(&result.get_item("single_pp_val").unwrap()).unwrap();
        assert!((single_pp_val_py - (3.0_f64.sin() + 1.0_f64.sin())).abs() < f64::EPSILON);
    })
}

/// Test evaluate failure
#[test]
fn test_py03_evaluate_error0() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
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
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
        let br_wrapper = br.extract::<CheatedPauliZProductWrapper>().unwrap();

        #[allow(clippy::redundant_clone)]
        let br_clone = br_wrapper.clone();
        assert_eq!(format!("{:?}", br_wrapper), format!("{:?}", br_clone));

        let debug_string = "CheatedPauliZProductWrapper { internal: CheatedPauliZProduct { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }], input: CheatedPauliZProductInput { measured_exp_vals: {}, pauli_product_keys: {\"ro\": 0} } } }";
        assert_eq!(format!("{:?}", br.borrow()), debug_string);

        let debug_input = input;
        let debug_input_string = "CheatedPauliZProductInputWrapper { internal: CheatedPauliZProductInput { measured_exp_vals: {}, pauli_product_keys: {\"ro\": 0} } }";
        assert_eq!(format!("{:?}", input.borrow()), debug_input_string);
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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let mut roqoqo_bri = CheatedPauliZProductInput::new();
        roqoqo_bri.add_pauliz_product("ro".to_string());
        let circs: Vec<Circuit> = vec![Circuit::new()];
        let roqoqo_br = CheatedPauliZProduct {
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
        assert_eq!(serialised.0, "CheatedPauliZProduct");
        assert_eq!(serialised.1, comparison_serialised);
    })
}

/// Test to_bincode and from_bincode functions
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let serialised = input.call_method0("to_bincode").unwrap();
        let new_input = input;
        let binding = new_input
            .call_method1("from_bincode", (&serialised,))
            .unwrap();
        let deserialised = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        assert_eq!(format!("{:?}", input), format!("{:?}", deserialised));

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let new_br = br;

        let serialised = br.call_method0("to_bincode").unwrap();
        let binding = new_br.call_method1("from_bincode", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
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
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let serialised = input.call_method0("to_json").unwrap();
        let new_input = input;
        let binding = new_input.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        assert_eq!(format!("{:?}", input), format!("{:?}", deserialised));

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let binding = new_br.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
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
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let mut map: HashMap<String, f64> = HashMap::<String, f64>::new();
        map.insert("theta".to_string(), 0.0);
        let binding = br.call_method1("substitute_parameters", (map,)).unwrap();
        let br_sub = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

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
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let measurement_type = br.call_method0("measurement_type").unwrap();
        assert_eq!(measurement_type.to_string(), "CheatedPauliZProduct");
    })
}

/// Test input()
#[test]
fn test_return_input() {
    Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

        let binding = br.call_method0("input").unwrap();
        let input_returned = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();

        assert_eq!(format!("{:?}", input_returned), format!("{:?}", input));
    })
}

#[test]
fn test_pyo3_format_repr() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let format_repr = "CheatedPauliZProduct { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }], input: CheatedPauliZProductInput { measured_exp_vals: {}, pauli_product_keys: {\"ro\": 0} } }";
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
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
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
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
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br_one = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
        let arg: Option<CircuitWrapper> = None;
        let binding = br_type.call1((arg, circs.clone(), input)).unwrap();
        let br_two = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();
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
        serde_json::to_string_pretty(&schemars::schema_for!(CheatedPauliZProductInput)).unwrap();
    let rust_schema =
        serde_json::to_string_pretty(&schemars::schema_for!(CheatedPauliZProduct)).unwrap();
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let input_type = py.get_type::<CheatedPauliZProductInputWrapper>();
        let binding = input_type.call0().unwrap();
        let input = binding
            .downcast::<CheatedPauliZProductInputWrapper>()
            .unwrap();
        let _ = input.call_method1("add_pauliz_product", ("ro",)).unwrap();

        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<CheatedPauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone(), input))
            .unwrap();
        let br_one = binding.downcast::<CheatedPauliZProductWrapper>().unwrap();

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
