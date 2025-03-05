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

use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use qoqo::measurements::{PauliZProductInputWrapper, PauliZProductWrapper};
use qoqo::operations::{convert_operation_to_pyobject, RotateXWrapper};
use qoqo::{CircuitWrapper, OperationIteratorWrapper, QOQO_VERSION};
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::Circuit;
use roqoqo::ROQOQO_VERSION;
use std::collections::{HashMap, HashSet};
use test_case::test_case;

// helper functions
fn new_circuit(py: Python) -> Bound<CircuitWrapper> {
    let circuit_type = py.get_type::<CircuitWrapper>();
    circuit_type
        .call0()
        .unwrap()
        .downcast::<CircuitWrapper>()
        .unwrap()
        .to_owned()
}

fn populate_circuit_rotatex(
    py: Python,
    circuit: &Bound<CircuitWrapper>,
    start: usize,
    stop: usize,
) {
    let rotatex_type = py.get_type::<RotateXWrapper>();
    for i in start..stop {
        let new_rotatex_0 = rotatex_type.call1((i, i)).unwrap();
        circuit.call_method1("add", (new_rotatex_0,)).unwrap();
    }
}

fn add_circuit_measurement_operation(circuit: &Bound<CircuitWrapper>) {
    let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
    qubit_mapping.insert(0, 1);
    let input_measurement: Operation = Operation::from(PragmaRepeatedMeasurement::new(
        String::from("ro"),
        2,
        Some(qubit_mapping),
    ));
    let measurement_operation = convert_operation_to_pyobject(input_measurement).unwrap();
    circuit
        .call_method1("add", (measurement_operation,))
        .unwrap();
}

/// Test default function of CircuitWrapper
#[test]
fn test_default() {
    let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation.clone_ref(py),))
            .unwrap();
        let circuit_wrapper = circuit.extract::<CircuitWrapper>();

        let helper_ne: bool = CircuitWrapper::default() != circuit_wrapper.unwrap();
        assert!(helper_ne);
        let helper_eq: bool = CircuitWrapper::default() == CircuitWrapper::new();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", CircuitWrapper::new()),
            "CircuitWrapper { internal: Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion } }"
        );
    })
}

/// Test substitute_parameters function of Circuit
#[test]
fn test_substitute_parameters() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let added_operation = Operation::from(RotateX::new(0, CalculatorFloat::from("test")));

        let operation = convert_operation_to_pyobject(added_operation).unwrap();
        let circuit = new_circuit(py);
        circuit.call_method1("add", (operation,)).unwrap();

        let mut substitution_dict: HashMap<String, f64> = HashMap::new();
        substitution_dict.insert("test".to_owned(), 1.0);
        let substitute_circ = circuit
            .call_method1("substitute_parameters", (substitution_dict,))
            .unwrap();

        let to_sub = Operation::from(RotateX::new(0, CalculatorFloat::from(1.0)));
        let subbed_operation = convert_operation_to_pyobject(to_sub).unwrap();

        let comp_op = substitute_circ.call_method1("__getitem__", (0,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (subbed_operation,)).unwrap())
                .unwrap();
        assert!(comparison);

        let mut substitution_dict_error = HashMap::new();
        substitution_dict_error.insert("fails", 0.0);
        let comparison = circuit.call_method1("substitute_parameters", (substitution_dict_error,));
        assert!(comparison.is_err());
    })
}

/// Test remap_qubits function of Circuit
#[test]
fn test_remap_qubits() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let added_operation = Operation::from(RotateX::new(0, CalculatorFloat::from(1.0)));
        let operation = convert_operation_to_pyobject(added_operation).unwrap();
        let circuit = new_circuit(py);
        circuit.call_method1("add", (operation,)).unwrap();

        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);

        let remap_circ = circuit
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();

        let to_remap = Operation::from(RotateX::new(2, CalculatorFloat::from(1.0)));
        let remapped_operation = convert_operation_to_pyobject(to_remap).unwrap();

        let comp_op = remap_circ.call_method1("__getitem__", (0,)).unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1("__eq__", (remapped_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let mut qubit_mapping_error = HashMap::new();
        qubit_mapping_error.insert(5, 3);
        let comparison = circuit.call_method1("remap_qubits", (qubit_mapping_error,));
        assert!(comparison.is_err());
    })
}

