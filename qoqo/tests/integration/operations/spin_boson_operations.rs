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

use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::{
    CZQubitResonatorWrapper, JaynesCummingsWrapper, LongitudinalCouplingWrapper,
    QuantumRabiWrapper, SingleExcitationLoadWrapper, SingleExcitationStoreWrapper,
};
use qoqo_calculator::{Calculator, CalculatorFloat};
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

use super::convert_cf_to_pyobject;

/// Test new() function for QuantumRabi
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())), (1, 0, 1.0,), "__eq__"; "QuantumRabi_eq")]
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())), (0, 1, 1.0,), "__ne__"; "QuantumRabi_ne")]
fn test_new_quantum_rabi(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<QuantumRabiWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<QuantumRabiWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<QuantumRabiWrapper>().unwrap();
        let binding = operation_type.call1((2, 3, 1.0)).unwrap();
        let new_op_diff = binding.downcast::<QuantumRabiWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<QuantumRabiWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "QuantumRabiWrapper { internal: QuantumRabi { qubit: 2, mode: 3, theta: Float(1.0) } }"
        );

        let comparison_copy = bool::extract_bound(
            &operation
                .call_method0(py, "theta")
                .unwrap()
                .bind(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(1.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test new() function for LongitudinalCoupling
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())), (1, 0, 1.0,), "__eq__"; "LongitudinalCoupling_eq")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())), (0, 1, 1.0,), "__ne__"; "LongitudinalCoupling_ne")]
fn test_new_longitudinal_coupling(
    input_operation: Operation,
    arguments: (u32, u32, f64),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<LongitudinalCouplingWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<LongitudinalCouplingWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py
            .extract::<LongitudinalCouplingWrapper>()
            .unwrap();
        let binding = operation_type.call1((2, 3, 1.0)).unwrap();
        let new_op_diff = binding.downcast::<LongitudinalCouplingWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<LongitudinalCouplingWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "LongitudinalCouplingWrapper { internal: LongitudinalCoupling { qubit: 2, mode: 3, theta: Float(1.0) } }"
        );

        let comparison_copy = bool::extract_bound(
            &operation
                .call_method0(py, "theta")
                .unwrap()
                .bind(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(1.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test new() function for JaynesCummings
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())), (1, 0, 1.0,), "__eq__"; "JaynesCummings_eq")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())), (0, 1, 1.0,), "__ne__"; "JaynesCummings_ne")]
fn test_new_jaynes_cummings(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<JaynesCummingsWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<JaynesCummingsWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<JaynesCummingsWrapper>().unwrap();
        let binding = operation_type.call1((2, 3, 1.0)).unwrap();
        let new_op_diff = binding.downcast::<JaynesCummingsWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<JaynesCummingsWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "JaynesCummingsWrapper { internal: JaynesCummings { qubit: 2, mode: 3, theta: Float(1.0) } }"
        );

        let comparison_copy = bool::extract_bound(
            &operation
                .call_method0(py, "theta")
                .unwrap()
                .bind(py)
                .call_method1(
                    "__eq__",
                    (convert_cf_to_pyobject(py, CalculatorFloat::Float(1.0)),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test new() function for SingleExcitationLoad
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)), (1, 0), "__eq__"; "SingleExcitationLoad_eq")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)), (0, 1), "__ne__"; "SingleExcitationLoad_ne")]
fn test_new_single_excitation_load(
    input_operation: Operation,
    arguments: (u32, u32),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SingleExcitationLoadWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<SingleExcitationLoadWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py
            .extract::<SingleExcitationLoadWrapper>()
            .unwrap();
        let binding = operation_type.call1((2, 3)).unwrap();
        let new_op_diff = binding.downcast::<SingleExcitationLoadWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<SingleExcitationLoadWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SingleExcitationLoadWrapper { internal: SingleExcitationLoad { qubit: 2, mode: 3 } }"
        );
    })
}

/// Test new() function for SingleExcitationStore
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)), (1, 0), "__eq__"; "SingleExcitationStore_eq")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)), (0, 1), "__ne__"; "SingleExcitationStore_ne")]
fn test_new_single_excitation_store(
    input_operation: Operation,
    arguments: (u32, u32),
    method: &str,
) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<SingleExcitationStoreWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<SingleExcitationStoreWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py
            .extract::<SingleExcitationStoreWrapper>()
            .unwrap();
        let binding = operation_type.call1((2, 3)).unwrap();
        let new_op_diff = binding.downcast::<SingleExcitationStoreWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff
            .extract::<SingleExcitationStoreWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "SingleExcitationStoreWrapper { internal: SingleExcitationStore { qubit: 2, mode: 3 } }"
        );
    })
}

