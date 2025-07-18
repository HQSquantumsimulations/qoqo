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

use pyo3::prelude::*;
use qoqo::noise_models::*;
#[cfg(feature = "json_schema")]
use roqoqo::noise_models::{SingleQubitOverrotationDescription, SingleQubitOverrotationOnGate};
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;

/// Test copy
#[test]
fn test_pyo3_copy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let binding = br_type.call0().unwrap();
        let br: &Bound<SingleQubitOverrotationOnGateWrapper> = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        let br_copied = br
            .call_method0("__copy__")
            .unwrap()
            .extract::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        let br_wrapper = br
            .extract::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        assert_eq!(br_copied, br_wrapper);

        let br_copied = br
            .call_method1("__deepcopy__", ("",))
            .unwrap()
            .extract::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        let br_wrapper = br
            .extract::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        assert_eq!(br_copied, br_wrapper);
    })
}

#[test]
fn test_pyo3_copy_description() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationDescriptionWrapper>();
        let binding = br_type.call1(("RotateX", 0.0, 1.0)).unwrap();
        let br: &Bound<SingleQubitOverrotationDescriptionWrapper> = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        let br_copied = br
            .call_method0("__copy__")
            .unwrap()
            .extract::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        let br_wrapper = br
            .extract::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        assert_eq!(br_copied, br_wrapper);

        let br_copied = br
            .call_method1("__deepcopy__", ("",))
            .unwrap()
            .extract::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        let br_wrapper = br
            .extract::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        assert_eq!(br_copied, br_wrapper);
    })
}

/// Test debug
#[test]
fn test_debug_description() {
    let wrapper = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 0.0, 1.0);

    let compare =  "SingleQubitOverrotationDescriptionWrapper { internal: SingleQubitOverrotationDescription { gate: \"RotateX\", theta_mean: 0.0, theta_std: 1.0 } }";
    assert_eq!(format!("{:?}", wrapper), compare);
}
#[test]
fn test_debug() {
    Python::with_gil(|py| {
        // Overrotation Model Wrapper
        let wrapper_description_type = py.get_type::<SingleQubitOverrotationDescriptionWrapper>();
        let binding = wrapper_description_type
            .call1(("RotateX", 0.0, 1.0))
            .unwrap();
        let py_wrapper_description = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();

        let mut wrapper = SingleQubitOverrotationOnGateWrapper::new();
        wrapper = wrapper
            .set_single_qubit_overrotation("RotateZ", 0, py_wrapper_description)
            .unwrap();

        let compare = "SingleQubitOverrotationOnGateWrapper { internal: SingleQubitOverrotationOnGate { single_qubit_overrotation: {(\"RotateZ\", 0): SingleQubitOverrotationDescription { gate: \"RotateX\", theta_mean: 0.0, theta_std: 1.0 }}, two_qubit_overrotation: {} } }";
        assert_eq!(format!("{:?}", wrapper), compare);
    })
}

/// Test clone
#[test]
fn test_clone_description() {
    let wrapper = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 0.0, 1.0);

    assert_eq!(wrapper, wrapper.clone());
}

/// Test PartialEq
#[test]
fn test_partialeq_description() {
    let wrapper1 = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 0.0, 1.0);
    let wrapper2 = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 0.0, 1.0);
    let wrapper3 = SingleQubitOverrotationDescriptionWrapper::new("RotateZ", 0.0, 1.0);

    assert!(wrapper1 == wrapper2);
    assert_eq!(wrapper1, wrapper2);
    assert_ne!(wrapper1, wrapper3);
    assert_ne!(wrapper3, wrapper1);
}

