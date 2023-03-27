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

use ndarray::{array, Array2};
use numpy::{pyarray, PyArray2};
use pyo3::prelude::*;
use qoqo::devices::{AllToAllDeviceWrapper, GenericDeviceWrapper, SquareLatticeDeviceWrapper};
use roqoqo::devices::{AllToAllDevice, GenericDevice, SquareLatticeDevice};
use test_case::test_case;

fn new_alltoalldevice() -> Py<PyAny> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> Py<PyAny> {
        let number_qubits = 4;
        let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
        let two_qubit_gates = ["CNOT".to_string()];
        // let three_qubit_gates = ["ControlledControlledPauliZ".to_string()];
        let arguments: (usize, [String; 2], [String; 1], f64) = (
            number_qubits,
            single_qubit_gates,
            two_qubit_gates,
            // three_qubit_gates,
            1.0,
        );
        let device_type = py.get_type::<AllToAllDeviceWrapper>();
        device_type.call1(arguments).unwrap().into()
    })
}

fn new_genericdevice() -> Py<PyAny> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> Py<PyAny> {
        let number_qubits: u32 = 4;
        let arguments = (number_qubits,);
        let device_type = py.get_type::<GenericDeviceWrapper>();
        device_type.call1(arguments).unwrap().into()
    })
}

fn new_genericlattice() -> Py<PyAny> {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| -> Py<PyAny> {
        let number_rows: usize = 2;
        let number_columns: usize = 2;
        let single_qubit_gates = ["RotateX".to_string(), "RotateZ".to_string()];
        let two_qubit_gates = ["CNOT".to_string()];
        // let three_qubit_gates = ["ControlledControlledPauliZ".to_string()];
        let arguments: (usize, usize, [String; 2], [String; 1], f64) = (
            number_rows,
            number_columns,
            single_qubit_gates,
            two_qubit_gates,
            // three_qubit_gates,
            1.0,
        );
        let device_type = py.get_type::<SquareLatticeDeviceWrapper>();
        device_type.call1(arguments).unwrap().into()
        // .downcast::<PyCell<PyAny>>()
        // .unwrap()
    })
}
#[test]
fn test_number_rows() {
    // test parameters
    let number_rows: usize = 2;
    let number_columns: usize = 2;
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = new_genericlattice();

        let number_rows_get = device
            .call_method0(py, "number_rows")
            .unwrap()
            .extract::<usize>(py)
            .unwrap();
        assert_eq!(number_rows_get, number_rows);

        let number_columns_get = device
            .call_method0(py, "number_columns")
            .unwrap()
            .extract::<usize>(py)
            .unwrap();
        assert_eq!(number_columns_get, number_columns);
    })
}

// Test number_qubits() for AllToAllDevice
#[test_case(new_alltoalldevice(); "all_to_all")]
#[test_case(new_genericdevice(); "generic")]
#[test_case(new_genericlattice(); "lattice")]
fn test_number_qubits(device: Py<PyAny>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let number_qubits = device
            .call_method0(py, "number_qubits")
            .unwrap()
            .extract::<usize>(py)
            .unwrap();
        assert_eq!(number_qubits, 4);
    })
}

// Test from_json and to_json for GenericGrid
#[test_case(new_alltoalldevice(); "all_to_all")]
#[test_case(new_genericdevice(); "generic")]
#[test_case(new_genericlattice(); "lattice")]
fn test_to_from_json(device: Py<PyAny>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let serialised = device.call_method0(py, "to_json").unwrap();
        let new = device.clone();
        let deserialised = new
            .call_method1(py, "from_json", (serialised.clone(),))
            .unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = new.call_method1(py, "from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0(py, "from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0(py, "to_json");
        assert!(serialised_error.is_err());
        let comparison: bool = deserialised
            .call_method1(py, "__eq__", (device,))
            .unwrap()
            .extract(py)
            .unwrap();
        assert!(comparison);
    });
}

