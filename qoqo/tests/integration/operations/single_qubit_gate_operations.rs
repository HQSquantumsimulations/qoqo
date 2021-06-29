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
use qoqo::operations::{
    HadamardWrapper, InvSqrtPauliXWrapper, PauliXWrapper, PauliYWrapper, PauliZWrapper,
    RotateAroundSphericalAxisWrapper, RotateXWrapper, RotateYWrapper, RotateZWrapper, SGateWrapper,
    SingleQubitGateWrapper, SqrtPauliXWrapper, TGateWrapper,
};
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::Operation;
use roqoqo::operations::SingleQubitGateOperation;
use roqoqo::operations::*;
use roqoqo::RoqoqoError;
use std::collections::HashMap;
use std::convert::TryInto;
use std::f64::consts::PI;
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

/// Test new() function for PauliX
#[test_case(Operation::from(PauliX::new(1)), (1,), "__eq__"; "PauliX_eq")]
#[test_case(Operation::from(PauliX::new(1)), (0,), "__ne__"; "PauliX_ne")]
fn test_new_paulix(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<PauliXWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<PauliXWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<PauliXWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<PauliXWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<PauliXWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "PauliXWrapper { internal: PauliX { qubit: 2 } }"
    );
}

/// Test new() function for PauliY
#[test_case(Operation::from(PauliY::new(1)), (1,), "__eq__"; "PauliY_eq")]
#[test_case(Operation::from(PauliY::new(1)), (0,), "__ne__"; "PauliY_ne")]
fn test_new_pauliy(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<PauliYWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<PauliYWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<PauliYWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<PauliYWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<PauliYWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "PauliYWrapper { internal: PauliY { qubit: 2 } }"
    );
}

/// Test new() function for PauliZ
#[test_case(Operation::from(PauliZ::new(1)), (1,), "__eq__"; "PauliZ_eq")]
#[test_case(Operation::from(PauliZ::new(1)), (0,), "__ne__"; "PauliZ_ne")]
fn test_new_pauliz(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<PauliZWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<PauliZWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<PauliZWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<PauliZWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<PauliZWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "PauliZWrapper { internal: PauliZ { qubit: 2 } }"
    );
}

/// Test new() function for SqrtPauliX
#[test_case(Operation::from(SqrtPauliX::new(1)), (1,), "__eq__"; "SqrtPauliX_eq")]
#[test_case(Operation::from(SqrtPauliX::new(1)), (0,), "__ne__"; "SqrtPauliX_ne")]
fn test_new_sqrtpaulix(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<SqrtPauliXWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<SqrtPauliXWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<SqrtPauliXWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<SqrtPauliXWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<SqrtPauliXWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "SqrtPauliXWrapper { internal: SqrtPauliX { qubit: 2 } }"
    );
}

/// Test new() function for InvSqrtPauliX
#[test_case(Operation::from(InvSqrtPauliX::new(1)), (1,), "__eq__"; "InvSqrtPauliX_eq")]
#[test_case(Operation::from(InvSqrtPauliX::new(1)), (0,), "__ne__"; "InvSqrtPauliX_ne")]
fn test_new_invsqrtpaulix(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<InvSqrtPauliXWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<InvSqrtPauliXWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<InvSqrtPauliXWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<InvSqrtPauliXWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<InvSqrtPauliXWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "InvSqrtPauliXWrapper { internal: InvSqrtPauliX { qubit: 2 } }"
    );
}

#[test_case(Operation::from(SGate::new(1)), (1,), "__eq__"; "SGate_eq")]
#[test_case(Operation::from(SGate::new(1)), (0,), "__ne__"; "SGate_ne")]
fn test_new_sgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<SGateWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<SGateWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<SGateWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<SGateWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<SGateWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "SGateWrapper { internal: SGate { qubit: 2 } }"
    );
}

