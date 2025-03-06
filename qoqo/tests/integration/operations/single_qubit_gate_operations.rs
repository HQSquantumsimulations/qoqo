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
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::{
    GPi2Wrapper, GPiWrapper, HadamardWrapper, IdentityWrapper, InvSGateWrapper, InvSXGateWrapper,
    InvSqrtPauliXWrapper, InvSqrtPauliYWrapper, InvTGateWrapper, PauliXWrapper, PauliYWrapper,
    PauliZWrapper, PhaseShiftState0Wrapper, PhaseShiftState1Wrapper,
    RotateAroundSphericalAxisWrapper, RotateXWrapper, RotateXYWrapper, RotateYWrapper,
    RotateZWrapper, SGateWrapper, SXGateWrapper, SingleQubitGateWrapper, SqrtPauliXWrapper,
    SqrtPauliYWrapper, TGateWrapper,
};
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::Operation;
use roqoqo::operations::SingleQubitGateOperation;
use roqoqo::operations::*;
use roqoqo::RoqoqoError;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::HashMap;
use std::convert::TryInto;
use std::f64::consts::PI;
use test_case::test_case;

use super::convert_cf_to_pyobject;

/// Test new() function for PauliX
#[test_case(Operation::from(PauliX::new(1)), (1,), "__eq__"; "PauliX_eq")]
#[test_case(Operation::from(PauliX::new(1)), (0,), "__ne__"; "PauliX_ne")]
fn test_new_paulix(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<PauliXWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<PauliXWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<PauliXWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<PauliXWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PauliXWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PauliXWrapper { internal: PauliX { qubit: 2 } }"
        );
    })
}

/// Test new() function for PauliY
#[test_case(Operation::from(PauliY::new(1)), (1,), "__eq__"; "PauliY_eq")]
#[test_case(Operation::from(PauliY::new(1)), (0,), "__ne__"; "PauliY_ne")]
fn test_new_pauliy(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<PauliYWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<PauliYWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<PauliYWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<PauliYWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PauliYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PauliYWrapper { internal: PauliY { qubit: 2 } }"
        );
    })
}

/// Test new() function for PauliZ
#[test_case(Operation::from(PauliZ::new(1)), (1,), "__eq__"; "PauliZ_eq")]
#[test_case(Operation::from(PauliZ::new(1)), (0,), "__ne__"; "PauliZ_ne")]
fn test_new_pauliz(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<PauliZWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<PauliZWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<PauliZWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<PauliZWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PauliZWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PauliZWrapper { internal: PauliZ { qubit: 2 } }"
        );
    })
}

/// Test new() function for SqrtPauliX
#[test_case(Operation::from(SqrtPauliX::new(1)), (1,), "__eq__"; "SqrtPauliX_eq")]
#[test_case(Operation::from(SqrtPauliX::new(1)), (0,), "__ne__"; "SqrtPauliX_ne")]
fn test_new_sqrtpaulix(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SqrtPauliXWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<SqrtPauliXWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<SqrtPauliXWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<SqrtPauliXWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SqrtPauliXWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SqrtPauliXWrapper { internal: SqrtPauliX { qubit: 2 } }"
        );
    })
}

/// Test new() function for InvSqrtPauliX
#[test_case(Operation::from(InvSqrtPauliX::new(1)), (1,), "__eq__"; "InvSqrtPauliX_eq")]
#[test_case(Operation::from(InvSqrtPauliX::new(1)), (0,), "__ne__"; "InvSqrtPauliX_ne")]
fn test_new_invsqrtpaulix(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<InvSqrtPauliXWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<InvSqrtPauliXWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<InvSqrtPauliXWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<InvSqrtPauliXWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InvSqrtPauliXWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "InvSqrtPauliXWrapper { internal: InvSqrtPauliX { qubit: 2 } }"
        );
    })
}

/// Test new() function for SGate
#[test_case(Operation::from(SGate::new(1)), (1,), "__eq__"; "SGate_eq")]
#[test_case(Operation::from(SGate::new(1)), (0,), "__ne__"; "SGate_ne")]
fn test_new_sgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SGateWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<SGateWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<SGateWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<SGateWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SGateWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SGateWrapper { internal: SGate { qubit: 2 } }"
        );
    })
}

/// Test new() function for InvSGate
#[test_case(Operation::from(InvSGate::new(1)), (1,), "__eq__"; "InvSGate_eq")]
#[test_case(Operation::from(InvSGate::new(1)), (0,), "__ne__"; "InvSGate_ne")]
fn test_new_invsgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<InvSGateWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<InvSGateWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<InvSGateWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<InvSGateWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InvSGateWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "InvSGateWrapper { internal: InvSGate { qubit: 2 } }"
        );
    })
}

