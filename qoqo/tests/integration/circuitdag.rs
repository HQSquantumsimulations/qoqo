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

use qoqo::operations::convert_operation_to_pyobject;
use qoqo::{CircuitDagWrapper, CircuitWrapper};

use roqoqo::operations::*;

// Helper functions
fn new_circuitdag(py: Python) -> &PyCell<CircuitDagWrapper> {
    let circuitdag_type = py.get_type::<CircuitDagWrapper>();
    circuitdag_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CircuitDagWrapper>>()
        .unwrap()
}

fn new_circuit(py: Python) -> &PyCell<CircuitWrapper> {
    let circuit_type = py.get_type::<CircuitWrapper>();
    circuit_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CircuitWrapper>>()
        .unwrap()
}

/// Test default
#[test]
fn test_default() {
    pyo3::prepare_freethreaded_python();
    let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (operation.clone(),))
            .unwrap();
        let circuitdag_wrapper = dag.extract::<CircuitDagWrapper>();

        let helper_ne: bool = CircuitDagWrapper::default() != circuitdag_wrapper.unwrap();
        assert!(helper_ne);
        let helper_eq: bool = CircuitDagWrapper::default() == CircuitDagWrapper::new();
        assert!(helper_eq);
    })
}

/// Test add_to_back and add_to_front
#[test]
fn test_add_to() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        dag.call_method1("add_to_front", (cnot_01.clone(),))
            .unwrap();
        let _circuit_wrapper = dag.extract::<CircuitDagWrapper>();
    })
}

/// Test get
#[test]
fn test_get() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_0.clone(),))
            .unwrap();

        let comp_op = dag.call_method1("get", (0,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();

        let helper1 = bool::extract(comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(helper1);

        let comp_op = dag.call_method1("get", (1,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();

        let helper2 = bool::extract(comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(helper2);
    })
}

/// Test copy
#[test]
fn test_copy() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_0.clone(),))
            .unwrap();
        let empty_dag = new_circuitdag(py);

        let dag_copy = dag.call_method0("__copy__").unwrap();
        let empty_dag_copy = empty_dag.call_method0("__copy__").unwrap();

        let full_dag_comparison =
            bool::extract(dag_copy.call_method1("__eq__", (&(*dag),)).unwrap()).unwrap();
        assert!(full_dag_comparison);

        let empty_dag_comparison = bool::extract(
            empty_dag_copy
                .call_method1("__eq__", (&(*empty_dag),))
                .unwrap(),
        )
        .unwrap();
        assert!(empty_dag_comparison);
    })
}