#[test_case(Operation::from(TGate::new(1)), (1,), "__eq__"; "TGate_eq")]
#[test_case(Operation::from(TGate::new(1)), (0,), "__ne__"; "TGate_ne")]
fn test_new_tgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<TGateWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<TGateWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<TGateWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<TGateWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<TGateWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "TGateWrapper { internal: TGate { qubit: 2 } }"
    );
}

#[test_case(Operation::from(Hadamard::new(1)), (1,), "__eq__"; "Hadamard_eq")]
#[test_case(Operation::from(Hadamard::new(1)), (0,), "__ne__"; "Hadamard_ne")]
fn test_new_hadamard(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();

    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation_type = py.get_type::<HadamardWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<HadamardWrapper>>()
        .unwrap();

    let comparison = bool::extract(
        operation
            .as_ref(py)
            .call_method1(method, (operation_py,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);

    let def_wrapper = operation_py.extract::<HadamardWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2,))
        .unwrap()
        .cast_as::<PyCell<HadamardWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<HadamardWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "HadamardWrapper { internal: Hadamard { qubit: 2 } }"
    );
}

#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "RotateX_eq")]
#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "RotateX_ne")]
fn test_new_rotatex(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    // Basic initialisation, no errors
    let operation_type = py.get_type::<RotateXWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<RotateXWrapper>>()
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
    let result = operation_type.call1((0, vec!["fails"]));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    // Testing PartialEq, Clone and Debug
    let def_wrapper = operation_py.extract::<RotateXWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2, 0.0))
        .unwrap()
        .cast_as::<PyCell<RotateXWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<RotateXWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "RotateXWrapper { internal: RotateX { qubit: 2, theta: Float(0.0) } }"
    );
}

#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "RotateY_eq")]
#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "RotateY_ne")]
fn test_new_rotatey(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    // Basic initialisation, no errors
    let operation_type = py.get_type::<RotateYWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<RotateYWrapper>>()
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
    let result = operation_type.call1((0, vec!["fails"]));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    // Testing PartialEq, Clone and Debug
    let def_wrapper = operation_py.extract::<RotateYWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2, 0.0))
        .unwrap()
        .cast_as::<PyCell<RotateYWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<RotateYWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "RotateYWrapper { internal: RotateY { qubit: 2, theta: Float(0.0) } }"
    );
}

#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "RotateZ_eq")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "RotateZ_ne")]
fn test_new_rotatez(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    // Basic initialisation, no errors
    let operation_type = py.get_type::<RotateZWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<RotateZWrapper>>()
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
    let result = operation_type.call1((0, vec!["fails"]));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    // Testing PartialEq, Clone and Debug
    let def_wrapper = operation_py.extract::<RotateZWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2, 0.0))
        .unwrap()
        .cast_as::<PyCell<RotateZWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<RotateZWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "RotateZWrapper { internal: RotateZ { qubit: 2, theta: Float(0.0) } }"
    );
}

#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        1,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        )
    ), (1, 0.0, 0.0, 0.0,), "__eq__"; "rotation_eq")]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        1,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        )
    ), (0, 0.0, 0.0, 0.0,), "__ne__"; "rotation_ne")]
fn test_new_rotate(input_operation: Operation, arguments: (u32, f64, f64, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    // Basic initialisation, no errors
    let operation_type = py.get_type::<RotateAroundSphericalAxisWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<RotateAroundSphericalAxisWrapper>>()
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
    let result = operation_type.call1((0, vec!["fails"], 0.0, 0.0));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());
    let result = operation_type.call1((0, 0.0, vec!["fails"], 0.0));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());
    let result = operation_type.call1((0, 0.0, 0.0, vec!["fails"]));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    // Testing PartialEq, Clone and Debug
    let def_wrapper = operation_py
        .extract::<RotateAroundSphericalAxisWrapper>()
        .unwrap();
    let new_op_diff = operation_type
        .call1((2, 0.0, 0.0, 0.0))
        .unwrap()
        .cast_as::<PyCell<RotateAroundSphericalAxisWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff
        .extract::<RotateAroundSphericalAxisWrapper>()
        .unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "RotateAroundSphericalAxisWrapper { internal: RotateAroundSphericalAxis { qubit: 2, theta: Float(0.0), spherical_theta: Float(0.0), spherical_phi: Float(0.0) } }"
    );
}