/// Test new() function for TGate
#[test_case(Operation::from(TGate::new(1)), (1,), "__eq__"; "TGate_eq")]
#[test_case(Operation::from(TGate::new(1)), (0,), "__ne__"; "TGate_ne")]
fn test_new_tgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<TGateWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<TGateWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<TGateWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<TGateWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<TGateWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "TGateWrapper { internal: TGate { qubit: 2 } }"
        );
    })
}

/// Test new() function for InvTGate
#[test_case(Operation::from(InvTGate::new(1)), (1,), "__eq__"; "InvTGate_eq")]
#[test_case(Operation::from(InvTGate::new(1)), (0,), "__ne__"; "InvTGate_ne")]
fn test_new_invtgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<InvTGateWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<InvTGateWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<InvTGateWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<InvTGateWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InvTGateWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "InvTGateWrapper { internal: InvTGate { qubit: 2 } }"
        );
    })
}

/// Test new() function for Hadamard
#[test_case(Operation::from(Hadamard::new(1)), (1,), "__eq__"; "Hadamard_eq")]
#[test_case(Operation::from(Hadamard::new(1)), (0,), "__ne__"; "Hadamard_ne")]
fn test_new_hadamard(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<HadamardWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<HadamardWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<HadamardWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<HadamardWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<HadamardWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "HadamardWrapper { internal: Hadamard { qubit: 2 } }"
        );
    })
}

/// Test new() function for RotateX
#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "RotateX_eq")]
#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "RotateX_ne")]
fn test_new_rotatex(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<RotateXWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<RotateXWrapper>().unwrap();
        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<RotateXWrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<RotateXWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<RotateXWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "RotateXWrapper { internal: RotateX { qubit: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for RotateY
#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "RotateY_eq")]
#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "RotateY_ne")]
fn test_new_rotatey(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<RotateYWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<RotateYWrapper>().unwrap();
        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<RotateYWrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<RotateYWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<RotateYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "RotateYWrapper { internal: RotateY { qubit: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for RotateZ
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "RotateZ_eq")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "RotateZ_ne")]
fn test_new_rotatez(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<RotateZWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<RotateZWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<RotateZWrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<RotateZWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<RotateZWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "RotateZWrapper { internal: RotateZ { qubit: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for PhaseShiftState0
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "PhaseShiftState0_eq")]
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "PhaseShiftState0_ne")]
fn test_new_phaseshiftstate0(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<PhaseShiftState0Wrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<PhaseShiftState0Wrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<PhaseShiftState0Wrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<PhaseShiftState0Wrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PhaseShiftState0Wrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseShiftState0Wrapper { internal: PhaseShiftState0 { qubit: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for PhaseShiftState1
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "PhaseShiftState1_eq")]
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "PhaseShiftState1_ne")]
fn test_new_phaseshiftstate1(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<PhaseShiftState1Wrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<PhaseShiftState1Wrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<PhaseShiftState1Wrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<PhaseShiftState1Wrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<PhaseShiftState1Wrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseShiftState1Wrapper { internal: PhaseShiftState1 { qubit: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for GPi
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "GPi_eq")]
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "GPi_ne")]
fn test_new_gpi(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<GPiWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<GPiWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<GPiWrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<GPiWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<GPiWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "GPiWrapper { internal: GPi { qubit: 2, theta: Float(0.0) } }"
        );
    })
}

/// Test new() function for GPi2
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::ZERO)), (1, 0.0,), "__eq__"; "GPi2_eq")]
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::ZERO)), (0, 0.0,), "__ne__"; "GPi2_ne")]
fn test_new_gpi2(input_operation: Operation, arguments: (u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<GPi2Wrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<GPi2Wrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<GPi2Wrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<GPi2Wrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<GPi2Wrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "GPi2Wrapper { internal: GPi2 { qubit: 2, theta: Float(0.0) } }"
        );
    })
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
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<RotateAroundSphericalAxisWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding
            .downcast::<RotateAroundSphericalAxisWrapper>()
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
        let result = operation_type.call1((0, vec!["fails"], 0.0, 0.0));
        assert!(result.is_err());
        let result = operation_type.call1((0, 0.0, vec!["fails"], 0.0));
        assert!(result.is_err());
        let result = operation_type.call1((0, 0.0, 0.0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<RotateAroundSphericalAxisWrapper>()
            .unwrap();
        let binding = operation_type.call1((2, 0.0, 0.0, 0.0)).unwrap();
        let new_op_diff = binding
            .downcast::<RotateAroundSphericalAxisWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<RotateAroundSphericalAxisWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "RotateAroundSphericalAxisWrapper { internal: RotateAroundSphericalAxis { qubit: 2, theta: Float(0.0), spherical_theta: Float(0.0), spherical_phi: Float(0.0) } }"
        );
    })
}

#[test_case(Operation::from(
    RotateXY::new(
        1,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        )
    ), (1, 0.0, 0.0,), "__eq__"; "rotation_eq")]
#[test_case(Operation::from(
    RotateXY::new(
        1,
        CalculatorFloat::ZERO,
        CalculatorFloat::ZERO,
        )
    ), (0, 0.0, 0.0,), "__ne__"; "rotation_ne")]
