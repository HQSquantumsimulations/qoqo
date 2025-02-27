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

use super::convert_cf_to_pyobject;

use std::collections::HashMap;

use pyo3::prelude::*;

use qoqo::{
    operations::{
        convert_operation_to_pyobject, ControlledControlledPauliZWrapper,
        ControlledControlledPhaseShiftWrapper, ControlledSWAPWrapper,
        PhaseShiftedControlledControlledPhaseWrapper, PhaseShiftedControlledControlledZWrapper,
        ToffoliWrapper,
    },
    CircuitWrapper,
};
use qoqo_calculator::CalculatorFloat;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use roqoqo::{operations::*, Circuit, RoqoqoError};

use test_case::test_case;

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
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

#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("phi"), CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledPhase")]
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
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "Toffoli",
        ],
    Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "ControlledSWAP",
        ],
    Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "PhaseShiftedControlledControlledZ",
        ],
    Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledZ")]
#[test_case(
    vec![
        "Operation",
        "GateOperation",
        "ThreeQubitGateOperation",
        "PhaseShiftedControlledControlledPhase",
        ],
    Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::PI, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
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

#[test_case("ControlledControlledPauliZ", Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case("ControlledControlledPhaseShift", Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case("Toffoli", Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case("ControlledSWAP", Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case("PhaseShiftedControlledControlledZ", Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledZ")]
#[test_case("PhaseShiftedControlledControlledPhase", Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("phi"), CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledPhase")]
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

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
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
        let target: usize = operation
            .call_method0(py, "target")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
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
        let target_new: usize = result
            .call_method0(py, "target")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(target_new.clone(), 0);

        // test that initial and rempapped qubits are different
        assert_ne!(control_0, control_0_new);
        assert_ne!(control_1, control_1_new);
        assert_ne!(target, target_new);
    })
}

#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
fn test_pyo3_remapqubits_cswap(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();

        // test initial qubits
        let control: usize = operation
            .call_method0(py, "control")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control.clone(), 0);
        let target_0: usize = operation
            .call_method0(py, "target_0")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(target_0.clone(), 1);
        let target_1: usize = operation
            .call_method0(py, "target_1")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(target_1.clone(), 2);

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
        let control_new: usize = result
            .call_method0(py, "control")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(control_new.clone(), 2);
        let target_0_new: usize = result
            .call_method0(py, "target_0")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(target_0_new.clone(), 3);
        let target_1_new: usize = result
            .call_method0(py, "target_1")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(target_1_new.clone(), 0);

        // test that initial and rempapped qubits are different
        assert_ne!(control, control_new);
        assert_ne!(target_0, target_0_new);
        assert_ne!(target_1, target_1_new);
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
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

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("theta"))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::from("theta"))); "PhaseShiftedControlledControlledPhase")]
fn test_pyo3_unitarymatrix_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone()).unwrap();
        let py_result = operation.call_method0(py, "unitary_matrix");
        assert!(py_result.is_err());
    })
}

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
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

#[test_case(
    "ControlledControlledPauliZ { control_0: 1, control_1: 0, target: 2 }",
    Operation::from(ControlledControlledPauliZ::new(1, 0, 2)); "ControlledControlledPauliZ")]
#[test_case(
    "ControlledControlledPhaseShift { control_0: 1, control_1: 0, target: 2, theta: Float(-1.0) }",
    Operation::from(ControlledControlledPhaseShift::new(1, 0, 2, CalculatorFloat::from(-1.0))); "ControlledControlledPhaseShift")]
#[test_case(
    "Toffoli { control_0: 1, control_1: 0, target: 2 }",
    Operation::from(Toffoli::new(1, 0, 2)); "Toffoli")]
#[test_case(
    "ControlledSWAP { control: 1, target_0: 0, target_1: 2 }",
    Operation::from(ControlledSWAP::new(1, 0, 2)); "ControlledSWAP")]
#[test_case(
    "PhaseShiftedControlledControlledZ { control_0: 0, control_1: 1, target: 2, phi: Float(1.5707963267948966) }",
    Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(
    "PhaseShiftedControlledControlledPhase { control_0: 0, control_1: 1, target: 2, theta: Float(1.5707963267948966), phi: Float(3.141592653589793) }",
    Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
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

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.2))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
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

