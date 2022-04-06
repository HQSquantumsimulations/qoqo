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

use ndarray::{array, Array2};
use numpy::{pyarray, PyArray2};
use pyo3::prelude::*;
use qoqo::{AllToAllDeviceWrapper, GenericChainWrapper, GenericDeviceWrapper, GenericGridWrapper};
use roqoqo::devices::{AllToAllDevice, GenericChain, GenericDevice, GenericGrid};
// use test_case::test_case;

// helper functions to create device objects in pyo3
fn new_genericgrid(py: Python) -> &PyCell<GenericGridWrapper> {
    // fixed test parameters
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

fn new_alltoalldevice(py: Python) -> &PyCell<AllToAllDeviceWrapper> {
    let number_qubits: u32 = 10;
    let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
    let two_qubit_gates = ["CNOT".to_string()];
    let multi_qubit_gates = ["".to_string()];
    let arguments = (
        number_qubits,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );
    let device_type = py.get_type::<AllToAllDeviceWrapper>();
    device_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<AllToAllDeviceWrapper>>()
        .unwrap()
}

fn new_genericdevice(py: Python) -> &PyCell<GenericDeviceWrapper> {
    let number_qubits: u32 = 10;
    let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
    let two_qubit_gates = ["CNOT".to_string()];
    let multi_qubit_gates = ["".to_string()];
    let arguments = (
        number_qubits,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );
    let device_type = py.get_type::<GenericDeviceWrapper>();
    device_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<GenericDeviceWrapper>>()
        .unwrap()
}

fn new_genericchain(py: Python) -> &PyCell<GenericChainWrapper> {
    let number_qubits: u32 = 10;
    let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
    let two_qubit_gates = ["CNOT".to_string()];
    let multi_qubit_gates = ["".to_string()];
    let arguments = (
        number_qubits,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );
    let device_type = py.get_type::<GenericChainWrapper>();
    device_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<GenericChainWrapper>>()
        .unwrap()
}

// Test number_qubits() for GenericGrid
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

// Test number_qubits() for AllToAllDevice
#[test]
fn test_number_qubits_all() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = new_alltoalldevice(py);
        let number_qubits = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits, 10);
    })
}

// Test number_qubits() for GenericDevice
#[test]
fn test_number_qubits_generic() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = new_genericdevice(py);
        let number_qubits = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits, 10);
    })
}

// Test number_qubits() for GenericChain
#[test]
fn test_number_qubits_chain() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = new_genericchain(py);
        let number_qubits = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits, 10);
    })
}

// Test copy and deepcopy
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

// Test qubit_decoherence_rates() for GenericGrid
#[test]
fn test_decoherence_rates_genericgrid() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_genericgrid(py);
        // reference matrix for an initialized deviced or a non-existing qubit
        let matrix_zeros_py = Array2::<f64>::zeros((3, 3));
        let matrix_py = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test = matrix_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test, matrix_zeros_py);

        let matrix2_py = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test = matrix2_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test, matrix_zeros_py);

        let pyarray_testmatrix: Array2<f64> =
            array![[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let pyarray: &PyArray2<f64> =
            pyarray![py, [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let readonly = pyarray.readonly();
        device = device
            .call_method1("set_all_qubit_decoherence_rates", (readonly,))
            .unwrap()
            .cast_as::<PyCell<GenericGridWrapper>>()
            .unwrap();

        // proper matrix returned for the available qubit after setting decoherence rates
        let matrix_py2 = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);

        // matrix filled with zeros returned for a qubit not available in the given device
        let matrix2_py2 = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test2 = matrix2_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test2, matrix_zeros_py);
    })
}

