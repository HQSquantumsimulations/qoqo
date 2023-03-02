// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use qoqo_calculator_pyo3::CalculatorFloatWrapper;

use std::collections::HashMap;

use pyo3::prelude::*;

use qoqo::{
    operations::{
        convert_operation_to_pyobject, ControlledControlledPauliZWrapper,
        ControlledControlledPhaseShiftWrapper,
    },
    CircuitWrapper,
};
use qoqo_calculator::CalculatorFloat;
use roqoqo::{operations::*, Circuit, RoqoqoError};

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
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
        CalculatorFloat::Str(x) => parameter_type
            .call1((x,))
            .unwrap()
            .downcast::<PyCell<CalculatorFloatWrapper>>()
            .unwrap(),
    }
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(!bool::extract(
            operation
                .call_method0(py, "is_parametrized")
                .unwrap()
                .as_ref(py)
        )
        .unwrap());
    })
}

#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "ControlledControlledPauliZ",
        ],
    Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "Rotation",
        "ControlledControlledPhaseShift",
        ],
    Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_pyo3_tags(tags: Vec<&str>, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let tags_op: Vec<String> =
            Vec::<String>::extract(operation.call_method0(py, "tags").unwrap().as_ref(py)).unwrap();
        assert_eq!(tags_op.len(), tags.len());
        for i in 0..tags.len() {
            assert_eq!(tags_op[i], tags[i]);
        }
    })
}

#[test_case("ControlledControlledPauliZ", Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case("ControlledControlledPhaseShift", Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let name_op: String =
            String::extract(operation.call_method0(py, "hqslang").unwrap().as_ref(py)).unwrap();
        assert_eq!(name_op, name.to_string());
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();

        // test initial qubits
        let control_0: usize =
            usize::extract(operation.call_method0(py, "control_0").unwrap().as_ref(py)).unwrap();
        assert_eq!(control_0.clone(), 0);
        let control_1: usize =
            usize::extract(operation.call_method0(py, "control_1").unwrap().as_ref(py)).unwrap();
        assert_eq!(control_1.clone(), 1);
        let target: usize =
            usize::extract(operation.call_method0(py, "target").unwrap().as_ref(py)).unwrap();
        assert_eq!(target.clone(), 2);

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
        let control_0_new: usize =
            usize::extract(result.call_method0(py, "control_0").unwrap().as_ref(py)).unwrap();
        assert_eq!(control_0_new.clone(), 2);
        let control_1_new: usize =
            usize::extract(result.call_method0(py, "control_1").unwrap().as_ref(py)).unwrap();
        assert_eq!(control_1_new.clone(), 3);
        let target_new: usize =
            usize::extract(result.call_method0(py, "target").unwrap().as_ref(py)).unwrap();
        assert_eq!(target_new.clone(), 0);

        // test that initial and rempapped qubits are different
        assert_ne!(control_0, control_0_new);
        assert_ne!(control_1, control_1_new);
        assert_ne!(target, target_new);
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_pyo3_remapqubits_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_qubits", (qubit_mapping,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("theta"))); "ControlledControlledPhaseShift")]
fn test_pyo3_unitarymatrix_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix");
        let result_ref = py_result.as_ref();
        assert!(result_ref.is_err());
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_pyo3_unitarymatrix(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix").unwrap();
        let result_matrix = py_result
            .downcast::<PyArray2<Complex64>>(py)
            .unwrap()
            .to_owned_array();

        // compare to reference matrix obtained in Rust directly (without passing to Python)
        let gate: GateOperation = input_operation.try_into().unwrap();
        let rust_matrix: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
        let test_matrix: Array2<Complex64> = rust_matrix.unwrap();

        assert_eq!(result_matrix, test_matrix);
    })
}

#[test_case(
    "ControlledControlledPauliZ { control_0: 1, control_1: 0, target: 2 }",
    Operation::from(ControlledControlledPauliZ::new(1, 0, 2)); "ControlledControlledPauliZ")]
