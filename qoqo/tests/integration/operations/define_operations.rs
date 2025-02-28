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

#[cfg(feature = "unstable_operation_definition")]
use super::pragma_operations::new_circuit;
use pyo3::prelude::*;
use qoqo::operations::*;
use roqoqo::operations::*;
#[cfg(feature = "unstable_operation_definition")]
use roqoqo::Circuit;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

/// Test DefinitionFloat new() function
#[test]
fn test_pyo3_new_definition_float() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<DefinitionFloatWrapper>();
        let binding = operation.call1(("ro".to_string(), 1, false)).unwrap();
        let new_op = binding.downcast::<DefinitionFloatWrapper>().unwrap();

        let input_definition = Operation::from(DefinitionFloat::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionFloatWrapper>().unwrap();
        let binding = operation.call1(("ro".to_string(), 1, true)).unwrap();
        let new_op_diff = binding.downcast::<DefinitionFloatWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<DefinitionFloatWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper),
            "DefinitionFloatWrapper { internal: DefinitionFloat { name: \"ro\", length: 1, is_output: false } }"
        );
    })
}

/// Test DefinitionComplex new() function
#[test]
fn test_pyo3_new_definition_complex() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<DefinitionComplexWrapper>();
        let binding = operation.call1(("ro".to_string(), 1, false)).unwrap();
        let new_op = binding.downcast::<DefinitionComplexWrapper>().unwrap();

        let input_definition =
            Operation::from(DefinitionComplex::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionComplexWrapper>().unwrap();
        let binding = operation.call1(("ro".to_string(), 1, true)).unwrap();
        let new_op_diff = binding.downcast::<DefinitionComplexWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<DefinitionComplexWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper),
            "DefinitionComplexWrapper { internal: DefinitionComplex { name: \"ro\", length: 1, is_output: false } }"
        );
    })
}

/// Test DefinitionUsize new() function
#[test]
fn test_pyo3_new_definition_usize() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<DefinitionUsizeWrapper>();
        let binding = operation.call1(("ro".to_string(), 1, false)).unwrap();
        let new_op = binding.downcast::<DefinitionUsizeWrapper>().unwrap();

        let input_definition = Operation::from(DefinitionUsize::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionUsizeWrapper>().unwrap();
        let binding = operation.call1(("ro".to_string(), 1, true)).unwrap();
        let new_op_diff = binding.downcast::<DefinitionUsizeWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<DefinitionUsizeWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper),
            "DefinitionUsizeWrapper { internal: DefinitionUsize { name: \"ro\", length: 1, is_output: false } }"
        );
    })
}

/// Test DefinitionBit new() function
#[test]
fn test_pyo3_new_definition_bit() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<DefinitionBitWrapper>();
        let binding = operation.call1(("ro".to_string(), 1, false)).unwrap();
        let new_op = binding.downcast::<DefinitionBitWrapper>().unwrap();

        let input_definition = Operation::from(DefinitionBit::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionBitWrapper>().unwrap();
        let binding = operation.call1(("ro".to_string(), 1, true)).unwrap();
        let new_op_diff = binding.downcast::<DefinitionBitWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<DefinitionBitWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper),
            "DefinitionBitWrapper { internal: DefinitionBit { name: \"ro\", length: 1, is_output: false } }"
        );
    })
}

/// Test InputSymbolic new() function
#[test]
fn test_pyo3_new_input_symbolic() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<InputSymbolicWrapper>();
        let binding = operation.call1(("ro".to_string(), 1.0)).unwrap();
        let new_op = binding.downcast::<InputSymbolicWrapper>().unwrap();

        let input_definition = Operation::from(InputSymbolic::new(String::from("ro"), 1.0));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<InputSymbolicWrapper>().unwrap();
        let binding = operation.call1(("ro".to_string(), 2.0)).unwrap();
        let new_op_diff = binding.downcast::<InputSymbolicWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InputSymbolicWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper),
            "InputSymbolicWrapper { internal: InputSymbolic { name: \"ro\", input: 1.0 } }"
        );
    })
}

/// Test InputBit new() function
#[test]
fn test_pyo3_new_input_bit() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<InputBitWrapper>();
        let binding = operation.call1(("ro".to_string(), 1, false)).unwrap();
        let new_op = binding.downcast::<InputBitWrapper>().unwrap();

        let input_definition = Operation::from(InputBit::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<InputBitWrapper>().unwrap();
        let binding = operation.call1(("ro".to_string(), 2, false)).unwrap();
        let new_op_diff = binding.downcast::<InputBitWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<InputBitWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper),
            "InputBitWrapper { internal: InputBit { name: \"ro\", index: 1, value: false } }"
        );
    })
}

