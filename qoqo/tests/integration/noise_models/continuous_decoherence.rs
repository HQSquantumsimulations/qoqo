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
use struqture::OperateOnDensityMatrix;
use struqture_py::spins;

/// Test copy
#[test]
fn test_pyo3_init() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper::new();
        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call1((plus_minus_operator.clone(),))
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let comparison = br
            .call_method0("get_noise_operator")
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>();
        assert_eq!(plus_minus_operator, comparison.unwrap());
    })
}

/// Test copy
#[test]
fn test_add_damping() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
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

        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let br = br
            .call_method1("add_damping_rate", ([0], 0.1))
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let comparison = br
            .call_method0("get_noise_operator")
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>();
        assert_eq!(plus_minus_operator, comparison.unwrap());
    })
}

#[test]
fn test_add_dephasing() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut internal_plus_minus = struqture::spins::PlusMinusLindbladNoiseOperator::new();
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().z(0),
                struqture::spins::PlusMinusProduct::new().z(0),
            ),
            0.1.into(),
        );
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper {
            internal: internal_plus_minus,
        };

        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let br = br
            .call_method1("add_dephasing_rate", ([0], 0.1))
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let comparison = br
            .call_method0("get_noise_operator")
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>();
        assert_eq!(plus_minus_operator, comparison.unwrap());
    })
}

#[test]
fn test_add_depolarising() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut internal_plus_minus = struqture::spins::PlusMinusLindbladNoiseOperator::new();
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().z(0),
                struqture::spins::PlusMinusProduct::new().z(0),
            ),
            0.05.into(),
        );
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().plus(0),
                struqture::spins::PlusMinusProduct::new().plus(0),
            ),
            0.1.into(),
        );
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().minus(0),
                struqture::spins::PlusMinusProduct::new().minus(0),
            ),
            0.1.into(),
        );
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper {
            internal: internal_plus_minus,
        };

        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let br = br
            .call_method1("add_depolarising_rate", ([0], 0.1))
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let comparison = br
            .call_method0("get_noise_operator")
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>();
        assert_eq!(plus_minus_operator, comparison.unwrap());
    })
}

#[test]
fn test_add_excitation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut internal_plus_minus = struqture::spins::PlusMinusLindbladNoiseOperator::new();
        let _ = internal_plus_minus.add_operator_product(
            (
                struqture::spins::PlusMinusProduct::new().minus(0),
                struqture::spins::PlusMinusProduct::new().minus(0),
            ),
            0.1.into(),
        );
        let plus_minus_operator = spins::PlusMinusLindbladNoiseOperatorWrapper {
            internal: internal_plus_minus,
        };

        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let br = br
            .call_method1("add_excitation_rate", ([0], 0.1))
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let comparison = br
            .call_method0("get_noise_operator")
            .unwrap()
            .extract::<spins::PlusMinusLindbladNoiseOperatorWrapper>();
        assert_eq!(plus_minus_operator, comparison.unwrap());
    })
}

/// Test debug and clone
#[test]
fn test_pyo3_debug() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let br_wrapper = br.extract::<ContinuousDecoherenceModelWrapper>().unwrap();

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
        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let deserialised = new_br
            .call_method1("from_json", (serialised,))
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
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
        let br_type = py.get_type::<ContinuousDecoherenceModelWrapper>();
        let br = br_type
            .call0()
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
            .unwrap();
        let new_br = br;
        let serialised = br.call_method0("to_bincode").unwrap();
        let deserialised = new_br
            .call_method1("from_bincode", (serialised,))
            .unwrap()
            .downcast::<PyCell<ContinuousDecoherenceModelWrapper>>()
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