#[test_case(
    "ControlledControlledPhaseShift { control_0: 1, control_1: 0, target: 2, theta: Float(-1.0) }",
    Operation::from(ControlledControlledPhaseShift::new(1, 0, 2, CalculatorFloat::from(-1.0))); "ControlledControlledPhaseShift")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format.as_ref(py)).unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr.as_ref(py)).unwrap();
        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let copy_op = operation.call_method0(py, "__copy__").unwrap();
        let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

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
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)),
            Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("test"))),
            Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(1.0))); "ControlledControlledPhaseShift")]
fn test_pyo3_substitute_parameters(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();
        let mut substitution_dict: HashMap<&str, f64> = HashMap::new();
        substitution_dict.insert("test", 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();
        let substitute_param = convert_operation_to_pyobject(second_op).unwrap();

        let comparison = bool::extract(
            substitute_op
                .as_ref(py)
                .call_method1("__eq__", (substitute_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("test"))); "ControlledControlledPhaseShift")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<&str, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(1.0))),
            Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(1.0 * 1.5))); "ControlledControlledPhaseShift")]
fn test_pyo3_powercf(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
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
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)), (0 ,1, 2), "__eq__"; "ControlleControlledPauliZ_eq")]
#[test_case(Operation::from(ControlledControlledPauliZ::new(2, 1, 0)), (0 ,1, 2), "__ne__"; "ControlleControlledPauliZ_ne")]
fn test_new_controlledcontrolledpauliz(
    input_operation: Operation,
    arguments: (u32, u32, u32),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<ControlledControlledPauliZWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ControlledControlledPauliZWrapper>>()
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
        let result = operation_type.call1((0, 1, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, vec!["fails"], 2));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<ControlledControlledPauliZWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 3))
            .unwrap()
            .downcast::<PyCell<ControlledControlledPauliZWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ControlledControlledPauliZWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledControlledPauliZWrapper { internal: ControlledControlledPauliZ { control_0: 1, control_1: 2, target: 3 } }"
        );
    })
}

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.0))), (0 ,1, 2, 0.0), "__eq__"; "ControlleControlledPhaseShift_eq")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(2, 1, 0, CalculatorFloat::from(0.0))), (0 ,1, 2, 0.0), "__ne__"; "ControlleControlledPhaseShift_ne")]
fn test_new_controlledcontrolledphaseshift(
    input_operation: Operation,
    arguments: (u32, u32, u32, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<ControlledControlledPhaseShiftWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<ControlledControlledPhaseShiftWrapper>>()
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
        let result = operation_type.call1((0, 1, 2, vec!["fails"]));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        let result = operation_type.call1((0, vec!["fails"], 2));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<ControlledControlledPhaseShiftWrapper>()
            .unwrap();
        let new_op_diff = operation_type
            .call1((1, 2, 3, 0.0))
            .unwrap()
            .downcast::<PyCell<ControlledControlledPhaseShiftWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<ControlledControlledPhaseShiftWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledControlledPhaseShiftWrapper { internal: ControlledControlledPhaseShift { control_0: 1, control_1: 2, target: 3, theta: Float(0.0) } }"
        );
    })
}

#[test]
fn test_circuit_pyo3_controlledcontrolledpauliz() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(ControlledControlledPauliZ::new(0, 1, 2));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += CNOT::new(0, 2);
        circuit += ControlledPhaseShift::new(1, 2, -CalculatorFloat::FRAC_PI_2);
        circuit += CNOT::new(0, 2);
        circuit += ControlledPhaseShift::new(1, 2, CalculatorFloat::FRAC_PI_2);

        assert_eq!(result_circuit.internal, circuit);
    });
}

#[test]
fn test_circuit_pyo3_controlledcontrolledphaseshift() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(ControlledControlledPhaseShift::new(
            0,
            1,
            2,
            CalculatorFloat::PI,
        ));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += CNOT::new(0, 2);
        circuit += ControlledPhaseShift::new(1, 2, -CalculatorFloat::PI);
        circuit += CNOT::new(0, 2);
        circuit += ControlledPhaseShift::new(1, 2, CalculatorFloat::PI);

        assert_eq!(result_circuit.internal, circuit);
    });
}
