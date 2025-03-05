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

use ndarray::Array2;
use num_complex::Complex64;
use numpy::PyReadonlyArray2;
use pyo3::prelude::*;
use qoqo::operations::{
    convert_operation_to_pyobject, TripleControlledPauliXWrapper, TripleControlledPauliZWrapper,
    TripleControlledPhaseShiftWrapper,
};
use qoqo::CircuitWrapper;
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use roqoqo::{Circuit, RoqoqoError};
use std::collections::HashMap;
use test_case::test_case;

#[test_case(Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(!operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from("theta"))); "TripleControlledPhaseShift")]
fn test_pyo3_is_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "FourQubitGateOperation",
        "TripleControlledPauliX",
        ],
    Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "FourQubitGateOperation",
        "TripleControlledPauliZ",
        ],
    Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "FourQubitGateOperation",
        "TripleControlledPhaseShift",
        ],
    Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_tags(tags: Vec<&str>, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let tags_op: Vec<String> = operation
            .call_method0(py, "tags")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(tags_op.len(), tags.len());
        for i in 0..tags.len() {
            assert_eq!(tags_op[i], tags[i]);
        }
    })
}

#[test_case("TripleControlledPauliX", Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case("TripleControlledPauliZ", Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case("TripleControlledPhaseShift", Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let name_op: String = operation
            .call_method0(py, "hqslang")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(name_op, name.to_string());
    })
}

#[test_case(Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();

        // test initial qubits
        let control_0: usize = operation
            .call_method0(py, "control_0")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control_0.clone(), 0);
        let control_1: usize = operation
            .call_method0(py, "control_1")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control_1.clone(), 1);
        let control_2: usize = operation
            .call_method0(py, "control_2")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control_2.clone(), 2);
        let target: usize = operation
            .call_method0(py, "target")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(target.clone(), 3);

        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);
        qubit_mapping.insert(1, 3);
        qubit_mapping.insert(3, 1);
        let result = operation
            .call_method1(py, "remap_qubits", (qubit_mapping,))
            .unwrap();

        // test re-mapped qubit
        let control_0_new: usize = result
            .call_method0(py, "control_0")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control_0_new.clone(), 2);
        let control_1_new: usize = result
            .call_method0(py, "control_1")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control_1_new.clone(), 3);
        let control_2_new: usize = result
            .call_method0(py, "control_2")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control_2_new.clone(), 0);
        let target_new: usize = result
            .call_method0(py, "target")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(target_new.clone(), 1);

        // test that initial and rempapped qubits are different
        assert_ne!(control_0, control_0_new);
        assert_ne!(control_1, control_1_new);
        assert_ne!(control_2, control_2_new);
        assert_ne!(target, target_new);
    })
}

#[test_case(Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_remapqubits_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_qubits", (qubit_mapping,));
        assert!(result.is_err());
    })
}

#[test_case(Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_unitarymatrix(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix").unwrap();
        let result_matrix = py_result
            .extract::<PyReadonlyArray2<Complex64>>(py)
            .unwrap()
            .as_array()
            .to_owned();

        // compare to reference matrix obtained in Rust directly (without passing to Python)
        let gate: GateOperation = input_operation.try_into().unwrap();
        let rust_matrix: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
        let test_matrix: Array2<Complex64> = rust_matrix.unwrap();

        assert_eq!(result_matrix, test_matrix);
    })
}

#[test_case("TripleControlledPauliX { control_0: 0, control_1: 1, control_2: 2, target: 3 }", Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case("TripleControlledPauliZ { control_0: 0, control_1: 1, control_2: 2, target: 3 }", Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case("TripleControlledPhaseShift { control_0: 0, control_1: 1, control_2: 2, target: 3, theta: Float(1.0) }", Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = to_format.bind(py).extract().unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = to_repr.bind(py).extract().unwrap();
        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    })
}

#[test_case(Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let copy_op = operation.call_method0(py, "__copy__").unwrap();
        let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = copy_op
            .bind(py)
            .call_method1("__eq__", (copy_deepcopy_param.clone_ref(py),))
            .unwrap()
            .extract::<bool>()
            .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_op
                .bind(py)
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

#[test_case(Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)), Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)), Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from("test"))), Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_substitute_parameters(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();
        let mut substitution_dict: HashMap<String, f64> = HashMap::new();
        substitution_dict.insert("test".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();
        let substitute_param = convert_operation_to_pyobject(second_op).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (substitute_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test_case(Operation::from(TripleControlledPauliX::new(0, 1, 2, 3)), (0 ,1, 2, 3), "__eq__"; "TripleControlledPauliX_eq")]
#[test_case(Operation::from(TripleControlledPauliX::new(3, 2, 1, 0)), (0 ,1, 2, 3), "__ne__"; "TripleControlledPauliX_ne")]
fn test_new_triplecontrolledpaulix(
    input_operation: Operation,
    arguments: (u32, u32, u32, u32),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<TripleControlledPauliXWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<TripleControlledPauliXWrapper>().unwrap();
        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        assert!(result.is_err());

        let result = operation_type.call1((0, vec!["fails"], 2));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<TripleControlledPauliXWrapper>()
            .unwrap();
        let binding = operation_type.call1((1, 2, 3, 4)).unwrap();
        let new_op_diff = binding.downcast::<TripleControlledPauliXWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<TripleControlledPauliXWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "TripleControlledPauliXWrapper { internal: TripleControlledPauliX { control_0: 1, control_1: 2, control_2: 3, target: 4 } }"
        );
    })
}