fn test_new_rotatexy(input_operation: Operation, arguments: (u32, f64, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<RotateXYWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<RotateXYWrapper>().unwrap();
        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"], 0.0));
        assert!(result.is_err());
        let result = operation_type.call1((0, vec!["fails"], 0.0));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<RotateXYWrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<RotateXYWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<RotateXYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "RotateXYWrapper { internal: RotateXY { qubit: 2, theta: Float(0.0), phi: Float(0.0) } }"
        );
    })
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
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<SingleQubitGateWrapper>();
        let operation_py = operation_type.call1(arguments).unwrap();
        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(
                    method,
                    (operation_py.downcast::<SingleQubitGateWrapper>().unwrap(),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((0, vec!["fails"], 0.0, 0.0, 0.0, 0.0));
        assert!(result.is_err());

        let result = operation_type.call1((0, 0.0, vec!["fails"], 0.0, 0.0, 0.0));
        assert!(result.is_err());

        let result = operation_type.call1((0, 0.0, 0.0, vec!["fails"], 0.0, 0.0));
        assert!(result.is_err());

        let result = operation_type.call1((0, 0.0, 0.0, 0.0, vec!["fails"], 0.0));
        assert!(result.is_err());

        let result = operation_type.call1((0, 0.0, 0.0, 0.0, 0.0, vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<SingleQubitGateWrapper>().unwrap();
        let binding = operation_type.call1((2, 0.0, 0.0, 0.0, 0.0, 0.0)).unwrap();
        let new_op_diff = binding.downcast::<SingleQubitGateWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SingleQubitGateWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SingleQubitGateWrapper { internal: SingleQubitGate { qubit: 2, alpha_r: Float(0.0), alpha_i: Float(0.0), beta_r: Float(0.0), beta_i: Float(0.0), global_phase: Float(0.0) } }"
        );
    })
}

/// Test new() function for Identity
#[test_case(Operation::from(Identity::new(1)), (1,), "__eq__"; "Identity_eq")]
#[test_case(Operation::from(Identity::new(1)), (0,), "__ne__"; "Identity_ne")]
fn test_new_identity(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<IdentityWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<IdentityWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<IdentityWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<IdentityWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<IdentityWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "IdentityWrapper { internal: Identity { qubit: 2 } }"
        );
    })
}

/// Test new() function for SqrtPauliY
#[test_case(Operation::from(SqrtPauliY::new(1)), (1,), "__eq__"; "SqrtPauliY_eq")]
#[test_case(Operation::from(SqrtPauliY::new(1)), (0,), "__ne__"; "SqrtPauliY_ne")]
fn test_new_sqrtpauliy(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SqrtPauliYWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<SqrtPauliYWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<SqrtPauliYWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<SqrtPauliYWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SqrtPauliYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SqrtPauliYWrapper { internal: SqrtPauliY { qubit: 2 } }"
        );
    })
}

/// Test new() function for InvSqrtPauliY
#[test_case(Operation::from(InvSqrtPauliY::new(1)), (1,), "__eq__"; "InvSqrtPauliY_eq")]
#[test_case(Operation::from(InvSqrtPauliY::new(1)), (0,), "__ne__"; "InvSqrtPauliY_ne")]
fn test_new_invsqrtpauliy(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<InvSqrtPauliYWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<InvSqrtPauliYWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<InvSqrtPauliYWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<InvSqrtPauliYWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InvSqrtPauliYWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "InvSqrtPauliYWrapper { internal: InvSqrtPauliY { qubit: 2 } }"
        );
    })
}

/// Test new() function for SXGate
#[test_case(Operation::from(SXGate::new(1)), (1,), "__eq__"; "SXGate_eq")]
#[test_case(Operation::from(SXGate::new(1)), (0,), "__ne__"; "SXGate_ne")]
fn test_new_sxgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SXGateWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<SXGateWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<SXGateWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<SXGateWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<SXGateWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SXGateWrapper { internal: SXGate { qubit: 2 } }"
        );
    })
}

/// Test new() function for InvSXGate
#[test_case(Operation::from(InvSXGate::new(1)), (1,), "__eq__"; "InvSXGate_eq")]
#[test_case(Operation::from(InvSXGate::new(1)), (0,), "__ne__"; "InvSXGate_ne")]
fn test_new_invsxgate(input_operation: Operation, arguments: (u32,), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<InvSXGateWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<InvSXGateWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<InvSXGateWrapper>().unwrap();
        let binding = operation_type.call1((2,)).unwrap();
        let new_op_diff = binding.downcast::<InvSXGateWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InvSXGateWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "InvSXGateWrapper { internal: InvSXGate { qubit: 2 } }"
        );
    })
}

/// Test is_parametrized() function for SingleQubitGate Operations
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("theta"))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from("theta"))); "RotateY")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from("theta"))); "RotateZ")]
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
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from("theta"))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from("theta"))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::from("theta"))); "GPi")]
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::from("theta"))); "GPi2")]
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

