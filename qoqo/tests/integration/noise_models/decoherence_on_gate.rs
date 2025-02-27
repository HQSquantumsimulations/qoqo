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
use roqoqo::{noise_models::DecoherenceOnGateModel, ROQOQO_VERSION};
use struqture::OperateOnDensityMatrix;
use struqture_py::spins;

/// Test copy
#[test]
fn test_pyo3_init() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();
        let br_copied = br
            .call_method0("__copy__")
            .unwrap()
            .extract::<DecoherenceOnGateModelWrapper>()
            .unwrap();
        let br_wrapper = br.extract::<DecoherenceOnGateModelWrapper>().unwrap();
        assert_eq!(br_copied, br_wrapper);
    })
}

/// Test debug
#[test]
fn test_pyo3_debug() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();
        let br_wrapper = br.extract::<DecoherenceOnGateModelWrapper>().unwrap();

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
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let binding = new_br.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();
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
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();
        let new_br = br;
        let serialised = br.call_method0("to_bincode").unwrap();
        let binding = new_br.call_method1("from_bincode", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();
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
fn test_singe_qubit_noise_term() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();

        let mut internal_plus_minus = struqture::spins::PlusMinusLindbladNoiseOperator::new();
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().plus(0),
                struqture::spins::PlusMinusProduct::new().plus(0),
            ),
            0.1.into(),
        );
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper {
            internal: internal_plus_minus,
        };
        let binding = br
            .call_method1(
                "set_single_qubit_gate_error",
                ("RotateX", 0, plus_minus_operator.clone()),
            )
            .unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();
        let operator = br
            .call_method1("get_single_qubit_gate_error", ("RotateX", 0))
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>()
            .unwrap();
        assert_eq!(operator, plus_minus_operator);
    })
}

#[test]
fn test_two_qubit_noise_term() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();

        let mut internal_plus_minus = struqture::spins::PlusMinusLindbladNoiseOperator::new();
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().plus(0),
                struqture::spins::PlusMinusProduct::new().plus(0),
            ),
            0.1.into(),
        );
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper {
            internal: internal_plus_minus,
        };
        let br = br
            .call_method1(
                "set_two_qubit_gate_error",
                ("CNOT", 0, 1, plus_minus_operator.clone()),
            )
            .unwrap();
        let operator = br
            .downcast::<DecoherenceOnGateModelWrapper>()
            .unwrap()
            .call_method1("get_two_qubit_gate_error", ("CNOT", 0, 1))
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>()
            .unwrap();
        assert_eq!(operator, plus_minus_operator);
    })
}

#[test]
fn test_three_qubit_noise_term() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();

        let mut internal_plus_minus = struqture::spins::PlusMinusLindbladNoiseOperator::new();
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().plus(0),
                struqture::spins::PlusMinusProduct::new().plus(0),
            ),
            0.1.into(),
        );
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper {
            internal: internal_plus_minus,
        };
        let br = br
            .call_method1(
                "set_three_qubit_gate_error",
                (
                    "ControlledControlledPauliZ",
                    0,
                    1,
                    2,
                    plus_minus_operator.clone(),
                ),
            )
            .unwrap();
        let operator = br
            .downcast::<DecoherenceOnGateModelWrapper>()
            .unwrap()
            .call_method1(
                "get_three_qubit_gate_error",
                ("ControlledControlledPauliZ", 0, 1, 2),
            )
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>()
            .unwrap();
        assert_eq!(operator, plus_minus_operator);
    })
}

#[test]
fn test_multi_qubit_noise_term() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();

        let mut internal_plus_minus = struqture::spins::PlusMinusLindbladNoiseOperator::new();
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().plus(0),
                struqture::spins::PlusMinusProduct::new().plus(0),
            ),
            0.1.into(),
        );
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper {
            internal: internal_plus_minus,
        };
        let br = br
            .call_method1(
                "set_multi_qubit_gate_error",
                ("MultiQubitMS", vec![0, 1, 2], plus_minus_operator.clone()),
            )
            .unwrap();
        let operator = br
            .downcast::<DecoherenceOnGateModelWrapper>()
            .unwrap()
            .call_method1(
                "get_multi_qubit_gate_error",
                ("MultiQubitMS", vec![0, 1, 2]),
            )
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>()
            .unwrap();
        assert_eq!(operator, plus_minus_operator);
    })
}

/// Test json_schema function of DecoherenceOnGateModel
#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let br_type = py.get_type::<DecoherenceOnGateModelWrapper>();
        let binding = br_type.call0().unwrap();
        let br = binding.downcast::<DecoherenceOnGateModelWrapper>().unwrap();

        let schema: String =
            String::extract_bound(&br.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema =
            serde_json::to_string_pretty(&schemars::schema_for!(DecoherenceOnGateModel)).unwrap();
        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&br.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&br.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.6.0");
    });
}
