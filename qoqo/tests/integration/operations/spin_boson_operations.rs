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

use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
use qoqo::operations::{
    QuantumRabi, LongitudinalCoupling, JaynesCummings
};
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::{HashMap, HashSet};
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

/// Test new() function for QuantumRabi
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())), (1, 0, 1.0,), "__eq__"; "QuantumRabi_eq")]
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())), (0, 1, 1.0,), "__ne__"; "QuantumRabi_ne")]
fn test_new_quantumrabi(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<QuantumRabiWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<QuantumRabiWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<QuantumRabiWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, 3, 1.0))
            .unwrap()
            .downcast::<PyCell<QuantumRabiWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<QuantumRabiWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "QuantumRabiWrapper { internal: QuantumRabi { qubit: 2, mode: 3, theta: Float(1.0) } }"
        );

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "theta")
                .unwrap()
                .as_ref(py)
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
fn test_new_longitudinal_coupling(input_operation: Operation, arguments: (u32, u32, f64), method: &str) {
    let operation = convert_operation_to_pyobject(input_operation).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_type = py.get_type::<LongitudinalCouplingWrapper>();
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<LongitudinalCouplingWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<LongitudinalCouplingWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, 3, 1.0))
            .unwrap()
            .downcast::<PyCell<LongitudinalCouplingWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<LongitudinalCouplingWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "LongitudinalCouplingWrapper { internal: LongitudinalCoupling { qubit: 2, mode: 3, theta: Float(1.0) } }"
        );

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "theta")
                .unwrap()
                .as_ref(py)
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
        let operation_py = operation_type
            .call1(arguments)
            .unwrap()
            .downcast::<PyCell<JaynesCummingsWrapper>>()
            .unwrap();

        let comparison = bool::extract(
            operation
                .as_ref(py)
                .call_method1(method, (operation_py,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let def_wrapper = operation_py.extract::<JaynesCummingsWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1((2, 3, 1.0))
            .unwrap()
            .downcast::<PyCell<JaynesCummingsWrapper>>()
            .unwrap();
        let def_wrapper_diff = new_op_diff.extract::<JaynesCummingsWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper_diff),
            "JaynesCummingsWrapper { internal: JaynesCummings { qubit: 2, mode: 3, theta: Float(1.0) } }"
        );

        let comparison_copy = bool::extract(
            operation
                .call_method0(py, "theta")
                .unwrap()
                .as_ref(py)
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

/// Test is_parametrized() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from("theta"))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from("theta"))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from("theta"))); "JaynesCummings")]
fn test_pyo3_is_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        assert!(bool::extract(
            operation
                .call_method0(py, "is_parametrized")
                .unwrap()
                .as_ref(py)
        )
                .unwrap());
    })
}

/// Test is_parametrized = false for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
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

/// Test mode() function for SingleMode Operations
#[test_case(0, Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(0, Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(0, Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
fn test_pyo3_mode(mode: usize, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let mode_op: usize =
            usize::extract(operation.call_method0(py, "mode").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_op, mode);
    })
}

#[test_case(1, Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(1, Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(1, Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
fn test_pyo3_qubit(qubit: usize, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let qubit_op: usize =
            usize::extract(operation.call_method0(py, "qubit").unwrap().as_ref(py)).unwrap();
        assert_eq!(qubit_op, qubit);
    })
}

/// Test hqslang() function for SingleModeGate Operations
#[test_case("QuantumRabi", Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case("LongitudinalCoupling", Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case("JaynesCummings", Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let name_op: String =
            String::extract(operation.call_method0(py, "hqslang").unwrap().as_ref(py)).unwrap();
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
        "SingleQubitGate",
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
        "SingleQubitGate",
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
        "SingleQubitGate",
        "JaynesCummings",
    ];
    "JaynesCummings")]
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
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

/// Test involved_modes() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())), HashSet::<usize>::from([0]); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())), HashSet::<usize>::from([0]); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())), HashSet::<usize>::from([0]); "JaynesCummings")]
fn test_pyo3_involved_modes(input_operation: Operation, modes: HashSet<usize>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial mode
        let involved_modes: HashSet<usize> = HashSet::<usize>::extract(
            operation
                .call_method0(py, "involved_modes")
                .unwrap()
                .as_ref(py),
        )
        .unwrap();
        assert_eq!(involved_modes, modes);
    })
}

