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
use qoqo::measurements::ClassicalRegisterWrapper;
use qoqo::CircuitWrapper;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use roqoqo::{measurements::ClassicalRegister, Circuit};
use std::collections::HashMap;

#[test]
fn test_returning_circuits() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

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

/// Test copy
#[test]
fn test_pyo3_copy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, 0.0.into());
        circs.push(circ1);
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
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
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type.call1((Some(CircuitWrapper::new()), circs)).unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
        let br_wrapper = br.extract::<ClassicalRegisterWrapper>().unwrap();

        #[allow(clippy::redundant_clone)]
        let br_clone = br_wrapper.clone();
        assert_eq!(format!("{:?}", br_wrapper), format!("{:?}", br_clone));
        let debug_string = "ClassicalRegisterWrapper { internal: ClassicalRegister { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }] } }";
        assert_eq!(format!("{:?}", br.borrow()), debug_string);
    })
}

/// Test _internal_to_bincode function
#[test]
fn test_internal_to_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type.call1((Some(CircuitWrapper::new()), circs)).unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let circs: Vec<Circuit> = vec![Circuit::new()];
        let roqoqo_br = ClassicalRegister {
            constant_circuit: Some(Circuit::new()),
            circuits: circs,
        };
        let comparison_serialised = serialize(&roqoqo_br).unwrap();

        let serialised: (String, Vec<u8>) = br
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
    Python::with_gil(|py| {
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type.call1((Some(CircuitWrapper::new()), circs)).unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let binding = new_br.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
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

/// Test to_bincode and from_bincode functions
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type.call1((Some(CircuitWrapper::new()), circs)).unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_bincode").unwrap();
        let binding = new_br.call_method1("from_bincode", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
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

/// Test substitute_parameters
#[test]
fn test_substitute_parameters() {
    Python::with_gil(|py| {
        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let mut map: HashMap<String, f64> = HashMap::<String, f64>::new();
        map.insert("theta".to_string(), 0.0);
        let binding = br.call_method1("substitute_parameters", (map,)).unwrap();
        let br_sub = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let br_wrapper = br.extract::<ClassicalRegisterWrapper>().unwrap();
        let br_sub_wrapper = br_sub.extract::<ClassicalRegisterWrapper>().unwrap();
        assert_ne!(format!("{:?}", br_wrapper), format!("{:?}", br_sub_wrapper));
    })
}

/// Test substitute_parameters returning an error
#[test]
fn test_substitute_parameters_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let map: HashMap<String, f64> = HashMap::<String, f64>::new();
        let br_sub = br.call_method1("substitute_parameters", (map,));
        assert!(br_sub.is_err());
    })
}

/// Test measurement_type()
#[test]
fn test_measurement_type() {
    Python::with_gil(|py| {
        let mut circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let mut circ1 = CircuitWrapper::new();
        circ1.internal += roqoqo::operations::RotateX::new(0, "theta".into());
        circs.push(circ1);
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let measurement_type = br.call_method0("measurement_type").unwrap();
        assert_eq!(measurement_type.to_string(), "ClassicalRegister");
    })
}

#[test]
fn test_pyo3_format_repr() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let format_repr = "ClassicalRegister { constant_circuit: Some(Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }), circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }] }";
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type.call1((Some(CircuitWrapper::new()), circs)).unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
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
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type.call1((Some(CircuitWrapper::new()), circs)).unwrap();
        let br = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
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
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let br_one = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
        let arg: Option<CircuitWrapper> = None;
        let binding = br_type.call1((arg, circs)).unwrap();
        let br_two = binding.downcast::<ClassicalRegisterWrapper>().unwrap();
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
    let rust_schema =
        serde_json::to_string_pretty(&schemars::schema_for!(ClassicalRegister)).unwrap();
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];
        let br_type = py.get_type::<ClassicalRegisterWrapper>();
        #[allow(clippy::redundant_clone)]
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs.clone()))
            .unwrap();
        let br_one = binding.downcast::<ClassicalRegisterWrapper>().unwrap();

        let schema: String =
            String::extract_bound(&br_one.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&br_one.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&br_one.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.0.0");
    });
}