/// Test __richcmp__
#[test]
fn test_richcmp() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    Python::with_gil(|py| {
        let dag1 = new_circuitdag(py);
        dag1.call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        dag1.call_method1("add_to_back", (pauliy_0.clone(),))
            .unwrap();
        let dag2 = new_circuitdag(py);
        dag2.call_method1("add_to_back", (cnot_01.clone(),))
            .unwrap();
        dag2.call_method1("add_to_front", (paulix_0.clone(),))
            .unwrap();
        dag2.call_method1("add_to_back", (pauliy_0.clone(),))
            .unwrap();

        let comparison = bool::extract(dag1.call_method1("__eq__", (dag2,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison = bool::extract(dag1.call_method1("__ne__", (dag2,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison = dag1.call_method1("__ge__", (dag2,));
        assert!(comparison.is_err());
    })
}

/// Test from_circuit
#[test]
fn test_from_circuit() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    Python::with_gil(|py| {
        let circuit = new_circuit(py);
        circuit.call_method1("add", (paulix_0.clone(),)).unwrap();
        circuit.call_method1("add", (pauliy_0.clone(),)).unwrap();
        circuit.call_method1("add", (cnot_01.clone(),)).unwrap();

        let dag = new_circuitdag(py);
        let dag = dag
            .call_method1("from_circuit", (circuit,))
            .unwrap()
            .cast_as::<PyCell<CircuitDagWrapper>>()
            .unwrap();

        let comp_op = dag.call_method1("get", (0,)).unwrap();
        let helper1 = bool::extract(comp_op.call_method1("__eq__", (paulix_0,)).unwrap()).unwrap();
        assert!(helper1);

        let comp_op = dag.call_method1("get", (1,)).unwrap();
        let helper2 = bool::extract(comp_op.call_method1("__eq__", (pauliy_0,)).unwrap()).unwrap();
        assert!(helper2);

        let comp_op = dag.call_method1("get", (2,)).unwrap();
        let helper3 = bool::extract(comp_op.call_method1("__eq__", (cnot_01,)).unwrap()).unwrap();
        assert!(helper3);
    })
}

#[test]
fn test_to_circuit() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_0 = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_0.clone(),))
            .unwrap();
        dag.call_method1("add_to_back", (cnot_01.clone(),)).unwrap();

        let circuit = new_circuit(py);
        circuit.call_method1("add", (paulix_0.clone(),)).unwrap();
        circuit.call_method1("add", (pauliy_0.clone(),)).unwrap();
        circuit.call_method1("add", (cnot_01.clone(),)).unwrap();

        let new_circuit = dag.call_method0("to_circuit").unwrap();

        let comp_op = new_circuit.call_method1("get", (0,)).unwrap();
        let helper1 = bool::extract(comp_op.call_method1("__eq__", (paulix_0,)).unwrap()).unwrap();
        assert!(helper1);

        let comp_op = new_circuit.call_method1("get", (1,)).unwrap();
        let helper2 = bool::extract(comp_op.call_method1("__eq__", (pauliy_0,)).unwrap()).unwrap();
        assert!(helper2);

        let comp_op = new_circuit.call_method1("get", (2,)).unwrap();
        let helper3 = bool::extract(comp_op.call_method1("__eq__", (cnot_01,)).unwrap()).unwrap();
        assert!(helper3);
    })
}

#[test]
fn test_execution_blocked() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_1 = convert_operation_to_pyobject(Operation::from(PauliY::new(1))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    let pauliz_0 = convert_operation_to_pyobject(Operation::from(PauliZ::new(0))).unwrap();
    let cpauliz_12 =
        convert_operation_to_pyobject(Operation::from(ControlledPauliZ::new(1, 2))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        let a = dag
            .call_method1("add_to_back", (paulix_0.clone(),))
            .unwrap();
        let b = dag
            .call_method1("add_to_back", (pauliz_0.clone(),))
            .unwrap();
        let c = dag
            .call_method1("add_to_back", (pauliy_1.clone(),))
            .unwrap();
        let d = dag.call_method1("add_to_back", (cnot_01.clone(),)).unwrap();
        let e = dag
            .call_method1("add_to_back", (cpauliz_12.clone(),))
            .unwrap();

        let comp = dag
            .call_method1("execution_blocked", (vec![a, b, c], d))
            .unwrap();
        assert_eq!(comp.len().unwrap(), 0);

        let comp = dag
            .call_method1("execution_blocked", (vec![a, b, c], e))
            .unwrap();
        let helper = bool::extract(comp.call_method1("__eq__", (vec![d],)).unwrap()).unwrap();
        assert!(helper);

        let comp = dag
            .call_method1("execution_blocked", (vec![a, b, c, d], e))
            .unwrap();
        assert_eq!(comp.len().unwrap(), 0);

        let comp = dag
            .call_method1("execution_blocked", (vec![d, e], a))
            .unwrap();
        assert_eq!(comp.len().unwrap(), 0);

        let comp = dag
            .call_method1("execution_blocked", (vec![d, e], b))
            .unwrap();
        let helper = bool::extract(comp.call_method1("__eq__", (vec![a],)).unwrap()).unwrap();
        assert!(helper);

        let comp = dag
            .call_method1("execution_blocked", (Vec::<usize>::new(), e))
            .unwrap();
        let helper =
            bool::extract(comp.call_method1("__eq__", (vec![a, b, c, d],)).unwrap()).unwrap();
        assert!(helper);
    })
}