/// Test remap_qubits() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial qubit
        let qubit: usize =
            usize::extract(operation.call_method0(py, "qubit").unwrap().as_ref(py)).unwrap();
        assert_eq!(qubit.clone(), 1);
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 1);
        qubit_mapping.insert(1, 0);
        let result = operation
            .call_method1(py, "remap_qubits", (qubit_mapping,))
            .unwrap();
        // test re-mapped qubit
        let qubit_new: usize =
            usize::extract(result.call_method0(py, "qubit").unwrap().as_ref(py)).unwrap();
        assert_eq!(qubit_new.clone(), 0);
        // test that initial and rempapped qubits are different
        assert_ne!(qubit, qubit_new);
    })
}

/// Test remap_modes() function for SingleModeGate Operations
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
fn test_pyo3_remapmodes_single(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // test initial mode
        let mode: usize =
            usize::extract(operation.call_method0(py, "mode").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode.clone(), 0);
        // remap modes
        let mut mode_mapping: HashMap<usize, usize> = HashMap::new();
        mode_mapping.insert(0, 1);
        mode_mapping.insert(1, 0);
        let result = operation
            .call_method1(py, "remap_modes", (mode_mapping,))
            .unwrap();
        // test re-mapped mode
        let mode_new: usize =
            usize::extract(result.call_method0(py, "mode").unwrap().as_ref(py)).unwrap();
        assert_eq!(mode_new.clone(), 1);
        // test that initial and rempapped modes are different
        assert_ne!(mode, mode_new);
    })
}


// test remap_modes() function returning an error.
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
fn test_pyo3_remapmodes_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        // remap modes
        let mut mode_mapping: HashMap<usize, usize> = HashMap::new();
        mode_mapping.insert(2, 0);
        let result = operation.call_method1(py, "remap_modes", (mode_mapping,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
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

/// Test copy and deepcopy functions
#[test_case(Operation::from(QuantumRabi::new(1, 0, 1.0.into())); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, 1.0.into())); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, 1.0.into())); "JaynesCummings")]
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

/// Test format and repr functions
#[test_case(
    "QuantumRabi { qubit: 1, mode: 0, theta: Float(1.0) }",
    Operation::from(QuantumRabi::new(1, 0, 0.0.into()));
    "QuantumRabi")]
#[test_case(
    "LongitudinalCoupling { qubit: 1, mode: 0, theta: Float(1.0) }",
    Operation::from(LongitudinalCoupling::new(1, 0, 0.0.into()));
    "LongitudinalCoupling")]
#[test_case(
    "JaynesCummings { qubit: 1, mode: 0, theta: Float(1.0) }",
    Operation::from(JaynesCummings::new(1, 0, 0.0.into()));
    "JaynesCummings")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format.as_ref(py)).unwrap();
        assert_eq!(format_op, format_repr);
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr.as_ref(py)).unwrap();
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
        let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
        substitution_dict_py.insert("theta", 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param).unwrap();

        let comparison = bool::extract(
            substitute_op
                .as_ref(py)
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
        let substitution_dict: HashMap<&str, f64> = HashMap::new();
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

/// Test substitute parameters function for SingleModeGate Operations where it has no effect
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from(1.0))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from(1.0))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from(1.0))); "JaynesCummings")]
fn test_ineffective_substitute_parameters(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
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
    "QuantumRabi")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
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
    })
}

/// Test json_schema function 
#[cfg(feature = "json_schema")]
#[test_case(Operation::from(QuantumRabi::new(1, 0, CalculatorFloat::from(1.0))); "QuantumRabi")]
#[test_case(Operation::from(LongitudinalCoupling::new(1, 0, CalculatorFloat::from(1.0))); "LongitudinalCoupling")]
#[test_case(Operation::from(JaynesCummings::new(1, 0, CalculatorFloat::from(1.0))); "JaynesCummings")]
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
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = "1.10.0".to_string();
        let pyobject = convert_operation_to_pyobject(operation).unwrap();
        let operation = pyobject.as_ref(py);

        let schema: String =
            String::extract(operation.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract(operation.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract(operation.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, minimum_version);
    });
}