/// Test new() function for CZQubitResonator
#[test_case(Operation::from(CZQubitResonator::new(1, 0)), (1, 0), "__eq__"; "CZQubitResonator_eq")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)), (0, 1), "__ne__"; "CZQubitResonator_ne")]
fn test_new_cz_qubit_resonator(input_operation: Operation, arguments: (u32, u32), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<CZQubitResonatorWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<CZQubitResonatorWrapper>().unwrap();

        let comparison = bool::extract_bound(
            &operation
                .bind(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<CZQubitResonatorWrapper>().unwrap();
        let binding = operation_type.call1((2, 3)).unwrap();
        let new_op_diff = binding.downcast::<CZQubitResonatorWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<CZQubitResonatorWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "CZQubitResonatorWrapper { internal: CZQubitResonator { qubit: 2, mode: 3 } }"
        );
    })
}

/// Test is_parametrized() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from("theta"))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from("theta"))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from("theta"))); "JaynesCummings")]
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

/// Test is_parametrized = false for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
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

/// Test mode() function for SingleMode Operations
#[test_case(0, Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(0, Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(0, Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(0, Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(0, Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(0, Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
fn test_pyo3_mode(mode: usize, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let mode_op: usize = operation
            .call_method0(py, "mode")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(mode_op, mode);
    })
}

#[test_case(1, Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(1, Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(1, Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(1, Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(1, Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(1, Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
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

/// Test hqslang() function for SingleModeGate Operations
#[test_case("QuantumRabi", Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case("LongitudinalCoupling", Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case("JaynesCummings", Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case("SingleExcitationLoad", Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case("SingleExcitationStore", Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case("CZQubitResonator", Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
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

/// Test tags() function for SingleModeGate Operations
#[test_case(
    Operation::from(QuantumRabi::new(1, 0, 1.0.into())),
    vec![
        "Operation",
        "GateOperation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "SingleQubitGateOperation",
        "QuantumRabi",
    ];
    "QuantumRabi")]
#[test_case(
    Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())),
    vec![
        "Operation",
        "GateOperation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "SingleQubitGateOperation",
        "LongitudinalCoupling",
    ];
    "LongitudinalCoupling")]
#[test_case(
    Operation::from(JaynesCummings::new(1, 0, 1.0.into())),
    vec![
        "Operation",
        "GateOperation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "SingleQubitGateOperation",
        "JaynesCummings",
    ];
    "JaynesCummings")]
#[test_case(
    Operation::from(SingleExcitationLoad::new(1, 0)),
    vec![
        "Operation",
        "GateOperation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "SingleQubitGateOperation",
        "SingleExcitationLoad",
    ];
    "SingleExcitationLoad")]
#[test_case(
    Operation::from(SingleExcitationStore::new(1, 0)),
    vec![
        "Operation",
        "GateOperation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "SingleQubitGateOperation",
        "SingleExcitationStore",
    ];
    "SingleExcitationStore")]
#[test_case(
    Operation::from(CZQubitResonator::new(1, 0)),
    vec![
        "Operation",
        "GateOperation",
        "ModeGateOperation",
        "SingleModeGateOperation",
        "SingleQubitGateOperation",
        "CZQubitResonator",
    ];
    "CZQubitResonator")]
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

/// Test involved_modes() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())), HashSet::<usize>::from([0]); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())), HashSet::<usize>::from([0]); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())), HashSet::<usize>::from([0]); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)), HashSet::<usize>::from([0]); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)), HashSet::<usize>::from([0]); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)), HashSet::<usize>::from([0]); "CZQubitResonator")]
fn test_pyo3_involved_modes(input_operation: Operation, modes: HashSet<usize>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial mode
        let involved_modes: HashSet<usize> = operation
            .call_method0(py, "involved_modes")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(involved_modes, modes);
    })
}

/// Test remap_qubits() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
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
        assert_eq!(qubit.clone(), 1);
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
        assert_eq!(qubit_new.clone(), 0);
        // test that initial and rempapped qubits are different
        assert_ne!(qubit, qubit_new);
    })
}

/// Test remap_modes() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
fn test_pyo3_remapmodes_single(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial mode
        let mode: usize = operation
            .call_method0(py, "mode")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(mode.clone(), 0);
        // remap modes
        let mut mode_mapping: HashMap<usize, usize> = HashMap::new();
        mode_mapping.insert(0, 1);
        mode_mapping.insert(1, 0);
        let result = operation
            .call_method1(py, "remap_modes", (mode_mapping,))
            .unwrap();
        // test re-mapped mode
        let mode_new: usize = result
            .call_method0(py, "mode")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(mode_new.clone(), 1);
        // test that initial and rempapped modes are different
        assert_ne!(mode, mode_new);
    })
}