#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)),
            Operation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("test"))),
            Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(1.0))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)),
            Operation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)),
            Operation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("test"))),
            Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from(1.0))); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("test"), CalculatorFloat::from("test"))),
            Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0))); "PhaseShiftedControlledControlledPhase")]
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

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("test"))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from("test"))); "PhaseShiftedControlledControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from("test"), CalculatorFloat::from("test"))); "PhaseShiftedControlledControlledPhase")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<String, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(1.0))),
            Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(1.0 * 1.5))); "ControlledControlledPhaseShift")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0))),
            Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from(1.0 * 1.5), CalculatorFloat::from(1.0))); "PhaseShiftedControlledControlledPhase")]
fn test_pyo3_powercf(first_op: Operation, second_op: Operation) {
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
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding
            .downcast::<ControlledControlledPauliZWrapper>()
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
            .extract::<ControlledControlledPauliZWrapper>()
            .unwrap();
        let binding = operation_type.call1((1, 2, 3)).unwrap();
        let new_op_diff = binding
            .downcast::<ControlledControlledPauliZWrapper>()
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

#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from(0.0))), (0, 1, 2, 0.0), "__eq__"; "ControlleControlledPhaseShift_eq")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(2, 1, 0, CalculatorFloat::from(0.0))), (0, 1, 2, 0.0), "__ne__"; "ControlleControlledPhaseShift_ne")]
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
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding
            .downcast::<ControlledControlledPhaseShiftWrapper>()
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
        let result = operation_type.call1((0, 1, 2, vec!["fails"]));
        assert!(result.is_err());

        let result = operation_type.call1((0, vec!["fails"], 2));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py
            .extract::<ControlledControlledPhaseShiftWrapper>()
            .unwrap();
        let binding = operation_type.call1((1, 2, 3, 0.0)).unwrap();
        let new_op_diff = binding
            .downcast::<ControlledControlledPhaseShiftWrapper>()
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

#[test_case(Operation::from(Toffoli::new(0, 1, 2)), (0, 1, 2), "__eq__"; "Toffoli_eq")]
#[test_case(Operation::from(Toffoli::new(2, 1, 0)), (0, 1, 2), "__ne__"; "Toffoli_ne")]
fn test_new_toffoli(input_operation: Operation, arguments: (u32, u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<ToffoliWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<ToffoliWrapper>().unwrap();
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
        let def_wrapper = operation_py.extract::<ToffoliWrapper>().unwrap();
        let binding = operation_type.call1((1, 2, 3)).unwrap();
        let new_op_diff = binding.downcast::<ToffoliWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<ToffoliWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ToffoliWrapper { internal: Toffoli { control_0: 1, control_1: 2, target: 3 } }"
        );
    })
}

#[test_case(Operation::from(ControlledSWAP::new(0, 1, 2)), (0, 1, 2), "__eq__"; "ControlledSWAP_eq")]
#[test_case(Operation::from(ControlledSWAP::new(2, 1, 0)), (0, 1, 2), "__ne__"; "ControlledSWAP_ne")]
fn test_new_controlledswap(input_operation: Operation, arguments: (u32, u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<ControlledSWAPWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<ControlledSWAPWrapper>().unwrap();
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
        let def_wrapper = operation_py.extract::<ControlledSWAPWrapper>().unwrap();
        let binding = operation_type.call1((1, 2, 3)).unwrap();
        let new_op_diff = binding.downcast::<ControlledSWAPWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<ControlledSWAPWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "ControlledSWAPWrapper { internal: ControlledSWAP { control: 1, target_0: 2, target_1: 3 } }"
        );
    })
}

#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::from(1.0))), (0, 1, 2, 1.0), "__eq__"; "PhaseShiftedControlledControlledZ_eq")]
#[test_case(Operation::from(PhaseShiftedControlledControlledZ::new(2, 1, 0, CalculatorFloat::from(1.0))), (0, 1, 2, 1.0), "__ne__"; "PhaseShiftedControlledControlledZ_ne")]
fn test_new_phaseshiftedccz(
    input_operation: Operation,
    arguments: (u32, u32, u32, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<PhaseShiftedControlledControlledZWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding
            .downcast::<PhaseShiftedControlledControlledZWrapper>()
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
            .extract::<PhaseShiftedControlledControlledZWrapper>()
            .unwrap();
        let binding = operation_type.call1((1, 2, 3, 1.0)).unwrap();
        let new_op_diff = binding
            .downcast::<PhaseShiftedControlledControlledZWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<PhaseShiftedControlledControlledZWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseShiftedControlledControlledZWrapper { internal: PhaseShiftedControlledControlledZ { control_0: 1, control_1: 2, target: 3, phi: Float(1.0) } }"
        );
    })
}