/// Test count_occurences function of Circuit
#[test]
fn test_count_occurences() {
    let added_op1 = Operation::from(DefinitionBit::new("ro".to_string(), 1, false));
    let added_op2 = Operation::from(RotateX::new(0, CalculatorFloat::from(1.0)));
    let added_op3 = Operation::from(PauliX::new(0));
    let operation1 = convert_operation_to_pyobject(added_op1).unwrap();
    let operation2 = convert_operation_to_pyobject(added_op2).unwrap();
    let operation3 = convert_operation_to_pyobject(added_op3).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation1.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (operation2.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (operation3.clone_ref(py),))
            .unwrap();

        let comp_op = usize::extract_bound(
            &circuit
                .call_method1("count_occurences", (vec!["Definition"],))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(comp_op, 1_usize);
        let comp_op = usize::extract_bound(
            &circuit
                .call_method1("count_occurences", (vec!["Operation"],))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(comp_op, 3_usize);
        let comp_op = usize::extract_bound(
            &circuit
                .call_method1("count_occurences", (vec!["RotateX"],))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(comp_op, 1_usize);
        let comp_op = usize::extract_bound(
            &circuit
                .call_method1("count_occurences", (vec!["SingleQubitGateOperation"],))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(comp_op, 2_usize);
        let comp_op = usize::extract_bound(
            &circuit
                .call_method1("count_occurences", (vec!["MadeUp"],))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(comp_op, 0_usize);
    })
}

/// Test get_operation_types function of Circuit
#[test]
fn test_get_operation_types() {
    let added_op1 = Operation::from(DefinitionBit::new("ro".to_string(), 1, false));
    let added_op2 = Operation::from(RotateX::new(0, CalculatorFloat::from(1.0)));
    let added_op3 = Operation::from(PauliX::new(0));
    let operation1 = convert_operation_to_pyobject(added_op1).unwrap();
    let operation2 = convert_operation_to_pyobject(added_op2).unwrap();
    let operation3 = convert_operation_to_pyobject(added_op3).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation1.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (operation2.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (operation3.clone_ref(py),))
            .unwrap();

        let mut op_types: HashSet<String> = HashSet::new();
        op_types.insert("DefinitionBit".to_owned());
        op_types.insert("RotateX".to_owned());
        op_types.insert("PauliX".to_owned());

        let comp_op =
            HashSet::extract_bound(&circuit.call_method0("get_operation_types").unwrap()).unwrap();
        assert_eq!(comp_op, op_types);
    })
}

/// Test copy and deepcopy functions of Circuit
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 3);

        let copy_circ = circuit.call_method0("__copy__").unwrap();
        let deepcopy_circ = circuit.call_method1("__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = circuit;

        let comparison_copy = bool::extract_bound(
            &copy_circ
                .call_method1("__eq__", (&copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_circ
                .call_method1("__eq__", (&copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test qoqo_versions function of Circuit
#[test]
fn test_qoqo_versions() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 3);
        let mut rsplit = ROQOQO_VERSION.split('.').take(2);
        let mut qsplit = QOQO_VERSION.split('.').take(2);
        let rver = format!(
            "{}.{}",
            rsplit.next().expect("ROQOQO_VERSION badly formatted"),
            rsplit.next().expect("ROQOQO_VERSION badly formatted")
        );
        let qver = format!(
            "{}.{}",
            qsplit.next().expect("QOQO_VERSION badly formatted"),
            qsplit.next().expect("QOQO_VERSION badly formatted")
        );

        let comparison_copy: Vec<String> =
            Vec::extract_bound(&circuit.call_method0("_qoqo_versions").unwrap()).unwrap();
        assert_eq!(comparison_copy, vec![rver.as_str(), qver.as_str()]);
    })
}

/// Test to_ and from_bincode functions of Circuit
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 3);

        // testing 'to_bincode' and 'from_bincode' functions
        let serialised = circuit.call_method0("to_bincode").unwrap();
        let new = new_circuit(py);
        let deserialised = new.call_method1("from_bincode", (&serialised,)).unwrap();
        let comparison =
            bool::extract_bound(&deserialised.call_method1("__eq__", (&circuit,)).unwrap())
                .unwrap();
        assert!(comparison);

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_bincode", (bincode::serialize(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        // testing that 'from_bincode' can be called directly on a circuit (python staticmethod)
        let circuit_type = py.get_type::<CircuitWrapper>();
        let deserialised_py = circuit_type
            .call_method1("from_bincode", (&serialised,))
            .unwrap();

        let comparison =
            bool::extract_bound(&deserialised_py.call_method1("__eq__", (&circuit,)).unwrap())
                .unwrap();
        assert!(comparison);
    })
}

