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

use ndarray::Array2;
use num_complex::Complex64;
use numpy::PyArray2;
use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::MultiQubitMSWrapper;
use qoqo::CircuitWrapper;
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
use roqoqo::{Circuit, RoqoqoError};
use std::collections::HashMap;
use std::convert::TryInto;
use test_case::test_case;

// helper function to convert CalculatorFloat into a python object
fn convert_cf_to_pyobject(
    py: Python,
    parameter: CalculatorFloat,
) -> &PyCell<CalculatorFloatWrapper> {
    let parameter_type = py.get_type::<CalculatorFloatWrapper>();
    match parameter {
        CalculatorFloat::Float(x) => parameter_type
            .call1((x,))
            .unwrap()
            .cast_as::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .cast_as::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
    }
}

#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::ZERO)), (vec![0, 1], 0.0,), "__eq__"; "MultiQubitMS_eq")]
#[test_case(Operation::from(MultiQubitMS::new(vec![2, 3], CalculatorFloat::ZERO)), (vec![0, 1], 0.0,), "__ne__"; "MultiQubitMS_ne")]
fn test_new_multi_qubit_ms(input_operation: Operation, arguments: (Vec<u32>, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    // Basic initialisation, no errors
    let operation_type = py.get_type::<MultiQubitMSWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<MultiQubitMSWrapper>>()
        .unwrap();
    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    // Error initialisation
    let result = operation_type.call1(([0, 1], vec!["fails"]));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    // Testing PartialEq, Clone and Debug
    let def_wrapper = operation_py.extract::<MultiQubitMSWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((vec![1, 2], 0.0))
        .unwrap()
        .cast_as::<PyCell<MultiQubitMSWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<MultiQubitMSWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "MultiQubitMSWrapper { internal: MultiQubitMS { qubits: [1, 2], theta: Float(0.0) } }"
    );
}

/// Test is_parametrized() function for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from("theta"))); "MultiQubitMS")]
fn test_pyo3_is_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    assert!(bool::extract(
        operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .as_ref(py)
    )
    .unwrap());
}

/// Test is_parametrized = false for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::PI)); "MultiQubitMS")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    assert!(!bool::extract(
        operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .as_ref(py)
    )
    .unwrap());
}

/// Test theta() function for Rotations
#[test_case(CalculatorFloat::from(0), Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitMS float")]
#[test_case(CalculatorFloat::from("theta"), Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from("theta"))); "MultiQubitMS symb")]
fn test_pyo3_theta(theta: CalculatorFloat, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let theta_op: CalculatorFloatWrapper =
        CalculatorFloatWrapper::extract(operation.call_method0(py, "theta").unwrap().as_ref(py))
            .unwrap();
    let theta_param: CalculatorFloatWrapper =
        CalculatorFloatWrapper::extract(convert_cf_to_pyobject(py, theta)).unwrap();
    assert_eq!(theta_op.cf_internal, theta_param.cf_internal);
}

/// Test qubits() function for MultiQubitGate Operations
#[test_case(vec![0, 1], Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitMS two")]
#[test_case(vec![0, 1, 2], Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0))); "MultiQubitMS three")]
fn test_pyo3_qubits(qubit: Vec<usize>, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let qubit_op: Vec<usize> = operation
        .call_method0(py, "qubits")
        .unwrap()
        .as_ref(py)
        .extract()
        .unwrap();
    assert_eq!(qubit_op, qubit);
}

/// Test hqslang() function for MultiQubitGate Operations
#[test_case("MultiQubitMS", Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitMS")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let name_op: String =
        String::extract(operation.call_method0(py, "hqslang").unwrap().as_ref(py)).unwrap();
    assert_eq!(name_op, name.to_string());
}

/// Test tags() function for MultiQubitGate Operations
#[test_case(
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "MultiQubitGateOperation",
        // "Rotation",
        "MultiQubitMS",
        ];
    "MultiQubitMS")]
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let tags_op: Vec<String> =
        Vec::<String>::extract(operation.call_method0(py, "tags").unwrap().as_ref(py)).unwrap();
    assert_eq!(tags_op.len(), tags.len());
    for i in 0..tags.len() {
        assert_eq!(tags_op[i], tags[i]);
    }
}

/// Test remap_qubits() function for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    // test initial qubit
    let qubits: Vec<usize> = operation
        .call_method0(py, "qubits")
        .unwrap()
        .as_ref(py)
        .extract()
        .unwrap();
    assert_eq!(qubits.clone(), vec![0, 1, 2]);
    // remap qubits
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    qubit_mapping.insert(1, 2);
    qubit_mapping.insert(2, 0);
    let result = operation
        .call_method1(py, "remap_qubits", (qubit_mapping,))
        .unwrap();
    // test re-mapped qubit
    let qubits_new: Vec<usize> = result
        .call_method0(py, "qubits")
        .unwrap()
        .as_ref(py)
        .extract()
        .unwrap();
    assert_eq!(qubits_new.clone(), vec![1, 2, 0]);
    // test that initial and rempapped qubits are different
    assert_ne!(qubits, qubits_new);
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
fn test_pyo3_remapqubits_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    // remap qubits
    let qubit_mapping: HashMap<usize, usize> = HashMap::new();
    let result = operation.call_method1(py, "remap_qubits", (qubit_mapping,));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());
}