/// Test is_parametrized = false for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(Identity::new(1)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(3)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(3)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(100)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(100)); "InvSXGate")]
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
#[test_case(CalculatorFloat::from(0), Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(CalculatorFloat::from(0), Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(CalculatorFloat::from(0), Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(CalculatorFloat::from(0), Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
fn test_pyo3_theta(theta: CalculatorFloat, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let theta_op: CalculatorFloatWrapper = operation
            .call_method0(py, "theta")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let theta_param: CalculatorFloatWrapper =
            CalculatorFloatWrapper::extract_bound(&convert_cf_to_pyobject(py, theta)).unwrap();
        assert_eq!(theta_op.internal, theta_param.internal);
    })
}

/// Test qubit() function for SingleQubitGate Operations
#[test_case(0, Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
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
#[test_case(0, Operation::from(InvTGate::new(0)); "InvTGate")]
#[test_case(0, Operation::from(SGate::new(0)); "SGate")]
#[test_case(0, Operation::from(InvSGate::new(0)); "InvSGate")]
#[test_case(0, Operation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(1, Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(1, Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(1, Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(1, Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case(1, Operation::from(Identity::new(1)); "Identity")]
#[test_case(3, Operation::from(SqrtPauliY::new(3)); "SqrtPauliY")]
#[test_case(3, Operation::from(InvSqrtPauliY::new(3)); "InvSqrtPauliY")]
#[test_case(0, Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(0, Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_qubit(qubit: usize, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let qubit_op: usize = operation
            .call_method0(py, "qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(qubit_op, qubit);
    })
}

/// Test RotateX hqslang() function for SingleQubitGate Operations
#[test_case("RotateXY", Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
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
#[test_case("InvSGate", Operation::from(InvSGate::new(0)); "InvSGate")]
#[test_case("TGate", Operation::from(TGate::new(0)); "TGate")]
#[test_case("InvTGate", Operation::from(InvTGate::new(0)); "InvTGate")]
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
#[test_case("PhaseShiftState0", Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case("PhaseShiftState1", Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case("GPi", Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case("GPi2", Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case("Identity", Operation::from(Identity::new(1)); "Identity")]
#[test_case("SqrtPauliY", Operation::from(SqrtPauliY::new(3)); "SqrtPauliY")]
#[test_case("InvSqrtPauliY", Operation::from(InvSqrtPauliY::new(3)); "InvSqrtPauliY")]
#[test_case("SXGate", Operation::from(SXGate::new(0)); "SXGate")]
#[test_case("InvSXGate", Operation::from(InvSXGate::new(0)); "InvSXGate")]
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

/// Test tags() function for SingleQubitGate Operations
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
    Operation::from(InvTGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "InvTGate",
        ];
    "InvTGate")]
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
    Operation::from(InvSGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "InvSGate",
        ];
    "InvSGate")]
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
#[test_case(
    Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "PhaseShiftState0",
        ];
    "PhaseShiftState0")]
#[test_case(
    Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "PhaseShiftState1",
        ];
    "PhaseShiftState1")]
#[test_case(
    Operation::from(GPi::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "GPi",
        ];
    "GPi")]
#[test_case(
    Operation::from(GPi2::new(0, CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Rotation",
        "GPi2",
        ];
    "GPi2")]
#[test_case(
    Operation::from(Identity::new(1)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "Identity",
        ];
    "Identity")]
#[test_case(
    Operation::from(SqrtPauliY::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SqrtPauliY",
        ];
    "SqrtPauliY")]
#[test_case(
    Operation::from(InvSqrtPauliY::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "InvSqrtPauliY",
        ];
    "InvSqrtPauliY")]
#[test_case(
    Operation::from(SXGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "SXGate",
        ];
    "SXGate")]
#[test_case(
    Operation::from(InvSXGate::new(0)),
    vec![
        "Operation",
        "GateOperation",
        "SingleQubitGateOperation",
        "InvSXGate",
        ];
    "InvSXGate")]
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
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

/// Test remap_qubits() function for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(0)); "InvSGate")]
#[test_case(Operation::from(TGate::new(0)); "TGate")]
#[test_case(Operation::from(InvTGate::new(0)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial qubit
        let qubit: usize = operation
            .call_method0(py, "qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(qubit.clone(), 0);
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 1);
        qubit_mapping.insert(1, 0);
        let result = operation
            .call_method1(py, "remap_qubits", (qubit_mapping,))
            .unwrap();
        // test re-mapped qubit
        let qubit_new: usize = result
            .call_method0(py, "qubit")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(qubit_new.clone(), 1);
        // test that initial and rempapped qubits are different
        assert_ne!(qubit, qubit_new);
    })
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(0)); "InvSGate")]
#[test_case(Operation::from(TGate::new(0)); "TGate")]
#[test_case(Operation::from(InvTGate::new(0)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(0)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
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