#[test_case(Operation::from(
    SingleQubitGate::new(
        1,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::from("global_phase"),
        )
    ), (1, 0.0, 0.0, 0.0, 0.0, "global_phase".to_string(),), "__eq__"; "rotation_eq")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        1,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        )
    ), (1, 0.0, 0.0, 0.0, 0.0, "global_phase".to_string(),), "__ne__"; "rotation_ne")]
fn test_new_singlequbitgate(
    input_operation: Operation,
    arguments: (u32, f64, f64, f64, f64, String),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    // Basic initialisation, no errors
    let operation_type = py.get_type::<SingleQubitGateWrapper>();
    let operation_py = operation_type
        .call1(arguments)
        .unwrap()
        .cast_as::<PyCell<SingleQubitGateWrapper>>()
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
    let result = operation_type.call1((0, vec!["fails"], 0.0, 0.0, 0.0, 0.0));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    let result = operation_type.call1((0, 0.0, vec!["fails"], 0.0, 0.0, 0.0));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    let result = operation_type.call1((0, 0.0, 0.0, vec!["fails"], 0.0, 0.0));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    let result = operation_type.call1((0, 0.0, 0.0, 0.0, vec!["fails"], 0.0));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    let result = operation_type.call1((0, 0.0, 0.0, 0.0, 0.0, vec!["fails"]));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());

    // Testing PartialEq, Clone and Debug
    let def_wrapper = operation_py.extract::<SingleQubitGateWrapper>().unwrap();
    let new_op_diff = operation_type
        .call1((2, 0.0, 0.0, 0.0, 0.0, 0.0))
        .unwrap()
        .cast_as::<PyCell<SingleQubitGateWrapper>>()
        .unwrap();
    let def_wrapper_diff = new_op_diff.extract::<SingleQubitGateWrapper>().unwrap();
    let helper_ne: bool = def_wrapper_diff != def_wrapper.clone();
    assert!(helper_ne);
    let helper_eq: bool = def_wrapper == def_wrapper.clone();
    assert!(helper_eq);

    assert_eq!(
        format!("{:?}", def_wrapper_diff),
        "SingleQubitGateWrapper { internal: SingleQubitGate { qubit: 2, alpha_r: Float(0.0), alpha_i: Float(0.0), beta_r: Float(0.0), beta_i: Float(0.0), global_phase: Float(0.0) } }"
    );
}