// Test qubit_decoherence_rates() for GenericChain
#[test]
fn test_decoherence_rates_genericchain() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_genericchain(py);
        // reference matrix for an initialized deviced or a non-existing qubit
        let matrix_zeros_py = Array2::<f64>::zeros((3, 3));
        let matrix_py = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test = matrix_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test, matrix_zeros_py);

        let matrix2_py = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test = matrix2_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test, matrix_zeros_py);

        let pyarray_testmatrix: Array2<f64> =
            array![[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let pyarray: &PyArray2<f64> =
            pyarray![py, [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let readonly = pyarray.readonly();
        device = device
            .call_method1("set_all_qubit_decoherence_rates", (readonly,))
            .unwrap()
            .cast_as::<PyCell<GenericChainWrapper>>()
            .unwrap();

        // proper matrix returned for the available qubit after setting decoherence rates
        let matrix_py2 = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);

        // matrix filled with zeros returned for a qubit not available in the given device
        let matrix2_py2 = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test2 = matrix2_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test2, matrix_zeros_py);
    })
}

// Test qubit_decoherence_rates() for GenericDevice
#[test]
fn test_decoherence_rates_genericdevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_genericdevice(py);
        // reference matrix for an initialized deviced or a non-existing qubit
        let matrix_zeros_py = Array2::<f64>::zeros((3, 3));
        let matrix_py = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test = matrix_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test, matrix_zeros_py);

        let matrix2_py = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test = matrix2_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test, matrix_zeros_py);

        let pyarray_testmatrix: Array2<f64> =
            array![[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let pyarray: &PyArray2<f64> =
            pyarray![py, [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let readonly = pyarray.readonly();
        device = device
            .call_method1("set_all_qubit_decoherence_rates", (readonly,))
            .unwrap()
            .cast_as::<PyCell<GenericDeviceWrapper>>()
            .unwrap();

        // proper matrix returned for the available qubit after setting decoherence rates
        let matrix_py2 = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);

        // matrix filled with zeros returned for a qubit not available in the given device
        let matrix2_py2 = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test2 = matrix2_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test2, matrix_zeros_py);
    })
}