#[test_case(Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3)), (0 ,1, 2, 3), "__eq__"; "TripleControlledPauliZ_eq")]
#[test_case(Operation::from(TripleControlledPauliZ::new(3, 2, 1, 0)), (0 ,1, 2, 3), "__ne__"; "TripleControlledPauliZ_ne")]
fn test_new_triplecontrolledpauliz(
    input_operation: Operation,
    arguments: (u32, u32, u32, u32),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<TripleControlledPauliZWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<TripleControlledPauliZWrapper>().unwrap();
        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        assert!(result.is_err());

        let result = operation_type.call1((0, vec!["fails"], 2));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<TripleControlledPauliZWrapper>()
            .unwrap();
        let binding = operation_type.call1((1, 2, 3, 4)).unwrap();
        let new_op_diff = binding.downcast::<TripleControlledPauliZWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<TripleControlledPauliZWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "TripleControlledPauliZWrapper { internal: TripleControlledPauliZ { control_0: 1, control_1: 2, control_2: 3, target: 4 } }"
        );
    })
}

#[test_case(Operation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))), (0 ,1, 2, 3, 1.0), "__eq__"; "TripleControlledPhaseShift_eq")]
#[test_case(Operation::from(TripleControlledPhaseShift::new(3, 2, 1, 0, CalculatorFloat::from(1.0))), (0 ,1, 2, 3, 1.0), "__ne__"; "TripleControlledPhaseShift_ne")]
fn test_new_triplecontrolledphaseshift(
    input_operation: Operation,
    arguments: (u32, u32, u32, u32, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<TripleControlledPhaseShiftWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding
            .downcast::<TripleControlledPhaseShiftWrapper>()
            .unwrap();
        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, 1, vec!["fails"]));
        assert!(result.is_err());

        let result = operation_type.call1((0, vec!["fails"], 2));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<TripleControlledPhaseShiftWrapper>()
            .unwrap();
        let binding = operation_type.call1((1, 2, 3, 4, 1.0)).unwrap();
        let new_op_diff = binding
            .downcast::<TripleControlledPhaseShiftWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<TripleControlledPhaseShiftWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "TripleControlledPhaseShiftWrapper { internal: TripleControlledPhaseShift { control_0: 1, control_1: 2, control_2: 3, target: 4, theta: Float(1.0) } }"
        );
    })
}

#[test]
fn test_circuit_pyo3_triplecontrolledpaulix() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(TripleControlledPauliX::new(0, 1, 2, 3));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += CNOT::new(0, 3);
        circuit += CNOT::new(0, 1);
        circuit += CNOT::new(1, 3);
        circuit += CNOT::new(0, 1);
        circuit += CNOT::new(1, 3);
        circuit += CNOT::new(1, 2);
        circuit += CNOT::new(2, 3);
        circuit += CNOT::new(0, 2);
        circuit += CNOT::new(2, 3);
        circuit += CNOT::new(1, 2);
        circuit += CNOT::new(2, 3);
        circuit += CNOT::new(0, 2);
        circuit += CNOT::new(2, 3);

        assert_eq!(result_circuit.internal, circuit);
    });
}

#[test]
fn test_circuit_pyo3_triplecontrolledpauliz() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(TripleControlledPauliZ::new(0, 1, 2, 3));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += ControlledPauliZ::new(0, 3);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPauliZ::new(1, 3);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPauliZ::new(1, 3);
        circuit += CNOT::new(1, 2);
        circuit += ControlledPauliZ::new(2, 3);
        circuit += CNOT::new(0, 2);
        circuit += ControlledPauliZ::new(2, 3);
        circuit += CNOT::new(1, 2);
        circuit += ControlledPauliZ::new(2, 3);
        circuit += CNOT::new(0, 2);
        circuit += ControlledPauliZ::new(2, 3);

        assert_eq!(result_circuit.internal, circuit);
    });
}

#[test]
fn test_circuit_pyo3_triplecontrolledphaseshift() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(TripleControlledPhaseShift::new(
            0,
            1,
            2,
            3,
            CalculatorFloat::FRAC_PI_2,
        ));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += ControlledPhaseShift::new(0, 3, CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPhaseShift::new(1, 3, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPhaseShift::new(1, 3, CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(1, 2);
        circuit += ControlledPhaseShift::new(2, 3, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += ControlledPhaseShift::new(2, 3, CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(1, 2);
        circuit += ControlledPhaseShift::new(2, 3, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += ControlledPhaseShift::new(2, 3, CalculatorFloat::FRAC_PI_4);

        assert_eq!(result_circuit.internal, circuit);
    });
}

/// Test json_schema function for all three qubit gate operations
#[cfg(feature = "json_schema")]
#[test_case(FourQubitGateOperation::from(TripleControlledPauliX::new(0, 1, 2, 3)); "TripleControlledPauliX")]
#[test_case(FourQubitGateOperation::from(TripleControlledPauliZ::new(0, 1, 2, 3)); "TripleControlledPauliZ")]
#[test_case(FourQubitGateOperation::from(TripleControlledPhaseShift::new(0, 1, 2, 3, CalculatorFloat::from(1.0))); "TripleControlledPhaseShift")]
fn test_pyo3_json_schema(operation: FourQubitGateOperation) {
    let rust_schema = match operation {
        FourQubitGateOperation::TripleControlledPauliX(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(TripleControlledPauliX)).unwrap()
        }
        FourQubitGateOperation::TripleControlledPauliZ(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(TripleControlledPauliZ)).unwrap()
        }
        FourQubitGateOperation::TripleControlledPhaseShift(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(TripleControlledPhaseShift))
                .unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let converted_op = Operation::from(operation);
        let pyobject = convert_operation_to_pyobject(converted_op).unwrap();
        let operation = pyobject.bind(py);

        let schema: String =
            String::extract_bound(&operation.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&operation.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&operation.call_method0("min_supported_version").unwrap())
                .unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.16.0");
    });
}