/// Test is_parametrized() function for SingleQubitGate Operations
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("theta"))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from("theta"))); "RotateY float")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from("theta"))); "RotateZ float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from("alpha_r"),
        CalculatorFloat::from("alpha_i"),
        CalculatorFloat::from("beta_r"),
        CalculatorFloat::from("beta_i"),
        CalculatorFloat::from("global_phase"),  
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("spherical_theta"),
        CalculatorFloat::from("spherical_phi"),
        )
    ); "RotateAroundSphericalAxis")
]
fn test_pyo3_is_parametrized(input_operation: Operation) {
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

/// Test is_parametrized = false for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
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
#[test_case(CalculatorFloat::from(0), Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(CalculatorFloat::from("theta"), Operation::from(RotateX::new(0, CalculatorFloat::from("theta"))); "RotateX symb")]
#[test_case(CalculatorFloat::from(1.3), Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(CalculatorFloat::from(PI), Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(CalculatorFloat::from(0), Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0.0),
        )
    ); "RotateAroundSphericalAxis")
]
fn test_pyo3_theta(theta: CalculatorFloat, input_operation: Operation) {
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

/// Test qubit() function for SingleQubitGate Operations
#[test_case(0, Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(0, Operation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case(0, Operation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(0, Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(0, Operation::from(PauliX::new(0)); "PauliX")]
#[test_case(0, Operation::from(PauliY::new(0)); "PauliY")]
#[test_case(0, Operation::from(PauliZ::new(0)); "PauliZ")]
#[test_case(0, Operation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case(0, Operation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case(0, Operation::from(TGate::new(0)); "TGate")]
#[test_case(0, Operation::from(SGate::new(0)); "SGate")]
#[test_case(0, Operation::from(Hadamard::new(0)); "Hadamard")]
fn test_pyo3_qubit(qubit: usize, input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let qubit_op: usize =
        usize::extract(operation.call_method0(py, "qubit").unwrap().as_ref(py)).unwrap();
    assert_eq!(qubit_op, qubit);
}

/// Test RotateX hqslang() function for SingleQubitGate Operations
#[test_case("RotateX", Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case("RotateY", Operation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case("RotateZ", Operation::from(RotateZ::new(0, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(
    "RotateAroundSphericalAxis",
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case("PauliX", Operation::from(PauliX::new(0)); "PauliX")]
#[test_case("PauliY", Operation::from(PauliY::new(0)); "PauliY")]
#[test_case("PauliZ", Operation::from(PauliZ::new(0)); "PauliZ")]
#[test_case("SqrtPauliX", Operation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case("InvSqrtPauliX", Operation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case("SGate", Operation::from(SGate::new(0)); "SGate")]
#[test_case("TGate", Operation::from(TGate::new(0)); "TGate")]
#[test_case("Hadamard", Operation::from(Hadamard::new(0)); "Hadamard")]
#[test_case("SingleQubitGate", Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    )); "SingleQubitGate")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let name_op: String =
        String::extract(operation.call_method0(py, "hqslang").unwrap().as_ref(py)).unwrap();
    assert_eq!(name_op, name.to_string());
}

/// Test RotateX tags() function for SingleQubitGate Operations
#[test_case(
    Operation::from(RotateX::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateX",
        ];
    "RotateX")]
#[test_case(
    Operation::from(RotateY::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateY",
        ];
    "RotateY")]
#[test_case(
    Operation::from(RotateZ::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateZ",
        ];
    "RotateZ")]
#[test_case(
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "RotateAroundSphericalAxis",
        ];
    "RotateAroundSphericalAxis")]
#[test_case(
    Operation::from(TGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "TGate",
        ];
    "TGate")]
#[test_case(
    Operation::from(SGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SGate",
        ];
    "SGate")]
#[test_case(
    Operation::from(PauliX::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "PauliX",
        ];
    "PauliX")]
#[test_case(
    Operation::from(PauliY::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "PauliY",
        ];
    "PauliY")]
#[test_case(
    Operation::from(PauliZ::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "PauliZ",
        ];
    "PauliZ")]
#[test_case(
    Operation::from(SqrtPauliX::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SqrtPauliX",
        ];
    "SqrtPauliX")]
#[test_case(
    Operation::from(InvSqrtPauliX::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "InvSqrtPauliX",
        ];
    "InvSqrtPauliX")]
#[test_case(
    Operation::from(Hadamard::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Hadamard",
        ];
    "Hadamard")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(0.0),
        CalculatorFloat::from(PI),
    )),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SingleQubitGate",
        ];
    "SingleQubitGate")]
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
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

