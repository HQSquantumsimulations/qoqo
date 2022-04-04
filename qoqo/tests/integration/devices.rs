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

use pyo3::prelude::*;
use qoqo::GenericGridWrapper;
// use roqoqo::devices::GenericGrid;
// use test_case::test_case;
use ndarray::Array2;
use numpy::{PyArray2, ToPyArray};

// helper functions
fn new_genericgrid(py: Python) -> &PyCell<GenericGridWrapper> {
    // test parameters
    let number_rows: u32 = 3;
    let number_columns: u32 = 4;
    let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
    let two_qubit_gates = ["CNOT".to_string()];
    let multi_qubit_gates = ["".to_string()];
    let arguments = (
        number_rows,
        number_columns,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );
    let device_type = py.get_type::<GenericGridWrapper>();
    device_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<GenericGridWrapper>>()
        .unwrap()
}

// Test number_qubits()
#[test]
fn test_number_qubits() {
    // test parameters
    let number_rows: usize = 3;
    let number_columns: usize = 4;
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = new_genericgrid(py);

        let number_rows_get = device
            .call_method0("number_rows")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_rows_get, number_rows);

        let number_columns_get = device
            .call_method0("number_columns")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_columns_get, number_columns);

        let number_qubits = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits, number_rows * number_columns);
    })
}

/// Test copy and deepcopy
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericgrid(py);

        let copy_dev = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_dev.extract::<GenericGridWrapper>().unwrap();
        let deepcopy_dev = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_dev.extract::<GenericGridWrapper>().unwrap();

        let device_wrapper = device.extract::<GenericGridWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode functions for GenericGrid
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericgrid(py);

        let serialised = device.call_method0("to_bincode").unwrap();
        let new = device.clone();
        let deserialised = new.call_method1("from_bincode", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = new.call_method1("from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<GenericGridWrapper>().unwrap();
        let device_wrapper = device.extract::<GenericGridWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test from_json and to_json for GenericGrid
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericgrid(py);

        let serialised = device.call_method0("to_json").unwrap();
        let new = device.clone();
        let deserialised = new.call_method1("from_json", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = new.call_method1("from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<GenericGridWrapper>().unwrap();
        let device_wrapper = device.extract::<GenericGridWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test qubit_decoherence_rates()
#[test]
fn test_decoherence_rates() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = new_genericgrid(py);
        let matrix_py = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test = matrix_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        // reference matrix for an initialized deviced or a non-existing qubit
        let matrix_zeros_py = Array2::<f64>::zeros((3, 3));
        assert_eq!(matrix_test, matrix_zeros_py);
    })
}
