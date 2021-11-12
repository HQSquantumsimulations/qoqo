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
use qoqo::measurements::ClassicalRegisterWrapper;
use qoqo::CircuitWrapper;
use roqoqo::{measurements::ClassicalRegister, Circuit};
use std::collections::HashMap;

#[test]
fn test_returning_circuits() {
    pyo3::prepare_freethreaded_python();

    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br_type = py.get_type::<ClassicalRegisterWrapper>();
    let br = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone()))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
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
}

/// Test copy
#[test]
fn test_pyo3_copy() {
    pyo3::prepare_freethreaded_python();

    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
    circs.push(circ1);
    let br_type = py.get_type::<ClassicalRegisterWrapper>();
    let br = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone()))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
        .unwrap();
    let br_clone = br.clone();

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
}

/// Test debug and clone
#[test]
fn test_pyo3_debug() {
    pyo3::prepare_freethreaded_python();

    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());

    let br_type = py.get_type::<ClassicalRegisterWrapper>();
    let br = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone()))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
        .unwrap();
    let br_wrapper = br.extract::<ClassicalRegisterWrapper>().unwrap();

    let br_clone = br_wrapper.clone();
    assert_eq!(format!("{:?}", br_wrapper), format!("{:?}", br_clone));

    let debug_string = "RefCell { value: ClassicalRegisterWrapper { internal: ClassicalRegister { constant_circuit: Some(Circuit { definitions: [], operations: [] }), circuits: [Circuit { definitions: [], operations: [] }] } } }";
    assert_eq!(format!("{:?}", br), debug_string);
}

/// Test _internal_to_bincode function
#[test]
fn test_internal_to_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let mut circs: Vec<CircuitWrapper> = Vec::new();
        circs.push(CircuitWrapper::new());

        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let br = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap()
            .cast_as::<PyCell<ClassicalRegisterWrapper>>()
            .unwrap();

        let mut circs: Vec<Circuit> = Vec::new();
        circs.push(Circuit::new());
        let roqoqo_br = ClassicalRegister {
            constant_circuit: Some(Circuit::new()),
            circuits: circs.clone(),
        };
        let comparison_serialised = serialize(&roqoqo_br).unwrap();

        let serialised: (&str, Vec<u8>) = br
            .call_method0("_internal_to_bincode")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(serialised.0, "ClassicalRegister");
        assert_eq!(serialised.1, comparison_serialised);
    })
}

/// Test to_json and from_json functions
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();

    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());

    let br_type = py.get_type::<ClassicalRegisterWrapper>();
    let br = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone()))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
        .unwrap();

    let new_br = br.clone();
    let serialised = br.call_method0("to_json").unwrap();
    let deserialised = new_br
        .call_method1("from_json", (serialised,))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
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
}

/// Test substitute_parameters
#[test]
fn test_substitute_parameters() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
    circs.push(circ1);
    let br_type = py.get_type::<ClassicalRegisterWrapper>();
    let br = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone()))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
        .unwrap();

    let mut map: HashMap<String, f64> = HashMap::<String, f64>::new();
    map.insert("theta".to_string(), 0.0);
    let br_sub = br
        .call_method1("substitute_parameters", (map,))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
        .unwrap();

    let br_wrapper = br.extract::<ClassicalRegisterWrapper>().unwrap();
    let br_sub_wrapper = br_sub.extract::<ClassicalRegisterWrapper>().unwrap();
    assert_ne!(format!("{:?}", br_wrapper), format!("{:?}", br_sub_wrapper));
}

/// Test substitute_parameters returning an error
#[test]
fn test_substitute_parameters_error() {
    pyo3::prepare_freethreaded_python();

    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut circs: Vec<CircuitWrapper> = Vec::new();
    circs.push(CircuitWrapper::new());
    let mut circ1 = CircuitWrapper::new();
    circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
    circs.push(circ1);
    let br_type = py.get_type::<ClassicalRegisterWrapper>();
    let br = br_type
        .call1((Some(CircuitWrapper::new()), circs.clone()))
        .unwrap()
        .cast_as::<PyCell<ClassicalRegisterWrapper>>()
        .unwrap();

    let map: HashMap<String, f64> = HashMap::<String, f64>::new();
    let br_sub = br.call_method1("substitute_parameters", (map,));
    assert!(br_sub.is_err());
}