/// Test unitary_matrix() function for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
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
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(-1.0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(2.3))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(2.3))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(2.3))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
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
        let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
        let rust_matrix: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
        let test_matrix: Array2<Complex64> = rust_matrix.unwrap();

        assert_eq!(result_matrix, test_matrix);
    })
}

/// Test unitary_matrix() function for SingleQubitGate Operations for the error case
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from("PI"))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("PI"))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from("PI"))); "RotateY")]
#[test_case(Operation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from("PI"),
        CalculatorFloat::from("0"),
        CalculatorFloat::from("PI / 4.0"),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from("PI"))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from("PI"))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from("PI"))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from("PI"))); "GPi2")]
fn test_pyo3_unitarymatrix_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix");
        assert!(py_result.is_err());
    })
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
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix");
        assert!(py_result.is_err());
    })
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0.0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0.0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let copy_op = operation.call_method0(py, "__copy__").unwrap();
        let deepcopy_op = operation.call_method1(py, "__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = bool::extract_bound(
            &copy_op
                .bind(py)
                .call_method1("__eq__", (copy_deepcopy_param.clone_ref(py),))
                .unwrap(),
        )
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

/// Test alpha_r obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0.0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0.0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_alpha_r(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

        let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
        let alpha_r = gate.alpha_r();
        let alpha_r_param: CalculatorFloatWrapper =
            CalculatorFloatWrapper::extract_bound(&convert_cf_to_pyobject(py, alpha_r)).unwrap();
        let method_op = operation.call_method0(py, "alpha_r").unwrap();
        let comparison_alpha_r = bool::extract_bound(
            &method_op
                .bind(py)
                .call_method1("__eq__", (alpha_r_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_alpha_r);
    })
}

/// Test alpha_i obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0.0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0.0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_alpha_i(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

        let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
        let alpha_i = gate.alpha_i();
        let alpha_i_param: CalculatorFloatWrapper =
            CalculatorFloatWrapper::extract_bound(&convert_cf_to_pyobject(py, alpha_i)).unwrap();
        let method_op = operation.call_method0(py, "alpha_i").unwrap();
        let comparison_alpha_i = bool::extract_bound(
            &method_op
                .bind(py)
                .call_method1("__eq__", (alpha_i_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_alpha_i);
    })
}

/// Test beta_r obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0.0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0.0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_beta_r(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

        let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
        let beta_r = gate.beta_r();
        let beta_r_param: CalculatorFloatWrapper =
            CalculatorFloatWrapper::extract_bound(&convert_cf_to_pyobject(py, beta_r)).unwrap();
        let method_op = operation.call_method0(py, "beta_r").unwrap();
        let comparison_beta_r = bool::extract_bound(
            &method_op
                .bind(py)
                .call_method1("__eq__", (beta_r_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_beta_r);
    })
}

/// Test beta_i obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0.0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0.0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_beta_i(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

        let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
        let beta_i = gate.beta_i();
        let beta_i_param: CalculatorFloatWrapper =
            CalculatorFloatWrapper::extract_bound(&convert_cf_to_pyobject(py, beta_i)).unwrap();
        let method_op = operation.call_method0(py, "beta_i").unwrap();
        let comparison_beta_i = bool::extract_bound(
            &method_op
                .bind(py)
                .call_method1("__eq__", (beta_i_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_beta_i);
    })
}

/// Test global_phase obtained via the python interface for SingleQubitGate Operations
#[test_case(Operation::from(
    RotateXY::new(
        0,
        CalculatorFloat::from(0),
        CalculatorFloat::from(0),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
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
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0.0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0.0))); "GPi2")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_global_phase(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();

        let gate: SingleQubitGateOperation = input_operation.try_into().unwrap();
        let global_phase = gate.global_phase();
        let global_phase_param: CalculatorFloatWrapper =
            CalculatorFloatWrapper::extract_bound(&convert_cf_to_pyobject(py, global_phase))
                .unwrap();
        let method_op = operation.call_method0(py, "global_phase").unwrap();
        let comparison_global_phase = bool::extract_bound(
            &method_op
                .bind(py)
                .call_method1("__eq__", (global_phase_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_global_phase);
    })
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
    "InvSGate { qubit: 0 }",
    Operation::from(InvSGate::new(0));
    "InvSGate")]
#[test_case(
    "TGate { qubit: 0 }",
    Operation::from(TGate::new(0));
    "TGate")]
#[test_case(
    "InvTGate { qubit: 0 }",
    Operation::from(InvTGate::new(0));
    "InvTGate")]
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
#[test_case(
    "PhaseShiftState0 { qubit: 0, theta: Float(0.0) }",
    Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0)));
    "PhaseShiftState0")]
#[test_case(
    "PhaseShiftState1 { qubit: 0, theta: Float(0.0) }",
    Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0)));
    "PhaseShiftState1")]
#[test_case(
    "GPi { qubit: 0, theta: Float(0.0) }",
    Operation::from(GPi::new(0, CalculatorFloat::from(0)));
    "GPi")]