// Test qubit_decoherence_rates() for AllToAllDevice
#[test]
fn test_decoherence_rates_alltoalldevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_alltoalldevice(py);
        // reference matrix for an initialized deviced or a non-existing qubit
        let matrix_zeros_py = Array2::<f64>::zeros((3, 3));
        let matrix_py = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test = matrix_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test, matrix_zeros_py);

        let matrix2_py = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test = matrix2_py
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test, matrix_zeros_py);

        let pyarray_testmatrix: Array2<f64> =
            array![[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let pyarray: &PyArray2<f64> =
            pyarray![py, [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let readonly = pyarray.readonly();
        device = device
            .call_method1("set_all_qubit_decoherence_rates", (readonly,))
            .unwrap()
            .cast_as::<PyCell<AllToAllDeviceWrapper>>()
            .unwrap();

        // proper matrix returned for the available qubit after setting decoherence rates
        let matrix_py2 = device
            .call_method1("qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);

        // matrix filled with zeros returned for a qubit not available in the given device
        let matrix2_py2 = device
            .call_method1("qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test2 = matrix2_py2
            .cast_as::<PyArray2<f64>>()
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test2, matrix_zeros_py);
    })
}

// Test copy and deepcopy
#[test]
fn test_copy_deepcopy_genericchain() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericchain(py);

        let copy_dev = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_dev.extract::<GenericChainWrapper>().unwrap();
        let deepcopy_dev = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_dev.extract::<GenericChainWrapper>().unwrap();

        let device_wrapper = device.extract::<GenericChainWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode functions for GenericChain
#[test]
fn test_to_from_bincode_genericchain() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericchain(py);

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

        let serde_wrapper = deserialised.extract::<GenericChainWrapper>().unwrap();
        let device_wrapper = device.extract::<GenericChainWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test from_json and to_json for GenericChain
#[test]
fn test_to_from_json_genericchain() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericchain(py);

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

        let serde_wrapper = deserialised.extract::<GenericChainWrapper>().unwrap();
        let device_wrapper = device.extract::<GenericChainWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test copy and deepcopy
#[test]
fn test_copy_deepcopy_genericdevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericdevice(py);

        let copy_dev = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_dev.extract::<GenericDeviceWrapper>().unwrap();
        let deepcopy_dev = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_dev.extract::<GenericDeviceWrapper>().unwrap();

        let device_wrapper = device.extract::<GenericDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode functions for GenericDevice
#[test]
fn test_to_from_bincode_genericdevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericdevice(py);

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

        let serde_wrapper = deserialised.extract::<GenericDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<GenericDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test from_json and to_json for GenericDevice
#[test]
fn test_to_from_json_genericdevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_genericdevice(py);

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

        let serde_wrapper = deserialised.extract::<GenericDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<GenericDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test copy and deepcopy
#[test]
fn test_copy_deepcopy_alltoalldevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_alltoalldevice(py);

        let copy_dev = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_dev.extract::<AllToAllDeviceWrapper>().unwrap();
        let deepcopy_dev = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_dev.extract::<AllToAllDeviceWrapper>().unwrap();

        let device_wrapper = device.extract::<AllToAllDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode functions for AllToAllDevice
#[test]
fn test_to_from_bincode_alltoalldevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_alltoalldevice(py);

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

        let serde_wrapper = deserialised.extract::<AllToAllDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<AllToAllDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test from_json and to_json for AllToAllDevice
#[test]
fn test_to_from_json_alltoalldevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> () {
        let device = new_alltoalldevice(py);

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

        let serde_wrapper = deserialised.extract::<AllToAllDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<AllToAllDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

// Test gate_times for AllToAllDevice
#[test]
fn test_gatetimes_all() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_alltoalldevice(py);
        let gate_time = 0.5_f64;

        // Test single qubit gate times
        device = device
            .call_method1(
                "set_all_single_qubit_gate_times",
                ("RotateZ", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<AllToAllDeviceWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_rotatez = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 0_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 100_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_rotatez, Some(gate_time.clone()));
        assert_eq!(gate_time_none, None);

        // Test two qubit gate times
        device = device
            .call_method1("set_all_two_qubit_gate_times", ("CNOT", gate_time.clone()))
            .unwrap()
            .cast_as::<PyCell<AllToAllDeviceWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_cnot = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 1_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none2 = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 100_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_cnot, Some(gate_time.clone()));
        assert_eq!(gate_time_none2, None);

        // Test multi qubit gate times
        device = device
            .call_method1(
                "set_all_multi_qubit_gate_times",
                ("test", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<AllToAllDeviceWrapper>>()
            .unwrap();

        let gate_time_test = device
            .call_method1("multi_qubit_gate_time", ("test", Vec::<usize>::new()))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();
        assert_eq!(gate_time_test, None);
    })
}

// Test gate_times for GenericGrid
#[test]
fn test_gatetimes_genericgrid() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_genericgrid(py);
        let gate_time = 0.5_f64;

        // Test single qubit gate times
        device = device
            .call_method1(
                "set_all_single_qubit_gate_times",
                ("RotateZ", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<GenericGridWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_rotatez = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 0_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 100_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_rotatez, Some(gate_time.clone()));
        assert_eq!(gate_time_none, None);

        // Test two qubit gate times
        device = device
            .call_method1("set_all_two_qubit_gate_times", ("CNOT", gate_time.clone()))
            .unwrap()
            .cast_as::<PyCell<GenericGridWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_cnot = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 1_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none2 = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 2_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_cnot, Some(gate_time.clone()));
        assert_eq!(gate_time_none2, None);

        // Test multi qubit gate times
        device = device
            .call_method1(
                "set_all_multi_qubit_gate_times",
                ("test", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<GenericGridWrapper>>()
            .unwrap();

        let gate_time_test = device
            .call_method1("multi_qubit_gate_time", ("test", Vec::<usize>::new()))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();
        assert_eq!(gate_time_test, None);
    })
}

// Test gate_times for GenericDevice
#[test]
fn test_gatetimes_genericdevice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_genericdevice(py);
        let gate_time = 0.5_f64;

        // Test single qubit gate times
        device = device
            .call_method1(
                "set_all_single_qubit_gate_times",
                ("RotateZ", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<GenericDeviceWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_rotatez = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 0_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 100_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_rotatez, Some(gate_time.clone()));
        assert_eq!(gate_time_none, None);

        // Test two qubit gate times
        device = device
            .call_method1("set_all_two_qubit_gate_times", ("CNOT", gate_time.clone()))
            .unwrap()
            .cast_as::<PyCell<GenericDeviceWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_cnot = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 1_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none2 = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 100_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_cnot, Some(gate_time.clone()));
        assert_eq!(gate_time_none2, None);

        // Test multi qubit gate times
        device = device
            .call_method1(
                "set_all_multi_qubit_gate_times",
                ("test", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<GenericDeviceWrapper>>()
            .unwrap();

        let gate_time_test = device
            .call_method1("multi_qubit_gate_time", ("test", Vec::<usize>::new()))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();
        assert_eq!(gate_time_test, None);
    })
}