#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0))), (0, 1, 2, 1.0, 1.0), "__eq__"; "PhaseShiftedControlledControlledPhase_eq")]
#[test_case(Operation::from(PhaseShiftedControlledControlledPhase::new(2, 1, 0, CalculatorFloat::from(1.0), CalculatorFloat::from(1.0))), (0, 1, 2, 1.0, 1.0), "__ne__"; "PhaseShiftedControlledControlledPhase_ne")]
fn test_new_phaseshiftedccphase(
    input_operation: Operation,
    arguments: (u32, u32, u32, f64, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // Basic initialisation, no errors
        let operation_type = py.get_type::<PhaseShiftedControlledControlledPhaseWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding
            .downcast::<PhaseShiftedControlledControlledPhaseWrapper>()
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
            .extract::<PhaseShiftedControlledControlledPhaseWrapper>()
            .unwrap();
        let binding = operation_type.call1((1, 2, 3, 1.0, 1.0)).unwrap();
        let new_op_diff = binding
            .downcast::<PhaseShiftedControlledControlledPhaseWrapper>()
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<PhaseShiftedControlledControlledPhaseWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "PhaseShiftedControlledControlledPhaseWrapper { internal: PhaseShiftedControlledControlledPhase { control_0: 1, control_1: 2, target: 3, theta: Float(1.0), phi: Float(1.0) } }"
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
        circuit += ControlledPhaseShift::new(1, 2, CalculatorFloat::FRAC_PI_2);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPhaseShift::new(1, 2, -CalculatorFloat::FRAC_PI_2);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPhaseShift::new(0, 2, CalculatorFloat::FRAC_PI_2);

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
        circuit += ControlledPhaseShift::new(1, 2, CalculatorFloat::PI / 2.0);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPhaseShift::new(1, 2, -CalculatorFloat::PI / 2.0);
        circuit += CNOT::new(0, 1);
        circuit += ControlledPhaseShift::new(0, 2, CalculatorFloat::PI / 2.0);

        assert_eq!(result_circuit.internal, circuit);
    });
}

#[test]
fn test_circuit_pyo3_toffoli() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(Toffoli::new(0, 1, 2));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += Hadamard::new(2);
        circuit += CNOT::new(1, 2);
        circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += TGate::new(2);
        circuit += CNOT::new(1, 2);
        circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += TGate::new(1);
        circuit += TGate::new(2);
        circuit += Hadamard::new(2);
        circuit += CNOT::new(0, 1);
        circuit += TGate::new(0);
        circuit += RotateZ::new(1, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 1);

        assert_eq!(result_circuit.internal, circuit);
    });
}

#[test]
fn test_circuit_pyo3_controlledswap() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(ControlledSWAP::new(0, 1, 2));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += CNOT::new(2, 1);
        circuit += Hadamard::new(2);
        circuit += CNOT::new(1, 2);
        circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += TGate::new(2);
        circuit += CNOT::new(1, 2);
        circuit += RotateZ::new(2, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += TGate::new(1);
        circuit += TGate::new(2);
        circuit += Hadamard::new(2);
        circuit += CNOT::new(0, 1);
        circuit += TGate::new(0);
        circuit += RotateZ::new(1, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 1);
        circuit += CNOT::new(2, 1);

        assert_eq!(result_circuit.internal, circuit);
    });
}

#[test]
fn test_circuit_pyo3_phaseshiftedccz() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(PhaseShiftedControlledControlledZ::new(
            0,
            1,
            2,
            CalculatorFloat::from(1.0),
        ));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += PhaseShiftedControlledPhase::new(
            1,
            2,
            CalculatorFloat::FRAC_PI_2,
            CalculatorFloat::from(1.0),
        );
        circuit += CNOT::new(0, 1);
        circuit += PhaseShiftedControlledPhase::new(
            1,
            2,
            -CalculatorFloat::FRAC_PI_2,
            CalculatorFloat::from(1.0),
        );
        circuit += CNOT::new(0, 1);
        circuit += PhaseShiftedControlledPhase::new(
            0,
            2,
            CalculatorFloat::FRAC_PI_2,
            CalculatorFloat::from(1.0),
        );

        assert_eq!(result_circuit.internal, circuit);
    });
}