#[test]
fn test_value_error_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_type = py.get_type::<PauliZProductInputWrapper>();
        let binding = input_type.call1((3, false)).unwrap();
        let input = binding.downcast::<PauliZProductInputWrapper>().unwrap();
        let tmp_vec: Vec<usize> = Vec::new();
        let _ = input
            .call_method1("add_pauliz_product", ("ro", tmp_vec))
            .unwrap();

        let circs: Vec<CircuitWrapper> = vec![CircuitWrapper::new()];

        let br_type = py.get_type::<PauliZProductWrapper>();
        let binding = br_type
            .call1((Some(CircuitWrapper::new()), circs, input))
            .unwrap();
        let br = binding.downcast::<PauliZProductWrapper>().unwrap();

        let new_br = br;
        let serialised = br.call_method0("to_json").unwrap();
        let binding = &new_br.call_method1("from_json", (&serialised,)).unwrap();
        let deserialised = binding.downcast::<PauliZProductWrapper>().unwrap();

        let new = new_circuit(py);
        let deserialised_error = new.call_method1("from_bincode", (deserialised,));
        assert!(deserialised_error.is_err());
    })
}

/// Test to_ and from_json functions of Circuit
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 3);
        add_circuit_measurement_operation(&circuit);

        // testing 'from_json' and 'to_json' functions
        let serialised = &circuit.call_method0("to_json").unwrap();
        let new = new_circuit(py);
        let deserialised = new.call_method1("from_json", (serialised,)).unwrap();

        let comparison =
            bool::extract_bound(&deserialised.call_method1("__eq__", (&circuit,)).unwrap())
                .unwrap();
        assert!(comparison);

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string("fails").unwrap(),));
        assert!(deserialised_error.is_err());

        let deserialised_error =
            new.call_method1("from_json", (serde_json::to_string(&vec![0]).unwrap(),));
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        // testing that 'from_json' can be called directly on a circuit (python staticmethod)
        let circuit_type = py.get_type::<CircuitWrapper>();
        let deserialised_py = circuit_type
            .call_method1("from_json", (serialised,))
            .unwrap();

        let comparison =
            bool::extract_bound(&deserialised_py.call_method1("__eq__", (circuit,)).unwrap())
                .unwrap();
        assert!(comparison);
    })
}

/// Test json_schema function of Circuit
#[cfg(feature = "json_schema")]
#[test]
fn test_json_schema() {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 4);

        let schema: String =
            String::extract_bound(&circuit.call_method0("json_schema").unwrap()).unwrap();
        let rust_schema = serde_json::to_string_pretty(&schemars::schema_for!(Circuit)).unwrap();
        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&circuit.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&circuit.call_method0("min_supported_version").unwrap()).unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.0.0");
    });
}