#[test_case(new_alltoalldevice(); "all_to_all")]
#[test_case(new_genericdevice(); "generic")]
#[test_case(new_genericlattice(); "lattice")]
fn test_to_from_bincode(device: Py<PyAny>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let serialised = device.call_method0(py, "to_bincode").unwrap();
        let new = device.clone();
        let deserialised = new
            .call_method1(py, "from_bincode", (serialised.clone(),))
            .unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = new.call_method1(py, "from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0(py, "from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0(py, "to_bincode");
        assert!(serialised_error.is_err());
        let comparison: bool = deserialised
            .call_method1(py, "__eq__", (device,))
            .unwrap()
            .extract(py)
            .unwrap();
        assert!(comparison);
    });
}

// Test qubit_decoherence_rates() for GenericGrid
#[test_case(new_alltoalldevice(); "all_to_all")]
#[test_case(new_genericlattice(); "lattice")]
fn test_decoherence_rates_all(device: Py<PyAny>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // reference matrix for an initialized deviced or a non-existing qubit

        // test that invalid matrix format is not accepted
        let pyarray_invalid: &PyArray2<f64> = pyarray![py, [1.0], [2.0], [3.0]];
        // let readonly_invalid = pyarray_invalid.readonly();
        let error = device.call_method1(py, "set_all_qubit_decoherence_rates", (pyarray_invalid,));
        assert!(error.is_err());

        let pyarray_testmatrix: Array2<f64> =
            array![[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let pyarray: &PyArray2<f64> =
            pyarray![py, [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        // let readonly = pyarray.readonly();
        let device = device
            .call_method1(py, "set_all_qubit_decoherence_rates", (pyarray,))
            .unwrap();
        // .downcast::<PyCell<SquareLatticeDeviceWrapper>>(py)
        // .unwrap();

        // proper matrix returned for the available qubit after setting decoherence rates
        let matrix_py2 = device
            .call_method1(py, "qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .downcast::<PyArray2<f64>>(py)
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);

        let pyarray_testmatrix: Array2<f64> = array![
            [1.0 + 10. + 100. / 2.0, 0.0, 0.0],
            [0.0, 2.0 + 100.0 / 2.0, 0.0],
            [0.0, 0.0, 3.0 + 100.0 / 4.0 + 1000.]
        ];
        let device = device.call_method1(py, "add_damping_all", (10.,)).unwrap();
        let device = device
            .call_method1(py, "add_depolarising_all", (100.,))
            .unwrap();
        let device = device
            .call_method1(py, "add_dephasing_all", (1000.,))
            .unwrap();

        let matrix_py2 = device
            .call_method1(py, "qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .downcast::<PyArray2<f64>>(py)
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);
    })
}

#[test_case(new_alltoalldevice(); "all_to_all")]
#[test_case(new_genericdevice(); "generic")]
#[test_case(new_genericlattice(); "lattice")]
fn test_decoherence_rates(device: Py<PyAny>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // reference matrix for an initialized deviced or a non-existing qubit
        let matrix_zeros_py = Array2::<f64>::zeros((3, 3));
        let matrix_py = device
            .call_method1(py, "qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test = matrix_py
            .downcast::<PyArray2<f64>>(py)
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test, matrix_zeros_py);

        let matrix2_py = device
            .call_method1(py, "qubit_decoherence_rates", (100_i64,))
            .unwrap();
        let matrix2_test = matrix2_py
            .downcast::<PyArray2<f64>>(py)
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix2_test, matrix_zeros_py);

        // test that invalid matrix format is not accepted
        let pyarray_invalid: &PyArray2<f64> = pyarray![py, [1.0], [2.0], [3.0]];
        // let readonly_invalid = pyarray_invalid.readonly();
        let error = device.call_method1(py, "set_qubit_decoherence_rates", (0, pyarray_invalid));
        assert!(error.is_err());
        let error = device.call_method1(py, "add_damping", (20, 1));
        assert!(error.is_err());
        let error = device.call_method1(py, "add_dephasing", (20, 1));
        assert!(error.is_err());
        let error = device.call_method1(py, "add_depolarising", (20, 1));
        assert!(error.is_err());

        let pyarray_testmatrix: Array2<f64> =
            array![[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let pyarray: &PyArray2<f64> =
            pyarray![py, [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        // let readonly = pyarray.readonly();
        device
            .call_method1(py, "set_qubit_decoherence_rates", (0, pyarray))
            .unwrap();
        // .downcast::<PyCell<SquareLatticeDeviceWrapper>>(py)
        // .unwrap();

        // proper matrix returned for the available qubit after setting decoherence rates
        let matrix_py2 = device
            .call_method1(py, "qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .downcast::<PyArray2<f64>>(py)
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);

        // testing add_damping, add_dephasing, add_depolarising

        let pyarray_testmatrix: Array2<f64> = array![
            [1.0 + 10. + 100. / 2.0, 0.0, 0.0],
            [0.0, 2.0 + 100.0 / 2.0, 0.0],
            [0.0, 0.0, 3.0 + 100.0 / 4.0 + 1000.]
        ];
        device.call_method1(py, "add_damping", (0, 10.)).unwrap();
        device
            .call_method1(py, "add_depolarising", (0, 100.))
            .unwrap();
        device
            .call_method1(py, "add_dephasing", (0, 1000.))
            .unwrap();

        let matrix_py2 = device
            .call_method1(py, "qubit_decoherence_rates", (0_i64,))
            .unwrap();
        let matrix_test2 = matrix_py2
            .downcast::<PyArray2<f64>>(py)
            .unwrap()
            .to_owned_array();
        assert_eq!(matrix_test2, pyarray_testmatrix);
    })
}

// Test gate_times for AllToAllDevice
#[test_case(new_alltoalldevice(); "all_to_all")]
#[test_case(new_genericdevice(); "generic")]
#[test_case(new_genericlattice(); "lattice")]
fn test_gatetimes(device: Py<PyAny>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let gate_time = 0.5_f64;

        // Test single qubit gate times
        device
            .call_method1(py, "set_single_qubit_gate_time", ("RotateZ", 0, gate_time))
            .unwrap();
        // .downcast::<PyCell<AllToAllDeviceWrapper>>(py)
        // .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_rotatez = device
            .call_method1(py, "single_qubit_gate_time", ("RotateZ", 0_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none = device
            .call_method1(py, "single_qubit_gate_time", ("RotateZ", 100_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        assert_eq!(gate_time_rotatez, Some(gate_time));
        assert_eq!(gate_time_none, None);

        // Test two qubit gate times
        device
            .call_method1(py, "set_two_qubit_gate_time", ("CNOT", 0, 1, gate_time))
            .unwrap();
        // .downcast::<PyCell<AllToAllDeviceWrapper>>(py)
        // .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_cnot = device
            .call_method1(py, "two_qubit_gate_time", ("CNOT", 0_i64, 1_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none2 = device
            .call_method1(py, "two_qubit_gate_time", ("CNOT", 0_i64, 100_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        assert_eq!(gate_time_cnot, Some(gate_time));
        assert_eq!(gate_time_none2, None);

        // // Test three qubit gate times
        // device
        //     .call_method1(
        //         py,
        //         "set_three_qubit_gate_time",
        //         ("ControlledControlledPauliZ", 0, 1, 2, gate_time),
        //     )
        //     .unwrap();

        // // get the gate time for CCZs on qubit 0 1 2
        // let gate_time_ccz = device
        //     .call_method1(
        //         py,
        //         "three_qubit_gate_time",
        //         ("ControlledControlledPauliZ", 0_i64, 1_i64, 2_i64),
        //     )
        //     .unwrap()
        //     .extract::<Option<f64>>(py)
        //     .unwrap();

        // // get the gate time for CCZ for a qubit not which is not in the device
        // let gate_time_none3 = device
        //     .call_method1(
        //         py,
        //         "three_qubit_gate_time",
        //         ("ControlledControlledPauliZ", 0_i64, 4_i64, 100_i64),
        //     )
        //     .unwrap()
        //     .extract::<Option<f64>>(py)
        //     .unwrap();

        // assert_eq!(gate_time_ccz, Some(gate_time));
        // assert_eq!(gate_time_none3, None);

        // Test multi qubit gate times
        device
            .call_method1(
                py,
                "set_multi_qubit_gate_time",
                ("MultiQubitMS", vec![0, 1, 2], gate_time),
            )
            .unwrap();
        // .downcast::<PyCell<AllToAllDeviceWrapper>>(py)
        // .unwrap();

        let gate_time_test = device
            .call_method1(py, "multi_qubit_gate_time", ("MultiQubitMS", vec![0, 1, 2]))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();
        assert_eq!(gate_time_test, None);
    })
}

// Test gate_times for AllToAllDevice
#[test_case(new_alltoalldevice(); "all_to_all")]
#[test_case(new_genericlattice(); "lattice")]
fn test_gatetimes_all(device: Py<PyAny>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let gate_time = 0.5_f64;

        // Test single qubit gate times
        let device = device
            .call_method1(
                py,
                "set_all_single_qubit_gate_times",
                ("RotateZ", gate_time),
            )
            .unwrap();
        // .downcast::<PyCell<AllToAllDeviceWrapper>>(py)
        // .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_rotatez = device
            .call_method1(py, "single_qubit_gate_time", ("RotateZ", 0_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none = device
            .call_method1(py, "single_qubit_gate_time", ("RotateZ", 100_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        assert_eq!(gate_time_rotatez, Some(gate_time));
        assert_eq!(gate_time_none, None);

        // Test two qubit gate times
        let device = device
            .call_method1(py, "set_all_two_qubit_gate_times", ("CNOT", gate_time))
            .unwrap();
        // .downcast::<PyCell<AllToAllDeviceWrapper>>(py)
        // .unwrap();

        // get the gate time for RotateZ on qubit 0
        let gate_time_cnot = device
            .call_method1(py, "two_qubit_gate_time", ("CNOT", 0_i64, 1_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        // get the gate time for RotateZ for a qubit not which is not in the device
        let gate_time_none2 = device
            .call_method1(py, "two_qubit_gate_time", ("CNOT", 0_i64, 100_i64))
            .unwrap()
            .extract::<Option<f64>>(py)
            .unwrap();

        assert_eq!(gate_time_cnot, Some(gate_time));
        assert_eq!(gate_time_none2, None);
    })
}

#[test]
fn test_derive_generic_device() {
    let device = GenericDevice::default();

    let wrapper = GenericDeviceWrapper { internal: device };

    // Test Clone and PartialEq
    assert!(wrapper == wrapper);
}

#[test]
fn test_derive_all_to_all() {
    let device = AllToAllDevice::default();

    let wrapper = AllToAllDeviceWrapper { internal: device };

    // Test Clone and PartialEq
    assert!(wrapper == wrapper);
}

#[test]
fn test_derive_square_lattice() {
    let device = SquareLatticeDevice::default();

    let wrapper = SquareLatticeDeviceWrapper { internal: device };

    // Test Clone and PartialEq
    assert!(wrapper == wrapper);
}

#[test_case(new_alltoalldevice(), vec![(0,1), (0,2), (0,3), (1,2), (1,3), (2,3)]; "all_to_all")]
#[test_case(new_genericdevice(), vec![]; "generic")]
#[test_case(new_genericlattice(), vec![(0,1), (2,3) ,(0,2), (1,3)]; "lattice")]
fn test_edges(device: Py<PyAny>, test_edges: Vec<(usize, usize)>) {
    Python::with_gil(|py| {
        let edges = device
            .call_method0(py, "two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>(py)
            .unwrap();

        assert_eq!(test_edges.len(), edges.len());
        for edge in edges {
            assert!(test_edges.contains(&edge));
        }
    })
}
