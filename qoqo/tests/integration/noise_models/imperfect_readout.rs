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

/// Test copy
#[test]
fn test_pyo3_init() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<ImperfectReadoutModelWrapper>();
        let _br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
            .unwrap();
    })
}

#[test]
fn test_new_with_uniform_error_init() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<ImperfectReadoutModelWrapper>();
        let br = br_type
            .call_method1("new_with_uniform_error", (2, 0.2, 0.1))
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
            .unwrap();
        let br = br
            .call_method1("set_error_probabilites", (0, 0.3, 0.4))
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
            .unwrap();
        let zero_as_1_qubit_0: f64 = br
            .call_method1("prob_detect_0_as_1", (0,))
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(zero_as_1_qubit_0, 0.3);
        let zero_as_1_qubit_1: f64 = br
            .call_method1("prob_detect_0_as_1", (1,))
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(zero_as_1_qubit_1, 0.2);
        let one_as_0_qubit_0: f64 = br
            .call_method1("prob_detect_1_as_0", (0,))
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(one_as_0_qubit_0, 0.4);
    })
}

/// Test debug and clone
#[test]
fn test_pyo3_debug() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<ImperfectReadoutModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
            .unwrap();
        let br_wrapper = br.extract::<ImperfectReadoutModelWrapper>().unwrap();

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
        let br_type = py.get_type::<ImperfectReadoutModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
            .unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let deserialised = new_br
            .call_method1("from_json", (serialised,))
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
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
        let br_type = py.get_type::<ImperfectReadoutModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
            .unwrap();
        let new_br = br;
        let serialised = br.call_method0("to_bincode").unwrap();
        let deserialised = new_br
            .call_method1("from_bincode", (serialised,))
            .unwrap()
            .downcast::<PyCell<ImperfectReadoutModelWrapper>>()
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