///  Test single index set and write access using "get" function
#[test]
fn test_single_index_access_get() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 3);

        // test access at index 1
        let comp_op = circuit.call_method1("get", (1,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(RotateX::new(
            1,
            CalculatorFloat::from(1),
        )))
        .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(comparison);

        // test setting new operation at index 1
        let operation2 = convert_operation_to_pyobject(Operation::from(RotateX::new(
            1,
            CalculatorFloat::from(10),
        )))
        .unwrap();

        circuit
            .call_method1("__setitem__", (1, operation2.clone_ref(py)))
            .unwrap();

        let comp_op = circuit.call_method1("get", (1,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation2,)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = circuit.call_method1("get", (20,));
        assert!(comparison.is_err());
    })
}

/// Test get_slice property of Circuit
#[test]
fn test_get_slice() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 4);

        let circuit2 = new_circuit(py);
        populate_circuit_rotatex(py, &circuit2, 1, 3);

        let circuit3 = new_circuit(py);
        populate_circuit_rotatex(py, &circuit3, 0, 3);

        let circuit4 = new_circuit(py);
        populate_circuit_rotatex(py, &circuit4, 2, 4);

        let new_circuit_slice = circuit.call_method1("get_slice", (1, 2)).unwrap();
        let comparison = bool::extract_bound(
            &new_circuit_slice
                .call_method1("__eq__", (circuit2,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let new_circuit_slice = circuit
            .call_method1("get_slice", (Option::<usize>::None, 2))
            .unwrap();
        let comparison = bool::extract_bound(
            &new_circuit_slice
                .call_method1("__eq__", (circuit3,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let new_circuit_slice = circuit
            .call_method1("get_slice", (2, Option::<usize>::None))
            .unwrap();
        let comparison = bool::extract_bound(
            &new_circuit_slice
                .call_method1("__eq__", (circuit4,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        match circuit.call_method1("get_slice", (1, 20)) {
            Err(x) => assert!(x.is_instance_of::<PyIndexError>(py)),
            _ => panic!("Wrong error"),
        }

        match circuit.call_method1("get_slice", (2, 1)) {
            Err(x) => assert!(x.is_instance_of::<PyIndexError>(py)),
            _ => panic!("Wrong error"),
        }

        match circuit.call_method1("get_slice", (21, 22)) {
            Err(x) => assert!(x.is_instance_of::<PyIndexError>(py)),
            _ => panic!("Wrong error"),
        }
    })
}

/// Test definitions function of Circuit
#[test]
fn test_definitions() {
    let added_op1 = Operation::from(DefinitionBit::new("ro".to_string(), 1, false));
    let added_op2 = Operation::from(InputSymbolic::new("test".to_string(), 1.0));
    let operation1 = convert_operation_to_pyobject(added_op1).unwrap();
    let operation2 = convert_operation_to_pyobject(added_op2).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation1.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (operation2.clone_ref(py),))
            .unwrap();

        let comp_op = circuit.call_method0("definitions").unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1("__eq__", (vec![operation1, operation2],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison)
    })
}

/// Test operations function of Circuit
#[test]
fn test_operations() {
    let added_op1 = Operation::from(RotateX::new(0, CalculatorFloat::from("theta")));
    let added_op2 = Operation::from(RotateZ::new(0, CalculatorFloat::from(0.0)));
    let operation1 = convert_operation_to_pyobject(added_op1).unwrap();
    let operation2 = convert_operation_to_pyobject(added_op2).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation1.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (operation2.clone_ref(py),))
            .unwrap();

        let comp_op = circuit.call_method0("operations").unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1("__eq__", (vec![operation1, operation2],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison)
    })
}

/// Test filter_by_tag function of Circuit
#[test]
fn test_filter_by_tag() {
    let added_op1 = Operation::from(DefinitionBit::new("ro".to_string(), 1, false));
    let added_op2 = Operation::from(InputSymbolic::new("test".to_string(), 1.0));
    let operation1 = convert_operation_to_pyobject(added_op1).unwrap();
    let operation2 = convert_operation_to_pyobject(added_op2).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation1.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (operation2.clone_ref(py),))
            .unwrap();
        populate_circuit_rotatex(py, &circuit, 0, 2);

        let comp_op = circuit
            .call_method1("filter_by_tag", ("Definition",))
            .unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1("__eq__", (vec![operation1, operation2],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let rotatex_type = py.get_type::<RotateXWrapper>();
        let binding = rotatex_type.call1((0, 0)).unwrap();
        let rotatex_0 = binding.downcast::<RotateXWrapper>().unwrap();
        let binding = rotatex_type.call1((1, 1)).unwrap();
        let rotatex_1 = binding.downcast::<RotateXWrapper>().unwrap();

        let comp_op = circuit.call_method1("filter_by_tag", ("RotateX",)).unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1("__eq__", (vec![rotatex_0, rotatex_1],))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison)
    })
}