#[test_case(
    "GPi2 { qubit: 0, theta: Float(0.0) }",
    Operation::from(GPi2::new(0, CalculatorFloat::from(0)));
    "GPi2")]
#[test_case(
    "Identity { qubit: 0 }",
    Operation::from(Identity::new(0));
    "Identity")]
#[test_case(
    "SqrtPauliY { qubit: 0 }",
    Operation::from(SqrtPauliY::new(0));
    "SqrtPauliY")]
#[test_case(
    "InvSqrtPauliY { qubit: 0 }",
    Operation::from(InvSqrtPauliY::new(0));
    "InvSqrtPauliY")]
#[test_case(
    "SXGate { qubit: 0 }",
    Operation::from(SXGate::new(0));
    "SXGate")]
#[test_case(
    "InvSXGate { qubit: 0 }",
    Operation::from(InvSXGate::new(0));
    "InvSXGate")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = to_format.bind(py).extract().unwrap();
        assert_eq!(format_op, format_repr);
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = to_repr.bind(py).extract().unwrap();
        assert_eq!(repr_op, format_repr);
    })
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
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("param".to_owned(), 1.0);
        substitution_dict_py.insert("param2".to_owned(), 0.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("param", 1.0);
        substitution_dict.set_variable("param2", 0.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

/// Test substitute_parameters() function for one parameter
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from("theta"))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from("theta"))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from("theta"))); "RotateY")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from("theta"))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from("theta"))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from("theta"))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from("theta"))); "GPi2")]
fn test_pyo3_substitute_params_rotate(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
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
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from("theta"))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from("theta"))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from("theta"))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from("theta"))); "GPi2")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<String, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

/// Test substitute parameters function for SingleQubitGate Operations where it has no effect
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(0))); "RotateY")]
#[test_case(Operation::from(Identity::new(0)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_ineffective_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 0.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
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
    Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0.005))),
    Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0.005 * 1.5))); "PhaseShiftState0")]
#[test_case(
    Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0.005))),
    Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0.005 * 1.5))); "PhaseShiftState1")]
#[test_case(
    Operation::from(GPi::new(1, CalculatorFloat::from(0.005))),
    Operation::from(GPi::new(1, CalculatorFloat::from(0.005 * 1.5))); "GPi")]
#[test_case(
    Operation::from(GPi2::new(1, CalculatorFloat::from(0.005))),
    Operation::from(GPi2::new(1, CalculatorFloat::from(0.005 * 1.5))); "GPi2")]
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
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op).unwrap();

        let power = convert_cf_to_pyobject(py, CalculatorFloat::from(1.5));
        let comparison_op = convert_operation_to_pyobject(second_op).unwrap();

        let remapped_op = operation.call_method1(py, "powercf", (power,)).unwrap();
        let comparison = remapped_op
            .call_method1(py, "__eq__", (comparison_op,))
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap();
        assert!(comparison);
    })
}

/// Test that multiplication function can be called in python for SingleQubitGates
#[test_case(Operation::from(
    RotateXY::new(
        1,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(1)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(1)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(1)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::from(0))); "RotateY")]
#[test_case(Operation::from(Identity::new(1)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(1)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(1)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(1)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(1)); "InvSXGate")]
fn test_pyo3_mul(gate1: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let gate2: Operation = Operation::from(SingleQubitGate::new(
            1,
            1.0.into(),
            0.0.into(),
            0.0.into(),
            0.0.into(),
            0.0.into(),
        ));

        let operation = convert_operation_to_pyobject(gate1.clone()).unwrap();
        let operation2 = convert_operation_to_pyobject(gate2).unwrap();
        let result = operation.call_method1(py, "mul", (operation2,));

        assert!(result.is_ok());
    })
}

#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(1)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(1)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(1)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::from(0))); "RotateY")]
#[test_case(Operation::from(Identity::new(1)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(1)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(1)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(1)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(1)); "InvSXGate")]
fn test_pyo3_mul_error1(gate1: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let alpha_r = CalculatorFloat::from(PI / 2.0);
        let input_cf = convert_cf_to_pyobject(py, alpha_r);

        let operation = convert_operation_to_pyobject(gate1.clone()).unwrap();
        let result = operation.call_method1(py, "mul", (input_cf,));

        assert!(result.is_err());
    })
}

#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(1)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(1)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(1)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::from(0))); "RotateY")]
#[test_case(Operation::from(Identity::new(1)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(0)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_mul_error2(gate1: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let gate2 = Operation::from(CNOT::new(0, 1));

        let operation = convert_operation_to_pyobject(gate1.clone()).unwrap();
        let operation2 = convert_operation_to_pyobject(gate2).unwrap();
        let result = operation.call_method1(py, "mul", (operation2,));

        assert!(result.is_err());
    })
}