/// Test unitary_matrix() function for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
fn test_pyo3_unitarymatrix(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let py_result = operation.call_method0(py, "unitary_matrix").unwrap();
    let result_matrix = py_result
        .cast_as::<PyArray2<Complex64>>(py)
        .unwrap()
        .to_owned_array();

    // compare to reference matrix obtained in Rust directly (without passing to Python)
    let gate: MultiQubitGateOperation = input_operation.try_into().unwrap();
    let rust_matrix: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    let test_matrix: Array2<Complex64> = rust_matrix.unwrap();

    assert_eq!(result_matrix, test_matrix);
}

/// Test unitary_matrix() function for MultiQubitGate Operations for the error case
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from("PI"))); "MultiQubitMS")]
fn test_pyo3_unitarymatrix_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let py_result = operation.call_method0(py, "unitary_matrix");
    let result_ref = py_result.as_ref();
    assert!(result_ref.is_err());
}

/// Test circuit() function for MultiQubitMS
#[test]
fn test_pyo3_circuit_ms() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let input_operation =
        Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.0)));
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let py_result = operation.call_method0(py, "circuit").unwrap();
    let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

    let mut circuit = Circuit::new();
    circuit += Hadamard::new(0);
    circuit += Hadamard::new(1);
    circuit += Hadamard::new(2);
    circuit += CNOT::new(0, 1);
    circuit += CNOT::new(1, 2);
    circuit += RotateZ::new(2, CalculatorFloat::from(0.5));
    circuit += CNOT::new(1, 2);
    circuit += CNOT::new(0, 1);
    circuit += Hadamard::new(0);
    circuit += Hadamard::new(1);
    circuit += Hadamard::new(2);

    assert_eq!(result_circuit.internal, circuit);
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let copy_op = operation.call_method0(py, "__copy__").unwrap();
    let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();
    let copy_deepcopy_param = operation.clone();

    let comparison_copy = bool::extract(
        copy_op
            .as_ref(py)
            .call_method1("__eq__", (copy_deepcopy_param.clone(),))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_copy);
    let comparison_deepcopy = bool::extract(
        deepcopy_op
            .as_ref(py)
            .call_method1("__eq__", (copy_deepcopy_param,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_deepcopy);
}

/// Test format and repr functions
#[test_case(
    "MultiQubitMS { qubits: [0, 1, 2], theta: Float(0.0) }",
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::ZERO));
    "MultiQubitMS")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
    let format_op: &str = <&str>::extract(to_format.as_ref(py)).unwrap();
    assert_eq!(format_op, format_repr);
    let to_repr = operation.call_method0(py, "__repr__").unwrap();
    let repr_op: &str = <&str>::extract(to_repr.as_ref(py)).unwrap();
    assert_eq!(repr_op, format_repr);
}

/// Test substitute_parameters() function for one parameter
#[test_case(Operation::from(MultiQubitMS::new(vec![1, 2, 3], CalculatorFloat::from("theta"))); "MultiQubitMS")]
fn test_pyo3_substitute_params_rotate(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
    substitution_dict_py.insert("theta", 1.0);
    let substitute_op = operation
        .call_method1(py, "substitute_parameters", (substitution_dict_py,))
        .unwrap();

    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("theta", 1.0);
    let substitute_param = input_operation
        .substitute_parameters(&mut substitution_dict)
        .unwrap();
    let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

    let comparison = bool::extract(
        substitute_op
            .as_ref(py)
            .call_method1("__eq__", (test_operation.clone(),))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);
}

/// Test substitute_parameters() causing an error `None`
#[test_case(Operation::from(MultiQubitMS::new(vec![1, 2], CalculatorFloat::from("test"))); "MultiQubitMS")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let substitution_dict: HashMap<&str, f64> = HashMap::new();
    let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());
}

#[test_case(
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0.005))),
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0.005 * 1.5))); "MultiQubitMS")]
fn test_pyo3_rotate_powercf(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(first_op).unwrap();

    let power = convert_cf_to_pyobject(py, CalculatorFloat::from(1.5));
    let comparison_op = convert_operation_to_pyobject(second_op).unwrap();

    let remapped_op = operation.call_method1(py, "powercf", (power,)).unwrap();
    let comparison = bool::extract(
        remapped_op
            .call_method1(py, "__eq__", (comparison_op,))
            .unwrap()
            .as_ref(py),
    )
    .unwrap();
    assert!(comparison);
}

/// Test the __richcmp__ function
#[test_case(
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0))),
    Operation::from(MultiQubitMS::new(vec![1, 2], CalculatorFloat::from(0))); "MultiQubitMS")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_one = convert_operation_to_pyobject(definition_1).unwrap();
    let operation_two = convert_operation_to_pyobject(definition_2).unwrap();

    let comparison = bool::extract(
        operation_one
            .as_ref(py)
            .call_method1("__eq__", (operation_two.clone(),))
            .unwrap(),
    )
    .unwrap();
    assert!(!comparison);

    let comparison = bool::extract(
        operation_one
            .as_ref(py)
            .call_method1("__ne__", (operation_two.clone(),))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let comparison = operation_one.call_method1(py, "__eq__", (vec!["fails"],));
    assert!(comparison.is_err());

    let comparison = operation_one.call_method1(py, "__ge__", (operation_two,));
    assert!(comparison.is_err());
}