/// Test remap_qubits() function for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(0)); "PauliX")]
#[test_case(Operation::from(PauliY::new(0)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(0)); "SGate")]
#[test_case(Operation::from(TGate::new(0)); "TGate")]
#[test_case(Operation::from(Hadamard::new(0)); "Hadamard")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    // test initial qubit
    let qubit: usize =
        usize::extract(operation.call_method0(py, "qubit").unwrap().as_ref(py)).unwrap();
    assert_eq!(qubit.clone(), 0);
    // remap qubits
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let result = operation
        .call_method1(py, "remap_qubits", (qubit_mapping,))
        .unwrap();
    // test re-mapped qubit
    let qubit_new: usize =
        usize::extract(result.call_method0(py, "qubit").unwrap().as_ref(py)).unwrap();
    assert_eq!(qubit_new.clone(), 1);
    // test that initial and rempapped qubits are different
    assert_ne!(qubit, qubit_new);
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(0)); "PauliX")]
#[test_case(Operation::from(PauliY::new(0)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(0)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(0)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(0)); "SGate")]
#[test_case(Operation::from(TGate::new(0)); "TGate")]
#[test_case(Operation::from(Hadamard::new(0)); "Hadamard")]
fn test_pyo3_remapqubits_error(input_operation: Operation) {
    // preparation
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    // remap qubits
    let qubit_mapping: HashMap<usize, usize> = HashMap::new();
    let result = operation.call_method1(py, "remap_qubits", (qubit_mapping,));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());
}

/// Test unitary_matrix() function for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(1.0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
fn test_pyo3_unitarymatrix(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let py_result = operation.call_method0(py, "unitary_matrix").unwrap();
    let result_matrix = py_result
        .cast_as::<PyArray2<Complex64>>(py)
        .unwrap()
        .to_owned_array();

    // compare to reference matrix obtained in Rust directly (without passing to Python)
    let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
    let rust_matrix: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
    let test_matrix: Array2<Complex64> = rust_matrix.unwrap();

    assert_eq!(result_matrix, test_matrix);
}

/// Test unitary_matrix() function for SingleQubitGate Operations for the error case
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from("PI"))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("PI"))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from("PI"))); "RotateY float")]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from("PI"),
        CalculatorFloat::from("0"),
        CalculatorFloat::from("PI / 4.0"),
        )
    ); "RotateAroundSphericalAxis")
]
fn test_pyo3_unitarymatrix_error(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let py_result = operation.call_method0(py, "unitary_matrix");
    let result_ref = py_result.as_ref();
    assert!(result_ref.is_err());
}

/// Test unitary_matrix() function for SingleQubitGate returning an error
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from("PI"),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "symbolic_parameter")
]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(2.0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "normalization")
]
fn test_pyo3_unitarymatrix_singlequbitgate(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let py_result = operation.call_method0(py, "unitary_matrix");
    let result_ref = py_result.as_ref();
    assert!(result_ref.is_err());
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
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

/// Test alpha_r obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_pyo3_alpha_r(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

    let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
    let alpha_r = gate.alpha_r();
    let alpha_r_param: CalculatorFloatWrapper =
        CalculatorFloatWrapper::extract(convert_cf_to_pyobject(py, alpha_r)).unwrap();
    let method_op = operation.call_method0(py, "alpha_r").unwrap();
    let comparison_alpha_r = bool::extract(
        method_op
            .as_ref(py)
            .call_method1("__eq__", (alpha_r_param,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_alpha_r);
}

/// Test alpha_i obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_pyo3_alpha_i(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

    let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
    let alpha_i = gate.alpha_i();
    let alpha_i_param: CalculatorFloatWrapper =
        CalculatorFloatWrapper::extract(convert_cf_to_pyobject(py, alpha_i)).unwrap();
    let method_op = operation.call_method0(py, "alpha_i").unwrap();
    let comparison_alpha_i = bool::extract(
        method_op
            .as_ref(py)
            .call_method1("__eq__", (alpha_i_param,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_alpha_i);
}

/// Test beta_r obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_pyo3_beta_r(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

    let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
    let beta_r = gate.beta_r();
    let beta_r_param: CalculatorFloatWrapper =
        CalculatorFloatWrapper::extract(convert_cf_to_pyobject(py, beta_r)).unwrap();
    let method_op = operation.call_method0(py, "beta_r").unwrap();
    let comparison_beta_r = bool::extract(
        method_op
            .as_ref(py)
            .call_method1("__eq__", (beta_r_param,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_beta_r);
}

/// Test beta_i obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_pyo3_beta_i(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

    let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
    let beta_i = gate.beta_i();
    let beta_i_param: CalculatorFloatWrapper =
        CalculatorFloatWrapper::extract(convert_cf_to_pyobject(py, beta_i)).unwrap();
    let method_op = operation.call_method0(py, "beta_i").unwrap();
    let comparison_beta_i = bool::extract(
        method_op
            .as_ref(py)
            .call_method1("__eq__", (beta_i_param,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_beta_i);
}

/// Test global_phase obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY float")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_pyo3_global_phase(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

    let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
    let global_phase = gate.global_phase();
    let global_phase_param: CalculatorFloatWrapper =
        CalculatorFloatWrapper::extract(convert_cf_to_pyobject(py, global_phase)).unwrap();
    let method_op = operation.call_method0(py, "global_phase").unwrap();
    let comparison_global_phase = bool::extract(
        method_op
            .as_ref(py)
            .call_method1("__eq__", (global_phase_param,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison_global_phase);
}

/// Test format and repr functions
#[test_case(
    "RotateX { qubit: 0, theta: Float(0.0) }",
    Operation::from(RotateX::new(0, CalculatorFloat::from(0)));
    "RotateX")]
#[test_case(
    "RotateY { qubit: 0, theta: Float(0.0) }",
    Operation::from(RotateY::new(0, CalculatorFloat::from(0)));
    "RotateY")]
#[test_case(
    "RotateZ { qubit: 0, theta: Float(0.0) }",
    Operation::from(RotateZ::new(0, CalculatorFloat::from(0)));
    "RotateZ")]