#[test_case(Operation::from(PauliX::new(1)); "PauliX")]
#[test_case(Operation::from(PauliY::new(1)); "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(Operation::from(SqrtPauliX::new(1)); "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(1)); "InvSqrtPauliX")]
#[test_case(Operation::from(SGate::new(1)); "SGate")]
#[test_case(Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(Operation::from(TGate::new(1)); "TGate")]
#[test_case(Operation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(Operation::from(Hadamard::new(1)); "Hadamard")]
#[test_case(Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(0))); "RotateZ")]
#[test_case(Operation::from(RotateX::new(1, CalculatorFloat::from(0))); "RotateX")]
#[test_case(Operation::from(RotateY::new(1, CalculatorFloat::from(0))); "RotateY")]
#[test_case(Operation::from(Identity::new(1)); "Identity")]
#[test_case(Operation::from(SqrtPauliY::new(1)); "SqrtPauliY")]
#[test_case(Operation::from(InvSqrtPauliY::new(1)); "InvSqrtPauliY")]
#[test_case(Operation::from(SXGate::new(1)); "SXGate")]
#[test_case(Operation::from(InvSXGate::new(1)); "InvSXGate")]
fn test_pyo3_mul_error3(gate1: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let gate2: Operation = Operation::from(SingleQubitGate::new(
            0,
            1.0.into(),
            0.0.into(),
            0.0.into(),
            0.0.into(),
            0.0.into(),
        ));

        let operation = convert_operation_to_pyobject(gate1.clone()).unwrap();
        let operation2 = convert_operation_to_pyobject(gate2).unwrap();
        let result = operation.call_method1(py, "mul", (operation2,));

        assert!(result.is_err());
    })
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
    Operation::from(InvSGate::new(0)),
    Operation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(
    Operation::from(TGate::new(0)),
    Operation::from(TGate::new(1)); "TGate")]
#[test_case(
    Operation::from(InvTGate::new(0)),
    Operation::from(InvTGate::new(1)); "InvTGate")]
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
#[test_case(
    Operation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0))),
    Operation::from(PhaseShiftState0::new(1, CalculatorFloat::from(0))); "PhaseShiftState0")]
#[test_case(
    Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0))),
    Operation::from(PhaseShiftState1::new(1, CalculatorFloat::from(0))); "PhaseShiftState1")]
#[test_case(
    Operation::from(GPi::new(0, CalculatorFloat::from(0))),
    Operation::from(GPi::new(1, CalculatorFloat::from(0))); "GPi")]
#[test_case(
    Operation::from(GPi2::new(0, CalculatorFloat::from(0))),
    Operation::from(GPi2::new(1, CalculatorFloat::from(0))); "GPi2")]
#[test_case(
    Operation::from(Identity::new(0)),
    Operation::from(Identity::new(1)); "Identity")]
#[test_case(
    Operation::from(SqrtPauliY::new(0)),
    Operation::from(SqrtPauliY::new(1)); "SqrtPauliY")]
#[test_case(
    Operation::from(InvSqrtPauliY::new(0)),
    Operation::from(InvSqrtPauliY::new(1)); "InvSqrtPauliY")]
#[test_case(
    Operation::from(SXGate::new(0)),
    Operation::from(SXGate::new(1)); "SXGate")]