/// Test add function
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(0))); "RotateX float")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(1.3))); "RotateZ float")]
#[test_case(Operation::from(SingleQubitGate::new(2, CalculatorFloat::from(0), CalculatorFloat::from("var"), CalculatorFloat::from(0), CalculatorFloat::from(0), CalculatorFloat::from(0), )); "SingleQubitGate float")]
fn test_circuit_add_function(added_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(added_operation).unwrap();
        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation.clone_ref(py),))
            .unwrap();

        let comp_op = circuit.call_method1("__getitem__", (0,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = circuit.call_method1("add", (vec!["fails"],));
        assert!(comparison.is_err());
    })
}

/// Test the __repr__ and __format__ functions
#[test]
fn test_format_repr() {
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 2);
        let format_repr = "RotateX(RotateX { qubit: 0, theta: Float(0.0) })\nRotateX(RotateX { qubit: 1, theta: Float(1.0) })\n";

        let to_format = circuit.call_method1("__format__", ("",)).unwrap();
        let format_op: String = String::extract_bound(&to_format).unwrap();

        let to_repr = circuit.call_method0("__repr__").unwrap();
        let repr_op: String = String::extract_bound(&to_repr).unwrap();

        assert_eq!(format_op, format_repr);
        assert_eq!(repr_op, format_repr);
    })
}

/// Test fmt::Debug for OperationIteratorWrapper
#[test]
fn test_fmt_circuititerator() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_circuit = new_circuit(py);
        populate_circuit_rotatex(py, &new_circuit, 0, 2);
        let binding = &new_circuit.call_method0("__iter__").unwrap();
        let circuit_iter = binding.downcast::<OperationIteratorWrapper>().unwrap();

        let fmt = "OperationIteratorWrapper { internal: OperationIterator { definition_iter: IntoIter([]), operation_iter: IntoIter([RotateX(RotateX { qubit: 0, theta: Float(0.0) }), RotateX(RotateX { qubit: 1, theta: Float(1.0) })]) } }";

        assert_eq!(format!("{:?}", circuit_iter.borrow()), fmt);
    })
}