#[test]
fn test_circuit_pyo3_phaseshiftedccphase() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(PhaseShiftedControlledControlledPhase::new(
            0,
            1,
            2,
            CalculatorFloat::from(1.0),
            CalculatorFloat::from(1.0),
        ));
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let py_result = operation.call_method0(py, "circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract(py).unwrap();

        let mut circuit = Circuit::new();
        circuit += PhaseShiftedControlledPhase::new(
            1,
            2,
            CalculatorFloat::from(0.5),
            CalculatorFloat::from(1.0),
        );
        circuit += CNOT::new(0, 1);
        circuit += PhaseShiftedControlledPhase::new(
            1,
            2,
            -CalculatorFloat::from(0.5),
            CalculatorFloat::from(1.0),
        );
        circuit += CNOT::new(0, 1);
        circuit += PhaseShiftedControlledPhase::new(
            0,
            2,
            CalculatorFloat::from(0.5),
            CalculatorFloat::from(1.0),
        );

        assert_eq!(result_circuit.internal, circuit);
    });
}

/// Test json_schema function for all three qubit gate operations
#[cfg(feature = "json_schema")]
#[test_case(ThreeQubitGateOperation::from(ControlledControlledPauliZ::new(0, 1, 2)); "ControlleControlledPauliZ")]
#[test_case(ThreeQubitGateOperation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::from("test"))); "ControlledControlledPhaseShift")]
#[test_case(ThreeQubitGateOperation::from(Toffoli::new(0, 1, 2)); "Toffoli")]
#[test_case(ThreeQubitGateOperation::from(ControlledSWAP::new(0, 1, 2)); "ControlledSWAP")]
#[test_case(ThreeQubitGateOperation::from(PhaseShiftedControlledControlledZ::new(0, 1, 2, CalculatorFloat::FRAC_PI_2)); "PhaseShiftedControlledControlledZ")]
#[test_case(ThreeQubitGateOperation::from(PhaseShiftedControlledControlledPhase::new(0, 1, 2, CalculatorFloat::FRAC_PI_2, CalculatorFloat::PI)); "PhaseShiftedControlledControlledPhase")]
fn test_pyo3_json_schema(operation: ThreeQubitGateOperation) {
    let rust_schema = match operation {
        ThreeQubitGateOperation::ControlledControlledPauliZ(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(ControlledControlledPauliZ))
                .unwrap()
        }
        ThreeQubitGateOperation::ControlledControlledPhaseShift(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(ControlledControlledPhaseShift))
                .unwrap()
        }
        ThreeQubitGateOperation::Toffoli(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(Toffoli)).unwrap()
        }
        ThreeQubitGateOperation::ControlledSWAP(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(ControlledSWAP)).unwrap()
        }
        ThreeQubitGateOperation::PhaseShiftedControlledControlledZ(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(PhaseShiftedControlledControlledZ))
                .unwrap()
        }
        ThreeQubitGateOperation::PhaseShiftedControlledControlledPhase(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(
                PhaseShiftedControlledControlledPhase
            ))
            .unwrap()
        }
        _ => unreachable!(),
    };
    let og_op = operation.clone();
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
        match og_op {
            ThreeQubitGateOperation::ControlledControlledPauliZ(_)
            | ThreeQubitGateOperation::ControlledControlledPhaseShift(_)
            | ThreeQubitGateOperation::Toffoli(_) => {
                assert_eq!(minimum_supported_version_string, "1.3.0")
            }
            ThreeQubitGateOperation::ControlledSWAP(_)
            | ThreeQubitGateOperation::PhaseShiftedControlledControlledZ(_)
            | ThreeQubitGateOperation::PhaseShiftedControlledControlledPhase(_) => {
                assert_eq!(minimum_supported_version_string, "1.16.0")
            }
            _ => unreachable!(),
        };
    });
}