/// Test GateDefinition new() function
#[test]
#[cfg(feature = "unstable_operation_definition")]
fn test_pyo3_new_gate_definition() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<GateDefinitionWrapper>();
        let binding = operation
            .call1((
                new_circuit(py),
                String::from("ro"),
                vec![1],
                vec!["a".to_owned(), "b".to_owned()],
            ))
            .unwrap();
        let new_op = binding.downcast::<GateDefinitionWrapper>().unwrap();

        let input_definition = Operation::from(GateDefinition::new(
            Circuit::new(),
            "ro".into(),
            vec![1],
            vec!["a".into(), "b".into()],
        ));
        let copy_param = convert_operation_to_pyobject(input_definition)
            .unwrap()
            .extract::<GateDefinitionWrapper>(py)
            .unwrap()
            .into_py(py);

        let comparison_copy =
            bool::extract_bound(&new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<GateDefinitionWrapper>().unwrap();
        let new_op_diff = operation
            .call1((
                new_circuit(py),
                String::from("ro"),
                vec![2],
                vec!["a".to_owned(), "c".to_owned()],
            ))
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .downcast::<GateDefinitionWrapper>()
            .unwrap()
            .extract::<GateDefinitionWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", def_wrapper),
            "GateDefinitionWrapper { internal: GateDefinition { circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }, name: \"ro\", qubits: [1], free_parameters: [\"a\", \"b\"] } }"
        );
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic, name() function/input
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_name(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let name_op: String = operation
            .call_method0(py, "name")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let name_param: String = String::from("ro");
        assert_eq!(name_op, name_param);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit length() input
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
fn test_pyo3_length(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let length_op: &usize = &operation
            .call_method0(py, "length")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let length_param: &usize = &1_usize;
        assert_eq!(length_op, length_param);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit is_output() input
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
fn test_pyo3_is_output(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        assert!(!&operation
            .call_method0(py, "is_output")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

/// Test InputSymbolic input() input
#[test]
fn test_pyo3_input_symbolic_input() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(InputSymbolic::new(
            String::from("ro"),
            1.0,
        )))
        .unwrap();
        let input_op: &f64 = &operation
            .call_method0(py, "input")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let input_param: &f64 = &1.0;
        assert_eq!(input_op, input_param);
    })
}

/// Test InputBit index() input
#[test]
fn test_pyo3_input_bit_index() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(InputBit::new(
            String::from("ro"),
            1,
            true,
        )))
        .unwrap();
        let input_op: &usize = &operation
            .call_method0(py, "index")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let input_param: &usize = &1;
        assert_eq!(input_op, input_param);
    })
}

/// Test InputBit value() input
#[test]
fn test_pyo3_input_bit_value() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(InputBit::new(
            String::from("ro"),
            1,
            true,
        )))
        .unwrap();
        let input_op: &bool = &operation
            .call_method0(py, "value")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let input_param: &bool = &true;
        assert_eq!(input_op, input_param);
    })
}

#[cfg(feature = "unstable_operation_definition")]
/// Test inputs for GateDefinition
#[test]
fn test_pyo3_gate_definition_inputs() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            String::from("name"),
            vec![1, 2],
            vec!["test".into()],
        )))
        .unwrap();

        // Test circuit()
        let to_circuit = operation.call_method0(py, "circuit").unwrap();
        let circuit_op = to_circuit.bind(py);
        let circuit = new_circuit(py);
        let comparison_circuit =
            bool::extract_bound(&circuit_op.call_method1("__eq__", (circuit,)).unwrap()).unwrap();
        assert!(comparison_circuit);

        // Test name()
        let name_op: String = operation
            .call_method0(py, "name")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let name_param: String = String::from("name");
        assert_eq!(name_op, name_param);

        // Test qubits()
        let qubits: Vec<usize> = operation
            .call_method0(py, "qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(qubits, vec![1, 2]);

        // Test free_parameters()
        let free_parameters: Vec<String> = operation
            .call_method0(py, "free_parameters")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(free_parameters, vec!["test".to_owned()]);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic, involved_qubits function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_involved_qubits(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let involved_op: HashSet<String> = operation
            .call_method0(py, "involved_qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let involved_param: HashSet<_> = HashSet::new();
        assert_eq!(involved_op, involved_param);
    })
}