/// Test the __richcmp__ function
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit_one = new_circuit(py);
        populate_circuit_rotatex(py, &circuit_one, 0, 2);
        let circuit_two = new_circuit(py);
        populate_circuit_rotatex(py, &circuit_two, 0, 3);
        let operation1 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();

        let comparison =
            bool::extract_bound(&circuit_one.call_method1("__eq__", (&circuit_two,)).unwrap())
                .unwrap();
        assert!(!comparison);
        let comparison = bool::extract_bound(
            &circuit_one
                .call_method1("__eq__", (operation1.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison =
            bool::extract_bound(&circuit_one.call_method1("__ne__", (&circuit_two,)).unwrap())
                .unwrap();
        assert!(comparison);
        let comparison = bool::extract_bound(
            &circuit_one
                .call_method1("__ne__", (operation1.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = circuit_one.call_method1("__ge__", (operation1,));
        assert!(comparison.is_err());
    })
}

#[test]
fn test_circuit_iadd_magic_method() {
    let added_op1 = Operation::from(DefinitionBit::new("ro".to_string(), 1, false));
    let added_op2 = Operation::from(RotateX::new(0, CalculatorFloat::from(1.0)));
    let added_op3 = Operation::from(PauliX::new(0));
    let operation1 = convert_operation_to_pyobject(added_op1).unwrap();
    let operation2 = convert_operation_to_pyobject(added_op2).unwrap();
    let operation3 = convert_operation_to_pyobject(added_op3).unwrap();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let added_circuit = new_circuit(py);
        added_circuit
            .call_method1("add", (operation3.clone_ref(py),))
            .unwrap();

        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation1.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("__iadd__", (operation2.clone_ref(py),))
            .unwrap();
        circuit.call_method1("__iadd__", (added_circuit,)).unwrap();

        let comp_op = circuit.call_method1("__getitem__", (0,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation1,)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = circuit.call_method1("__getitem__", (1,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation2,)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = circuit.call_method1("__getitem__", (2,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation3,)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = circuit.call_method1("__iadd__", (vec!["fails"],));
        assert!(comparison.is_err());
    })
}

#[test]
fn test_circuit_add_magic_method() {
    pyo3::prepare_freethreaded_python();
    let added_op1 = Operation::from(DefinitionBit::new("ro".to_string(), 1, false));
    let added_op2 = Operation::from(RotateX::new(0, CalculatorFloat::from(1.0)));
    let added_op3 = Operation::from(PauliX::new(0));
    let operation1 = convert_operation_to_pyobject(added_op1).unwrap();
    let operation2 = convert_operation_to_pyobject(added_op2).unwrap();
    let operation3 = convert_operation_to_pyobject(added_op3).unwrap();
    Python::with_gil(|py| {
        let added_circuit = new_circuit(py);
        added_circuit
            .call_method1("add", (operation3.clone_ref(py),))
            .unwrap();

        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (operation1.clone_ref(py),))
            .unwrap();
        let circuit1 = circuit
            .call_method1("__add__", (operation2.clone_ref(py),))
            .unwrap();
        let circuit2 = circuit1.call_method1("__add__", (added_circuit,)).unwrap();

        let comp_op = circuit2.call_method1("__getitem__", (0,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation1,)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = circuit2.call_method1("__getitem__", (1,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation2,)).unwrap()).unwrap();
        assert!(comparison);

        let comp_op = circuit2.call_method1("__getitem__", (2,)).unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation3,)).unwrap()).unwrap();
        assert!(comparison);

        let comparison = circuit.call_method1("__add__", (vec!["fails"],));
        assert!(comparison.is_err());
    })
}

/// Test iterator interface of Circuit
#[test]
fn test_iter() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_circuit = new_circuit(py);
        populate_circuit_rotatex(py, &new_circuit, 0, 3);

        let rotatex_type = py.get_type::<RotateXWrapper>();
        let binding = rotatex_type.call1((0, 0)).unwrap();
        let new_rotatex_0 = binding.downcast::<RotateXWrapper>().unwrap();
        let binding = rotatex_type.call1((1, 1)).unwrap();
        let new_rotatex_1 = binding.downcast::<RotateXWrapper>().unwrap();
        let binding = rotatex_type.call1((2, 2)).unwrap();
        let new_rotatex_2 = binding.downcast::<RotateXWrapper>().unwrap();
        let comparison_vec = [new_rotatex_0, new_rotatex_1, new_rotatex_2];

        let binding = &new_circuit.call_method0("__iter__").unwrap();
        let t = binding.downcast::<OperationIteratorWrapper>().unwrap();

        let range = 0_usize..3_usize;
        for i in range {
            let comp_op = t.call_method0("__next__").unwrap();
            let comparison = bool::extract_bound(
                &comp_op
                    .call_method1("__eq__", (comparison_vec[i],))
                    .unwrap(),
            )
            .unwrap();
            assert!(comparison)
        }

        let iter_op = t.call_method0("__iter__").unwrap();
        let comparison =
            bool::extract_bound(&iter_op.call_method1("__eq__", (t,)).unwrap()).unwrap();
        assert!(comparison);
    })
}