#[test_case(
    "PauliX { qubit: 0 }",
    Operation::from(PauliX::new(0));
    "PauliX")]
#[test_case(
    "PauliY { qubit: 0 }",
    Operation::from(PauliY::new(0));
    "PauliY")]
#[test_case(
    "PauliZ { qubit: 0 }",
    Operation::from(PauliZ::new(0));
    "PauliZ")]
#[test_case(
    "SqrtPauliX { qubit: 0 }",
    Operation::from(SqrtPauliX::new(0));
    "SqrtPauliX")]
#[test_case(
    "InvSqrtPauliX { qubit: 0 }",
    Operation::from(InvSqrtPauliX::new(0));
    "InvSqrtPauliX")]
#[test_case(
    "SGate { qubit: 0 }",
    Operation::from(SGate::new(0));
    "SGate")]
#[test_case(
    "TGate { qubit: 0 }",
    Operation::from(TGate::new(0));
    "TGate")]
#[test_case(
    "Hadamard { qubit: 0 }",
    Operation::from(Hadamard::new(0));
    "Hadamard")]
#[test_case(
    "RotateAroundSphericalAxis { qubit: 0, theta: Float(0.0), spherical_theta: Float(0.0), spherical_phi: Float(0.0) }",
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case(
    "SingleQubitGate { qubit: 0, alpha_r: Str(\"alpha_r\"), alpha_i: Str(\"alpha_i\"), beta_r: Str(\"beta_r\"), beta_i: Str(\"beta_i\"), global_phase: Str(\"global_phase\") }",
    Operation::from(
        SingleQubitGate::new(
            0,
            CalculatorFloat::from("alpha_r"),
            CalculatorFloat::from("alpha_i"),
            CalculatorFloat::from("beta_r"),
            CalculatorFloat::from("beta_i"),
            CalculatorFloat::from("global_phase"),  
            )
    ); "SingleQubitGate")
]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
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