/// Test GateDefinition involved_qubits function
#[cfg(feature = "unstable_operation_definition")]
#[test_case(Operation::from(GateDefinition::new(Circuit::new(), String::from("ro"), vec![1], vec!["test".into()])); "GateDefinition")]
fn test_pyo3_involved_qubits_gate_definition(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let involved_op: HashSet<String> = operation
            .call_method0(py, "involved_qubits")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        let involved_param: HashSet<_> = HashSet::new();
        assert_eq!(involved_op, involved_param);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit format and repr functions
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)), "DefinitionFloat"; "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)), "DefinitionComplex"; "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)), "DefinitionUsize"; "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)), "DefinitionBit"; "DefinitionBit")]
fn test_pyo3_format_repr(input_definition: Operation, format_repr: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = to_format.bind(py).extract().unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = to_repr.bind(py).extract().unwrap();
        let mut format_repr_param: String = String::from(format_repr);
        format_repr_param.push_str(" { name: \"ro\", length: 1, is_output: false }");
        let comparison = format_repr_param.as_str();
        assert_eq!(format_op, comparison);
        assert_eq!(repr_op, comparison);
    })
}

/// Test InputSymbolic format and repr functions
#[test]
fn test_pyo3_input_symbolic_format_repr() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(InputSymbolic::new(
            String::from("ro"),
            1.0,
        )))
        .unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = to_format.bind(py).extract().unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = to_repr.bind(py).extract().unwrap();
        let format_repr_param: String = String::from("InputSymbolic { name: \"ro\", input: 1.0 }");
        let comparison = format_repr_param.as_str();
        assert_eq!(format_op, comparison);
        assert_eq!(repr_op, comparison);
    })
}

/// Test GateDefinition format and repr functions
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_gate_definition_format_repr() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            String::from("ro"),
            vec![1],
            vec!["test".into()],
        )))
        .unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: String = String::extract_bound(to_format.bind(py)).unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: String = String::extract_bound(to_repr.bind(py)).unwrap();
        let format_repr_param: String = String::from("GateDefinition { circuit: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }, name: \"ro\", qubits: [1], free_parameters: [\"test\"] }");
        let comparison = format_repr_param.as_str();
        assert_eq!(format_op, comparison);
        assert_eq!(repr_op, comparison);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic copy and deepcopy functions
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_copy_deepcopy(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
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

/// Test GateDefinition copy and deepcopy functions
#[test]
#[cfg(feature = "unstable_operation_definition")]
fn test_pyo3_copy_deepcopy_gate_definition() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            "name".to_owned(),
            vec![0],
            vec!["param".to_owned()],
        )))
        .unwrap();
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
/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic tags function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)), "DefinitionFloat"; "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)), "DefinitionComplex"; "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)), "DefinitionUsize"; "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)), "DefinitionBit"; "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)), "InputSymbolic"; "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)), "InputBit"; "InputBit")]
fn test_pyo3_tags(input_definition: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &["Operation", "Definition", tag_name];
        assert_eq!(tags_op, tags_param);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic hqslang function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)), String::from("DefinitionFloat"); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)), String::from("DefinitionComplex"); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)), String::from("DefinitionUsize"); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)), String::from("DefinitionBit"); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)), String::from("InputSymbolic"); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)), String::from("InputBit"); "InputBit")]