// Test gate_times for GenericChain
#[test]
fn test_gatetimes_genericchain() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut device = new_genericchain(py);
        let gate_time = 0.5_f64;

        // Test single qubit gate times
        device = device
            .call_method1(
                "set_all_single_qubit_gate_times",
                ("RotateZ", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<GenericChainWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_rotatez = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 0_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none = device
            .call_method1("single_qubit_gate_time", ("RotateZ", 100_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_rotatez, Some(gate_time.clone()));
        assert_eq!(gate_time_none, None);

        // Test two qubit gate times
        device = device
            .call_method1("set_all_two_qubit_gate_times", ("CNOT", gate_time.clone()))
            .unwrap()
            .cast_as::<PyCell<GenericChainWrapper>>()
            .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_cnot = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 1_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none2 = device
            .call_method1("two_qubit_gate_time", ("CNOT", 0_i64, 2_i64))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();

        assert_eq!(gate_time_cnot, Some(gate_time.clone()));
        assert_eq!(gate_time_none2, None);

        // Test multi qubit gate times
        device = device
            .call_method1(
                "set_all_multi_qubit_gate_times",
                ("test", gate_time.clone()),
            )
            .unwrap()
            .cast_as::<PyCell<GenericChainWrapper>>()
            .unwrap();

        let gate_time_test = device
            .call_method1("multi_qubit_gate_time", ("test", Vec::<usize>::new()))
            .unwrap()
            .extract::<Option<f64>>()
            .unwrap();
        assert_eq!(gate_time_test, None);
    })
}

// Test Clone, PartialEq and Debug for GenericDevice
#[test]
fn test_genericdevice_derive() {
    let number_qubits = 1usize;
    let single_qubit_gates = &[];
    let two_qubit_gates = &[];
    let multi_qubit_gates = &[];
    let device = GenericDevice::new(
        number_qubits,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );

    let wrapper = GenericDeviceWrapper {internal: device};

    // Test debug
    let debug =  "GenericDeviceWrapper { internal: GenericDevice { number_qubits: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} } }";
    assert_eq!(format!("{:?}", wrapper.clone()), debug);

    // Test Clone and PartialEq
    assert!(wrapper.clone() == wrapper);
}

// Test Clone, PartialEq and Debug for GenericChain
#[test]
fn test_genericchain_derive() {
    let number_qubits = 1usize;
    let single_qubit_gates = &[];
    let two_qubit_gates = &[];
    let multi_qubit_gates = &[];
    let device = GenericChain::new(
        number_qubits,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );

    let wrapper = GenericChainWrapper {internal: device};

    // Test debug
    let debug =  "GenericChainWrapper { internal: GenericChain { number_qubits: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} } }";
    assert_eq!(format!("{:?}", wrapper.clone()), debug);

    // Test Clone and PartialEq
    assert!(wrapper.clone() == wrapper);
}

// Test Clone, PartialEq and Debug for AllToAllDevice
#[test]
fn test_alltoalldevice_derive() {
    let number_qubits = 1usize;
    let single_qubit_gates = &[];
    let two_qubit_gates = &[];
    let multi_qubit_gates = &[];
    let device = AllToAllDevice::new(
        number_qubits,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );

    let wrapper = AllToAllDeviceWrapper {internal: device};

    // Test debug
    let debug =  "AllToAllDeviceWrapper { internal: AllToAllDevice { number_qubits: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} } }";
    assert_eq!(format!("{:?}", wrapper.clone()), debug);

    // Test Clone and PartialEq
    assert!(wrapper.clone() == wrapper);
}