/// Test substitute_parameters() function for gates having multiple parameters
#[test_case(
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from("param"),
            CalculatorFloat::from("param2"),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from("param"),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from("param2"),
        )
    ); "SingleQubitGate")
]
fn test_pyo3_substitute_parameters(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
    substitution_dict_py.insert("param", 1.0);
    substitution_dict_py.insert("param2", 0.0);
    let substitute_op = operation
        .call_method1(py, "substitute_parameters", (substitution_dict_py,))
        .unwrap();

    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("param", 1.0);
    substitution_dict.set_variable("param2", 0.0);
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

/// Test substitute_parameters() function for one parameter
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from("theta"))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("theta"))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from("theta"))); "RotateY")]
fn test_pyo3_substitute_params_rotate(input_operation: Operation) {
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
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from("test"))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("test"))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from("test"))); "RotateY")]
#[test_case(
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from("test"),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case(Operation::from(
    SingleQubitGate::new(
        0,
        CalculatorFloat::from("test"),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "SingleQubitGate")
]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    let substitution_dict: HashMap<&str, f64> = HashMap::new();
    let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
    let result_ref = result.as_ref();
    assert!(result_ref.is_err());
}

/// Test substitute parameters function for SingleQubitGate Operations where it has no effect
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
fn test_ineffective_substitute_parameters(input_operation: Operation) {
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
    let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
    substitution_dict_py.insert("theta", 0.0);
    let substitute_op = operation
        .call_method1(py, "substitute_parameters", (substitution_dict_py,))
        .unwrap();

    let comparison = bool::extract(
        substitute_op
            .as_ref(py)
            .call_method1("__eq__", (operation,))
            .unwrap(),
    )
    .unwrap();
    assert!(comparison);
}

#[test_case(
    Operation::from(RotateZ::new(1, CalculatorFloat::from(0.005))),
    Operation::from(RotateZ::new(1, CalculatorFloat::from(0.005 * 1.5))); "RotateZ")]
#[test_case(
    Operation::from(RotateY::new(1, CalculatorFloat::from(0.005))),
    Operation::from(RotateY::new(1, CalculatorFloat::from(0.005 * 1.5))); "RotateY")]
#[test_case(
    Operation::from(RotateX::new(1, CalculatorFloat::from(0.005))),
    Operation::from(RotateX::new(1, CalculatorFloat::from(0.005 * 1.5))); "RotateX")]
#[test_case(
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ),
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
fn test_pyo3_rotate_powercf(first_op: Operation, second_op: Operation) {
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
    Operation::from(RotateX::new(0, CalculatorFloat::from(0))),
    Operation::from(RotateX::new(1, CalculatorFloat::from(0))); "RotateX")]
#[test_case(
    Operation::from(RotateY::new(0, CalculatorFloat::from(0))),
    Operation::from(RotateY::new(1, CalculatorFloat::from(0))); "RotateY")]
#[test_case(
    Operation::from(RotateZ::new(0, CalculatorFloat::from(0))),
    Operation::from(RotateZ::new(1, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(
    Operation::from(PauliX::new(0)),
    Operation::from(PauliX::new(1)); "PauliX_0")]
#[test_case(
    Operation::from(PauliY::new(0)),
    Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(
    Operation::from(PauliZ::new(0)),
    Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(
    Operation::from(SqrtPauliX::new(0)),
    Operation::from(SqrtPauliX::new(1)); "SqrtPauliX")]
#[test_case(
    Operation::from(InvSqrtPauliX::new(0)),
    Operation::from(InvSqrtPauliX::new(1)); "InvSqrtPauliX")]
#[test_case(
    Operation::from(
        RotateAroundSphericalAxis::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ),
    Operation::from(
        RotateAroundSphericalAxis::new(
            1,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
        )
    ); "Rotation")]
#[test_case(
    Operation::from(SGate::new(0)),
    Operation::from(SGate::new(1)); "SGate")]
#[test_case(
    Operation::from(TGate::new(0)),
    Operation::from(TGate::new(1)); "TGate")]
#[test_case(
    Operation::from(Hadamard::new(0)),
    Operation::from(Hadamard::new(1)); "Hadamard")]
#[test_case(
    Operation::from(
        SingleQubitGate::new(
            0,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            )
    ),
    Operation::from(
        SingleQubitGate::new(
            1,
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            CalculatorFloat::from(0),
            )
    ); "SingleQubitGate")
]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
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
