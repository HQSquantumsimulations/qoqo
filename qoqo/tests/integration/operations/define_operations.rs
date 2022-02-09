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

use pyo3::prelude::*;
use qoqo::operations::*;
use roqoqo::operations::*;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

/// Test DefinitionFloat new() function
#[test]
fn test_pyo3_new_definition_float() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<DefinitionFloatWrapper>();
        let new_op = operation
            .call1(("ro".to_string(), 1, false))
            .unwrap()
            .cast_as::<PyCell<DefinitionFloatWrapper>>()
            .unwrap();

        let input_definition = Operation::from(DefinitionFloat::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract(new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionFloatWrapper>().unwrap();
        let new_op_diff = operation
            .call1(("ro".to_string(), 1, true))
            .unwrap()
            .cast_as::<PyCell<DefinitionFloatWrapper>>()
            .unwrap();
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
        let new_op = operation
            .call1(("ro".to_string(), 1, false))
            .unwrap()
            .cast_as::<PyCell<DefinitionComplexWrapper>>()
            .unwrap();

        let input_definition =
            Operation::from(DefinitionComplex::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract(new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionComplexWrapper>().unwrap();
        let new_op_diff = operation
            .call1(("ro".to_string(), 1, true))
            .unwrap()
            .cast_as::<PyCell<DefinitionComplexWrapper>>()
            .unwrap();
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
        let new_op = operation
            .call1(("ro".to_string(), 1, false))
            .unwrap()
            .cast_as::<PyCell<DefinitionUsizeWrapper>>()
            .unwrap();

        let input_definition = Operation::from(DefinitionUsize::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract(new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionUsizeWrapper>().unwrap();
        let new_op_diff = operation
            .call1(("ro".to_string(), 1, true))
            .unwrap()
            .cast_as::<PyCell<DefinitionUsizeWrapper>>()
            .unwrap();
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
        let new_op = operation
            .call1(("ro".to_string(), 1, false))
            .unwrap()
            .cast_as::<PyCell<DefinitionBitWrapper>>()
            .unwrap();

        let input_definition = Operation::from(DefinitionBit::new(String::from("ro"), 1, false));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract(new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<DefinitionBitWrapper>().unwrap();
        let new_op_diff = operation
            .call1(("ro".to_string(), 1, true))
            .unwrap()
            .cast_as::<PyCell<DefinitionBitWrapper>>()
            .unwrap();
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
        let new_op = operation
            .call1(("ro".to_string(), 1.0))
            .unwrap()
            .cast_as::<PyCell<InputSymbolicWrapper>>()
            .unwrap();

        let input_definition = Operation::from(InputSymbolic::new(String::from("ro"), 1.0));
        let copy_param = convert_operation_to_pyobject(input_definition).unwrap();
        let comparison_copy =
            bool::extract(new_op.call_method1("__eq__", (copy_param,)).unwrap()).unwrap();
        assert!(comparison_copy);

        let def_wrapper = new_op.extract::<InputSymbolicWrapper>().unwrap();
        let new_op_diff = operation
            .call1(("ro".to_string(), 2.0))
            .unwrap()
            .cast_as::<PyCell<InputSymbolicWrapper>>()
            .unwrap();
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

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic name() function/input
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
fn test_pyo3_name(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let name_op: String =
            String::extract(operation.call_method0(py, "name").unwrap().as_ref(py)).unwrap();
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
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let length_op: &usize =
            &usize::extract(operation.call_method0(py, "length").unwrap().as_ref(py)).unwrap();
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
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        assert!(
            !bool::extract(operation.call_method0(py, "is_output").unwrap().as_ref(py)).unwrap()
        );
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
        let input_op: &f64 =
            &f64::extract(operation.call_method0(py, "input").unwrap().as_ref(py)).unwrap();
        let input_param: &f64 = &1.0;
        assert_eq!(input_op, input_param);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic involved_qubits function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
fn test_pyo3_involved_qubits(input_definition: Operation) {
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let involved_op: HashSet<String> = HashSet::extract(
            operation
                .call_method0(py, "involved_qubits")
                .unwrap()
                .as_ref(py),
        )
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
        let format_op: &str = <&str>::extract(to_format.as_ref(py)).unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr.as_ref(py)).unwrap();
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
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(Operation::from(InputSymbolic::new(
            String::from("ro"),
            1.0,
        )))
        .unwrap();
        let to_format = operation.call_method1(py, "__format__", ("",)).unwrap();
        let format_op: &str = <&str>::extract(to_format.as_ref(py)).unwrap();
        let to_repr = operation.call_method0(py, "__repr__").unwrap();
        let repr_op: &str = <&str>::extract(to_repr.as_ref(py)).unwrap();
        let format_repr_param: String = String::from("InputSymbolic { name: \"ro\", input: 1.0 }");
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
fn test_pyo3_copy_deepcopy(input_definition: Operation) {
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
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

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic tags function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)), "DefinitionFloat"; "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)), "DefinitionComplex"; "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)), "DefinitionUsize"; "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)), "DefinitionBit"; "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)), "InputSymbolic"; "InputSymbolic")]
fn test_pyo3_tags(input_definition: Operation, tag_name: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let to_tag = operation.call_method0(py, "tags").unwrap();
        let tags_op: &Vec<&str> = &Vec::extract(to_tag.as_ref(py)).unwrap();
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
fn test_pyo3_hqslang(input_definition: Operation, hqslang_param: String) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let hqslang_op: String =
            String::extract(operation.call_method0(py, "hqslang").unwrap().as_ref(py)).unwrap();
        assert_eq!(hqslang_op, hqslang_param);
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit, InputSymbolic is_parametrized function
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
fn test_pyo3_is_parametrized(input_definition: Operation) {
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        assert!(!bool::extract(
            operation
                .call_method0(py, "is_parametrized")
                .unwrap()
                .as_ref(py)
        )
        .unwrap());
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit substitute_parameters functions
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
fn test_pyo3_substitute_parameters(input_definition: Operation) {
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let mut substitution_dict: HashMap<&str, f64> = HashMap::new();
        substitution_dict.insert("ro", 1.0);
        let substitute_op = operation
            .call_method1(py, "substitute_parameters", (substitution_dict,))
            .unwrap();
        let substitute_param = operation;

        let comparison_copy = bool::extract(
            substitute_op
                .as_ref(py)
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
fn test_pyo3_substitute_parameters_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation).unwrap();
        let mut substitution_dict: HashMap<&str, &str> = HashMap::new();
        substitution_dict.insert("ro", "test");
        let result = operation.call_method1(py, "substitute_parameters", (substitution_dict,));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());
    })
}

/// Test DefinitionFloat, DefinitionComplex, DefinitionUsize, DefinitionBit remap_qubits functions
#[test_case(Operation::from(DefinitionFloat::new(String::from("ro"), 1, false)); "DefinitionFloat")]
#[test_case(Operation::from(DefinitionComplex::new(String::from("ro"), 1, false)); "DefinitionComplex")]
#[test_case(Operation::from(DefinitionUsize::new(String::from("ro"), 1, false)); "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
#[test_case(Operation::from(InputSymbolic::new(String::from("ro"), 1.0)); "InputSymbolic")]
fn test_pyo3_remap_qubits(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition).unwrap();
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 1);
        let remap_op = operation
            .call_method1(py, "remap_qubits", (qubit_mapping,))
            .unwrap();
        let remap_param = operation;

        let comparison_copy = bool::extract(
            remap_op
                .as_ref(py)
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