fn test_pyo3_hqslang(input_definition: Operation, hqslang_param: String) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let hqslang_op: String = operation
            .call_method0(py, "hqslang")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(hqslang_op, hqslang_param);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic is_parametrized function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_is_parametrized(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        assert!(!operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

// Test GateDefinitions's tags, hslang and is_parametrized functions
#[cfg(feature = "unstable_operation_definition")]
#[test_case(Operation::from(GateDefinition::new(Circuit::new(), String::from("ro"), vec![1], vec!["test".into()])); "GateDefinition")]
fn test_pyo3_gate_definition(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();

        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.bind(py).extract().unwrap();
        let tags_param: &[&str] = &["Operation", "Definition", "GateDefinition"];
        assert_eq!(tags_op, tags_param);

        let hqslang_op: String = operation
            .call_method0(py, "hqslang")
            .unwrap()
            .bind(py)
            .extract()
            .unwrap();
        assert_eq!(hqslang_op, "GateDefinition");

        assert!(!operation
            .call_method0(py, "is_parametrized")
            .unwrap()
            .bind(py)
            .extract::<bool>()
            .unwrap());
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic, InputBit substitute_parameters functions
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_substitute_parameters(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let mut substitution_dict: HashMap<String, f64> = HashMap::new();
        substitution_dict.insert("ro".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();
        let substitute_param = operation;

        let comparison_copy = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (substitute_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test GateDefinitions's substitute_parameters functions
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_substitute_parameters_gate_definition() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            String::from("name"),
            vec![1],
            vec!["test".into()],
        )))
        .unwrap();
        let mut substitution_dict: HashMap<&str, f64> = HashMap::new();
        substitution_dict.insert("name", 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();
        let substitute_param = operation;

        let comparison_copy = bool::extract_bound(
            &substitute_op
                .bind(py)
                .call_method1("__eq__", (substitute_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test substitute_parameters() causing an error `not-a-real-number`
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_substitute_parameters_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let mut substitution_dict: HashMap<&str, &str> = HashMap::new();
        substitution_dict.insert("ro", "test");
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

/// Test GateDefinitions's substitute_parameters() causing an error `not-a-real-number`
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_substitute_parameters_error_gate_definition() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            String::from("name"),
            vec![1],
            vec!["test".into()],
        )))
        .unwrap();
        let mut substitution_dict: HashMap<&str, &str> = HashMap::new();
        substitution_dict.insert("name", "test");
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic, InputBit remap_qubits functions
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_remap_qubits(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 1);
        qubit_mapping.insert(1, 0);
        let remap_op = operation
            .call_method1(py, "remap_qubits", (qubit_mapping,))
            .unwrap();
        let remap_param = operation;

        let comparison_copy = bool::extract_bound(
            &remap_op
                .bind(py)
                .call_method1("__eq__", (remap_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test GateDefinitions's remap_qubits functions
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_remap_qubits_gate_definition() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            String::from("name"),
            vec![0],
            vec!["test".into()],
        )))
        .unwrap();
        let remaped_operation =
            convert_operation_to_pyobject(Operation::from(GateDefinition::new(
                Circuit::new(),
                String::from("name"),
                vec![1],
                vec!["test".into()],
            )))
            .unwrap();
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 1);
        qubit_mapping.insert(1, 0);
        let remap_op = operation
            .call_method1(py, "remap_qubits", (qubit_mapping,))
            .unwrap();
        let remap_param = remaped_operation;

        let comparison_copy = bool::extract_bound(
            &remap_op
                .bind(py)
                .call_method1("__eq__", (remap_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
    })
}

/// Test the __richcmp__ function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)),
            Operation::from(DefinitionFloat::new(String::from("ro"), 1, true));
            "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)),
            Operation::from(DefinitionComplex::new(String::from("ro"), 1, true));
            "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)),
            Operation::from(DefinitionUsize::new(String::from("ro"), 1, true));
            "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)),
            Operation::from(DefinitionBit::new(String::from("ro"), 1, true));
            "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)),
            Operation::from(InputSymbolic::new(String::from("ro"), 2.0));
            "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)),
            Operation::from(InputBit::new(String::from("ro"), 2, true));
            "InputBit")]
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

/// Test the __richcmp__ function for GateDefinitions
#[test]
#[cfg(feature = "unstable_operation_definition")]
fn test_pyo3_richcmp_gate_definition() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            String::from("name"),
            vec![0],
            vec!["test".into()],
        )))
        .unwrap();
        let operation_two = convert_operation_to_pyobject(Operation::from(GateDefinition::new(
            Circuit::new(),
            String::from("name"),
            vec![1],
            vec!["testing".into()],
        )))
        .unwrap();

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

/// Test json_schema function for all define operations
#[cfg(feature = "json_schema")]
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
#[test_case(Operation::from(InputBit::new(String::from("ro"), 1, true)); "InputBit")]
fn test_pyo3_json_schema(operation: Operation) {
    let rust_schema = match operation {
        Operation::DefinitionFloat(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(DefinitionFloat)).unwrap()
        }
        Operation::DefinitionComplex(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(DefinitionComplex)).unwrap()
        }
        Operation::DefinitionUsize(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(DefinitionUsize)).unwrap()
        }
        Operation::DefinitionBit(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(DefinitionBit)).unwrap()
        }
        Operation::InputSymbolic(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(InputSymbolic)).unwrap()
        }
        Operation::InputBit(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(InputBit)).unwrap()
        }
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = match operation {
            Operation::InputBit(_) => "1.1.0".to_string(),
            _ => "1.0.0".to_string(),
        };
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

/// Test json_schema function for GateDefinitions
#[test]
#[cfg(feature = "unstable_operation_definition")]
#[cfg(feature = "json_schema")]
fn test_pyo3_json_schema_gate_definition() {
    let operation = Operation::from(GateDefinition::new(
        Circuit::new(),
        String::from("name"),
        vec![0],
        vec!["test".into()],
    ));
    let rust_schema = serde_json::to_string_pretty(&schemars::schema_for!(GateDefinition)).unwrap();
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let minimum_version: String = "1.13.0".to_owned();
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