/// Test to_json and from_json functions
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let binding = new_br.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        assert_eq!(
            format!("{:?}", br.borrow()),
            format!("{:?}", deserialised.borrow())
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

#[test]
fn test_to_from_json_description() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationDescriptionWrapper>();
        let binding = br_type.call1(("RotateX", 0.0, 1.0)).unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let binding = new_br.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        assert_eq!(
            format!("{:?}", br.borrow()),
            format!("{:?}", deserialised.borrow())
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
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        let new_br = br;
        let serialised = br.call_method0("to_bincode").unwrap();
        let binding = new_br.call_method1("from_bincode", (&serialised,)).unwrap();
        let deserialised = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();
        assert_eq!(
            format!("{:?}", br.borrow()),
            format!("{:?}", deserialised.borrow())
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

#[test]
fn test_to_from_bincode_description() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationDescriptionWrapper>();
        let binding = br_type.call1(("RotateX", 0.0, 1.0)).unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        let new_br = br;
        let serialised = br.call_method0("to_bincode").unwrap();
        let binding = new_br.call_method1("from_bincode", (&serialised,)).unwrap();
        let deserialised = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        assert_eq!(
            format!("{:?}", br.borrow()),
            format!("{:?}", deserialised.borrow())
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

#[test]
fn test_single_qubit_noise_term() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();

        let desc = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 1.0, 1.0);
        let br = br
            .call_method1(
                "set_single_qubit_overrotation",
                ("RotateX", 0, desc.clone()),
            )
            .unwrap();
        let description = br
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap()
            .call_method1("get_single_qubit_overrotation", ("RotateX", 0))
            .unwrap()
            .extract::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        assert_eq!(description, desc);
    })
}

#[test]
fn test_two_qubit_noise_term() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();

        let desc1 = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 1.0, 1.0);
        let desc2 = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 1.0, 1.0);

        let br = br
            .call_method1(
                "set_two_qubit_overrotation",
                ("CNOT", 0, 1, (desc1.clone(), desc2.clone())),
            )
            .unwrap();
        let operator = br
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap()
            .call_method1("get_two_qubit_overrotation", ("CNOT", 0, 1))
            .unwrap()
            .extract::<(
                SingleQubitOverrotationDescriptionWrapper,
                SingleQubitOverrotationDescriptionWrapper,
            )>()
            .unwrap();
        assert_eq!(operator, (desc1, desc2));
    })
}

/// Test json_schema function of SingleQubitOverrotationOnGate
#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();

        let schema: String =
            String::extract_bound(&br.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(SingleQubitOverrotationOnGate))
                .unwrap();
        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&br.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&br.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.11.0");
    });
}

#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema_description() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationDescriptionWrapper>();
        let binding = br_type.call1(("RotateX", 0.0, 1.0)).unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();

        let schema: String =
            String::extract_bound(&br.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema = serde_json::to_string_pretty(&schemars::schema_for!(
            SingleQubitOverrotationDescription
        ))
        .unwrap();
        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&br.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&br.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.11.0");
    });
}

#[test]
fn test_pyo3_richcmp() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding
            .downcast::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();

        let desc1 = SingleQubitOverrotationDescriptionWrapper::new("RotateX", 1.0, 1.0);
        let desc2 = SingleQubitOverrotationDescriptionWrapper::new("Rotatez", 1.0, 1.0);

        let operation_one = br
            .call_method1(
                "set_two_qubit_overrotation",
                ("CNOT", 0, 1, (desc1.clone(), desc1.clone())),
            )
            .unwrap();
        let operation_two = br
            .call_method1(
                "set_two_qubit_overrotation",
                ("CNOT", 0, 1, (desc1.clone(), desc2.clone())),
            )
            .unwrap();

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__eq__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__ne__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1("__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}

#[test]
fn test_pyo3_richcmp_description() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationDescriptionWrapper>();
        let binding = br_type.call1(("RotateX", 0.0, 1.0)).unwrap();
        let operation_one = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        let binding = br_type.call1(("RotateZ", 0.0, 1.0)).unwrap();
        let operation_two = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__eq__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__ne__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1("__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}

#[test]
fn test_pyo3_repr_description() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationDescriptionWrapper>();
        let binding = br_type.call1(("RotateX", 0.0, 1.0)).unwrap();
        let operation = binding
            .downcast::<SingleQubitOverrotationDescriptionWrapper>()
            .unwrap();
        let to_repr = operation.call_method0("__repr__").unwrap();
        let repr_op: String = String::extract_bound(&to_repr).unwrap();
        let format_repr = "SingleQubitOverrotationDescription { gate: \"RotateX\", theta_mean: 0.0, theta_std: 1.0 }";
        assert_eq!(repr_op, format_repr);
    })
}