#[test_case(
    Operation::from(InvSXGate::new(0)),
    Operation::from(InvSXGate::new(1)); "InvSXGate")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(definition_1).unwrap();
        let operation_two = convert_operation_to_pyobject(definition_2).unwrap();

        let comparison = bool::extract_bound(
            &operation_one
                .bind(py)
                .call_method1("__eq__", (operation_two.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract_bound(
            &operation_one
                .bind(py)
                .call_method1("__ne__", (operation_two.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1(py, "__eq__", (vec!["fails"],));
        assert!(comparison.is_err());

        let comparison = operation_one.call_method1(py, "__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}

/// Test json_schema function for all single qubit gate operations
#[cfg(feature = "json_schema")]
#[test_case(SingleQubitGateOperation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX")]
#[test_case(SingleQubitGateOperation::from(RotateY::new(0, CalculatorFloat::from(PI))); "RotateY")]
#[test_case(SingleQubitGateOperation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ")]
#[test_case(SingleQubitGateOperation::from(
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
#[test_case(SingleQubitGateOperation::from(
    RotateAroundSphericalAxis::new(
        0,
        CalculatorFloat::from(PI),
        CalculatorFloat::from(0),
        CalculatorFloat::from(PI / 4.0),
        )
    ); "RotateAroundSphericalAxis")
]
#[test_case(SingleQubitGateOperation::from(
    RotateXY::new(
        1,
        CalculatorFloat::from("theta"),
        CalculatorFloat::from("phi"),
        )
    ); "RotateXY")
]
#[test_case(SingleQubitGateOperation::from(PauliX::new(1)); "PauliX")]
#[test_case(SingleQubitGateOperation::from(PauliY::new(1)); "PauliY")]
#[test_case(SingleQubitGateOperation::from(PauliZ::new(1)); "PauliZ")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliX::new(100)); "SqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliX::new(100)); "InvSqrtPauliX")]
#[test_case(SingleQubitGateOperation::from(SGate::new(1)); "SGate")]
#[test_case(SingleQubitGateOperation::from(InvSGate::new(1)); "InvSGate")]
#[test_case(SingleQubitGateOperation::from(TGate::new(1)); "TGate")]
#[test_case(SingleQubitGateOperation::from(InvTGate::new(1)); "InvTGate")]
#[test_case(SingleQubitGateOperation::from(Hadamard::new(3)); "Hadamard")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState0::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState0")]
#[test_case(SingleQubitGateOperation::from(PhaseShiftState1::new(0, CalculatorFloat::from(0.0))); "PhaseShiftState1")]
#[test_case(SingleQubitGateOperation::from(GPi::new(0, CalculatorFloat::from(0.0))); "GPi")]
#[test_case(SingleQubitGateOperation::from(GPi2::new(0, CalculatorFloat::from(0.0))); "GPi2")]
#[test_case(SingleQubitGateOperation::from(Identity::new(1)); "Identity")]
#[test_case(SingleQubitGateOperation::from(SqrtPauliY::new(0)); "SqrtPauliY")]
#[test_case(SingleQubitGateOperation::from(InvSqrtPauliY::new(0)); "InvSqrtPauliY")]
#[test_case(SingleQubitGateOperation::from(SXGate::new(0)); "SXGate")]
#[test_case(SingleQubitGateOperation::from(InvSXGate::new(0)); "InvSXGate")]
fn test_pyo3_json_schema(operation: SingleQubitGateOperation) {
    let rust_schema = match operation {
        SingleQubitGateOperation::SingleQubitGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(SingleQubitGate)).unwrap()
        }
        SingleQubitGateOperation::RotateZ(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(RotateZ)).unwrap()
        }
        SingleQubitGateOperation::RotateX(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(RotateX)).unwrap()
        }
        SingleQubitGateOperation::RotateY(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(RotateY)).unwrap()
        }
        SingleQubitGateOperation::PauliX(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PauliX)).unwrap()
        }
        SingleQubitGateOperation::PauliY(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PauliY)).unwrap()
        }
        SingleQubitGateOperation::PauliZ(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PauliZ)).unwrap()
        }
        SingleQubitGateOperation::SqrtPauliX(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(SqrtPauliX)).unwrap()
        }
        SingleQubitGateOperation::InvSqrtPauliX(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(InvSqrtPauliX)).unwrap()
        }
        SingleQubitGateOperation::Hadamard(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(Hadamard)).unwrap()
        }
        SingleQubitGateOperation::SGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(SGate)).unwrap()
        }
        SingleQubitGateOperation::InvSGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(InvSGate)).unwrap()
        }
        SingleQubitGateOperation::TGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(TGate)).unwrap()
        }
        SingleQubitGateOperation::InvTGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(InvTGate)).unwrap()
        }
        SingleQubitGateOperation::PhaseShiftState1(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PhaseShiftState1)).unwrap()
        }
        SingleQubitGateOperation::PhaseShiftState0(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PhaseShiftState0)).unwrap()
        }
        SingleQubitGateOperation::RotateAroundSphericalAxis(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(RotateAroundSphericalAxis)).unwrap()
        }
        SingleQubitGateOperation::RotateXY(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(RotateXY)).unwrap()
        }
        SingleQubitGateOperation::GPi(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(GPi)).unwrap()
        }
        SingleQubitGateOperation::GPi2(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(GPi2)).unwrap()
        }
        SingleQubitGateOperation::Identity(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(Identity)).unwrap()
        }
        SingleQubitGateOperation::SqrtPauliY(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(SqrtPauliY)).unwrap()
        }
        SingleQubitGateOperation::InvSqrtPauliY(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(InvSqrtPauliY)).unwrap()
        }
        SingleQubitGateOperation::SXGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(SXGate)).unwrap()
        }
        SingleQubitGateOperation::InvSXGate(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(InvSXGate)).unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = match operation {
            SingleQubitGateOperation::GPi(_) => "1.4.0".to_string(),
            SingleQubitGateOperation::GPi2(_) => "1.4.0".to_string(),
            SingleQubitGateOperation::Identity(_) => "1.7.0".to_string(),
            SingleQubitGateOperation::SqrtPauliY(_) => "1.15.0".to_string(),
            SingleQubitGateOperation::InvSqrtPauliY(_) => "1.15.0".to_string(),
            SingleQubitGateOperation::InvSGate(_)
            | SingleQubitGateOperation::InvTGate(_)
            | SingleQubitGateOperation::SXGate(_)
            | SingleQubitGateOperation::InvSXGate(_) => "1.16.0".to_string(),
            _ => "1.0.0".to_string(),
        };
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
        assert_eq!(minimum_supported_version_string, minimum_version);
    });
}