// test remap_modes() function returning an error.
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
fn test_pyo3_remapmodes_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // remap modes
        let mut mode_mapping: HashMap<usize, usize> = HashMap::new();
        mode_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_modes", (mode_mapping,));
        assert!(result.is_err());
    })
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
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

/// Test copy and deepcopy functions
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
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

/// Test format and repr functions
#[test_case(
    "QuantumRabi { qubit: 1, mode: 0, theta: Float(1.0) }",
    Operation::from(QuantumRabi::new(1, 0, 1.0.into()));
    "QuantumRabi")]
#[test_case(
    "LongitudinalCoupling { qubit: 1, mode: 0, theta: Float(1.0) }",
    Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into()));
    "LongitudinalCoupling")]
#[test_case(
    "JaynesCummings { qubit: 1, mode: 0, theta: Float(1.0) }",
    Operation::from(JaynesCummings::new(1, 0, 1.0.into()));
    "JaynesCummings")]
#[test_case(
    "SingleExcitationLoad { qubit: 1, mode: 0 }",
    Operation::from(SingleExcitationLoad::new(1, 0));
    "SingleExcitationLoad")]
#[test_case(
    "SingleExcitationStore { qubit: 1, mode: 0 }",
    Operation::from(SingleExcitationStore::new(1, 0));
    "SingleExcitationStore")]
#[test_case(
    "CZQubitResonator { qubit: 1, mode: 0 }",
    Operation::from(CZQubitResonator::new(1, 0));
    "CZQubitResonator")]
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

/// Test substitute_parameters() function for one parameter
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from("theta"))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from("theta"))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from("theta"))); "JaynesCummings")]
fn test_pyo3_substitute_params_single(input_operation: Operation) {
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
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from("theta"))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from("theta"))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from("theta"))); "JaynesCummings")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let substitution_dict: HashMap<String, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

/// Test substitute parameters function for SingleModeGate Operations where it has no effect
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from(1.0))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from(1.0))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from(1.0))); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
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

/// Test the __richcmp__ function
#[test_case(
    Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from(1.0))),
    Operation::from(QuantumRabi::new(0, 1, CalculatorFloat::from(1.0)));
    "QuantumRabi")]
#[test_case(
    Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from(1.0))),
    Operation::from(LongitudinalCoupling::new(0, 1, CalculatorFloat::from(1.0)));
    "LongitudinalCoupling")]
#[test_case(
    Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from(1.0))),
    Operation::from(JaynesCummings::new(0, 1, CalculatorFloat::from(1.0)));
    "JaynesCummings")]
#[test_case(
    Operation::from(SingleExcitationLoad::new(1, 0)),
    Operation::from(SingleExcitationLoad::new(0, 1));
    "SingleExcitationLoad")]
#[test_case(
    Operation::from(SingleExcitationStore::new(1, 0)),
    Operation::from(SingleExcitationStore::new(0, 1));
    "SingleExcitationStore")]
#[test_case(
    Operation::from(CZQubitResonator::new(1, 0)),
    Operation::from(CZQubitResonator::new(0, 1));
    "CZQubitResonator")]
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

/// Test json_schema function
#[cfg(feature = "json_schema")]
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from(1.0))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from(1.0))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from(1.0))); "JaynesCummings")]
#[test_case(Operation::from(SingleExcitationLoad::new(1, 0)); "SingleExcitationLoad")]
#[test_case(Operation::from(SingleExcitationStore::new(1, 0)); "SingleExcitationStore")]
#[test_case(Operation::from(CZQubitResonator::new(1, 0)); "CZQubitResonator")]
fn test_pyo3_json_schema(operation: Operation) {
    let rust_schema = match operation {
        Operation::QuantumRabi(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(QuantumRabi)).unwrap()
        }
        Operation::LongitudinalCoupling(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(LongitudinalCoupling)).unwrap()
        }
        Operation::JaynesCummings(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(JaynesCummings)).unwrap()
        }
        Operation::SingleExcitationLoad(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(SingleExcitationLoad)).unwrap()
        }
        Operation::SingleExcitationStore(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(SingleExcitationStore)).unwrap()
        }
        Operation::CZQubitResonator(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(CZQubitResonator)).unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = "1.11.0".to_string();
        let pyobject = convert_operation_to_pyobject(operation).unwrap();
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