// Test Clone, PartialEq and Debug for GenericGrid
#[test]
fn test_genericgrid_derive() {
    let number_rows = 1usize;
    let number_columns = 1usize;
    let single_qubit_gates = &[];
    let two_qubit_gates = &[];
    let multi_qubit_gates = &[];
    let device = GenericGrid::new(
        number_rows,
        number_columns,
        single_qubit_gates,
        two_qubit_gates,
        multi_qubit_gates,
    );

    let wrapper = GenericGridWrapper { internal: device };

    // Test debug
    let debug =  "GenericGridWrapper { internal: GenericGrid { number_rows: 1, number_columns: 1, single_qubit_gates: {}, two_qubit_gates: {}, multi_qubit_gates: {}, decoherence_rates: {0: [[0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0],\n [0.0, 0.0, 0.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2} } }";
    assert_eq!(format!("{:?}", wrapper.clone()), debug);

    // Test Clone and PartialEq
    assert!(wrapper.clone() == wrapper);
}

// Test two_qubit_edges() for AllToAllDevice
#[test]
fn test_alltoalldevice_edges() {
    Python::with_gil(|py| {
        let number_qubits: u32 = 3;
        let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
        let two_qubit_gates = ["CNOT".to_string()];
        let multi_qubit_gates = ["".to_string()];
        let arguments = (
            number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            multi_qubit_gates,
        );
        let device_type = py.get_type::<AllToAllDeviceWrapper>();
        let device = device_type
            .call1(arguments)
            .unwrap()
            .cast_as::<PyCell<AllToAllDeviceWrapper>>()
            .unwrap();
        let test_edges = vec![(0, 1), (0, 2), (1, 2)];
        let edges = device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();

        assert_eq!(test_edges.len(), edges.len());
        for edge in edges {
            assert!(test_edges.contains(&edge));
        }
    })
}

// Test two_qubit_edges() for GenericDevice
#[test]
fn test_genericdevice_edges() {
    Python::with_gil(|py| {
        let number_qubits: u32 = 3;
        let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
        let two_qubit_gates = ["CNOT".to_string()];
        let multi_qubit_gates = ["".to_string()];
        let arguments = (
            number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            multi_qubit_gates,
        );
        let device_type = py.get_type::<GenericDeviceWrapper>();
        let device = device_type
            .call1(arguments)
            .unwrap()
            .cast_as::<PyCell<GenericDeviceWrapper>>()
            .unwrap();
        let test_edges = vec![(0, 1), (0, 2), (1, 2)];
        let edges = device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();

        assert_eq!(test_edges.len(), edges.len());
        for edge in edges {
            assert!(test_edges.contains(&edge));
        }
    })
}

// Test two_qubit_edges() for GenericChain
#[test]
fn test_genericchain_edges() {
    Python::with_gil(|py| {
        let number_qubits: u32 = 3;
        let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
        let two_qubit_gates = ["CNOT".to_string()];
        let multi_qubit_gates = ["".to_string()];
        let arguments = (
            number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            multi_qubit_gates,
        );
        let device_type = py.get_type::<GenericChainWrapper>();
        let device = device_type
            .call1(arguments)
            .unwrap()
            .cast_as::<PyCell<GenericChainWrapper>>()
            .unwrap();
        let test_edges = vec![(0, 1), (1, 2)];
        let edges = device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();

        assert_eq!(test_edges.len(), edges.len());
        for edge in edges {
            assert!(test_edges.contains(&edge));
        }
    })
}

// Test two_qubit_edges() for GenericGrid
#[test]
fn test_genericgrid_edges() {
    Python::with_gil(|py| {
        let device = new_genericgrid(py);
        let test_edges = vec![(0, 1), (0, 4), (1, 2), (1, 5), (2, 3), (2, 6), (3, 7), (4, 5), (4, 8), (5, 6), (5, 9), (6, 7), (6, 10), (7, 11), (8, 9), (9, 10), (10, 11)];
        let edges = device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();

        assert_eq!(test_edges.len(), edges.len());
        for edge in edges {
            assert!(test_edges.contains(&edge));
        }
    })
}