/// Test the __len__ function
#[test]
fn test_len() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 5);

        let len_op: usize =
            usize::extract_bound(&circuit.call_method0("__len__").unwrap()).unwrap();
        assert_eq!(len_op, 5_usize);
    })
}

///  Test single index set and write access using "__getitem__" function
#[test]
fn test_single_index_access_getitem() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        populate_circuit_rotatex(py, &circuit, 0, 3);

        // test access at index 1
        let comp_op = circuit.call_method1("__getitem__", (1,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(RotateX::new(
            1,
            CalculatorFloat::from(1),
        )))
        .unwrap();
        let comparison =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(comparison);

        // test setting new operation at index 1
        let operation2 = convert_operation_to_pyobject(Operation::from(RotateX::new(
            1,
            CalculatorFloat::from(10),
        )))
        .unwrap();

        circuit
            .call_method1("__setitem__", (1, operation2.clone_ref(py)))
            .unwrap();

        let comp_op = circuit.call_method1("__getitem__", (1,)).unwrap();
        let comparison = bool::extract_bound(
            &comp_op
                .call_method1("__eq__", (operation2.clone_ref(py),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = circuit.call_method1("__setitem__", (1, vec!["fails"]));
        assert!(comparison.is_err());

        let comparison = circuit.call_method1("__getitem__", (20,));
        assert!(comparison.is_err());

        let comparison = circuit.call_method1("__setitem__", (3, operation2));
        assert!(comparison.is_err());
    })
}

#[test]
fn test_convert_into_circuit() {
    let added_op = Operation::from(PauliX::new(0));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(added_op).unwrap();

        let added_circuit = new_circuit(py);
        let comparison = added_circuit.call_method1("convert_into_circuit", (operation,));
        assert!(comparison.is_err());
    })
}

/// Test function overrotate() for Circuit
#[test]
#[cfg(feature = "overrotate")]
fn test_circuit_overrotate() {
    use qoqo::operations::{PragmaOverrotationWrapper, RotateYWrapper};
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);

        let overrotation_type = py.get_type::<PragmaOverrotationWrapper>();
        let _new_overrotation_1 = overrotation_type
            .call1(("RotateY".to_string(), vec![1], 20.0, 30.0))
            .unwrap();
        circuit.call_method1("add", (_new_overrotation_1,)).unwrap();

        let rotatex_type = py.get_type::<RotateXWrapper>();
        let new_rotatex_0 = rotatex_type.call1((0, 0.0)).unwrap();
        circuit.call_method1("add", (new_rotatex_0,)).unwrap();

        let rotatey_type = py.get_type::<RotateYWrapper>();
        let new_rotatey_0 = rotatey_type.call1((0, 1.0)).unwrap();
        circuit.call_method1("add", (new_rotatey_0,)).unwrap();

        let new_rotatey_1 = rotatey_type.call1((1, 2.0)).unwrap();
        circuit.call_method1("add", (new_rotatey_1,)).unwrap();
        let new_rotatey_2 = rotatey_type.call1((1, 3.0)).unwrap();
        circuit.call_method1("add", (new_rotatey_2,)).unwrap();

        let binding = &circuit.call_method0("overrotate").unwrap();
        let circuit_overrotated = binding.downcast::<CircuitWrapper>().unwrap();

        assert_ne!(
            format!("{:?}", circuit),
            format!("{:?}", circuit_overrotated)
        );

        let comparison = bool::extract_bound(
            &circuit
                .as_ref()
                .call_method1("__ne__", (circuit_overrotated,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}
