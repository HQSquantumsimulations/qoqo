// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use roqoqo::{noise_models::SingleQubitOverrotationOnGate, ROQOQO_VERSION};

/// Test copy
#[test]
fn test_pyo3_init() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
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
    })
}

/// Test debug
#[test]
fn test_pyo3_debug() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();
        let br_wrapper = br
            .extract::<SingleQubitOverrotationOnGateWrapper>()
            .unwrap();

        let br_clone = br_wrapper.clone();
        assert_eq!(format!("{:?}", br_wrapper), format!("{:?}", br_clone));

        let debug_string = format!("{:?}", br_clone);
        assert_eq!(format!("{:?}", br_wrapper), debug_string);
    })
}

/// Test to_json and from_json functions
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let deserialised = new_br
            .call_method1("from_json", (serialised,))
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
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

/// Test to_bincode and from_bincode functions
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();
        let new_br = br;
        let serialised = br.call_method0("to_bincode").unwrap();
        let deserialised = new_br
            .call_method1("from_bincode", (serialised,))
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();
        assert_eq!(format!("{:?}", br), format!("{:?}", deserialised));

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
fn test_singe_qubit_noise_term() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<SingleQubitOverrotationOnGateWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();

        let internal_desc =
            roqoqo::noise_models::SingleQubitOverrotationDescription::new("RotateX", 1.0, 1.0);
        let desc = SingleQubitOverrotationDescriptionWrapper {
            internal: internal_desc,
        };
        let br = br
            .call_method1(
                "set_single_qubit_overrotations",
                ("RotateX", 0, desc.clone()),
            )
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();
        let description = br
            .call_method1("get_single_qubit_overrotations", ("RotateX", 0))
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
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();

        let internal_desc =
            roqoqo::noise_models::SingleQubitOverrotationDescription::new("RotateX", 1.0, 1.0);
        let desc1 = SingleQubitOverrotationDescriptionWrapper {
            internal: internal_desc.clone(),
        };
        let desc2 = SingleQubitOverrotationDescriptionWrapper {
            internal: internal_desc.clone(),
        };

        let br = br
            .call_method1(
                "set_two_qubit_overrotations",
                ("CNOT", 0, 1, (desc1.clone(), desc2.clone())),
            )
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();
        let operator = br
            .call_method1("get_two_qubit_overrotations", ("CNOT", 0, 1))
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
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<SingleQubitOverrotationOnGateWrapper>>()
            .unwrap();

        let schema: String = String::extract(br.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(SingleQubitOverrotationOnGate))
                .unwrap();
        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract(br.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract(br.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.10.0");
    });
}
