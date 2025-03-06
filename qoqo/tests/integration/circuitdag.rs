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

use qoqo::operations::convert_operation_to_pyobject;
use qoqo::{CircuitDagWrapper, CircuitWrapper, QOQO_VERSION};

use roqoqo::operations::*;
use roqoqo::ROQOQO_VERSION;

// Helper functions
fn new_circuitdag(py: Python) -> Bound<CircuitDagWrapper> {
    let circuitdag_type = py.get_type::<CircuitDagWrapper>();
    circuitdag_type
        .call0()
        .unwrap()
        .downcast::<CircuitDagWrapper>()
        .unwrap()
        .to_owned()
}

fn new_circuit(py: Python) -> Bound<CircuitWrapper> {
    let circuit_type = py.get_type::<CircuitWrapper>();
    circuit_type
        .call0()
        .unwrap()
        .downcast::<CircuitWrapper>()
        .unwrap()
        .to_owned()
}

/// Test default
#[test]
fn test_default() {
    pyo3::prepare_freethreaded_python();
    let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (operation.clone_ref(py),))
            .unwrap();
        let circuitdag_wrapper = dag.extract::<CircuitDagWrapper>();

        let helper_ne: bool = CircuitDagWrapper::default() != circuitdag_wrapper.unwrap();
        assert!(helper_ne);
        let helper_eq: bool = CircuitDagWrapper::default() == CircuitDagWrapper::new(100, 300);
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
        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_front", (cnot_01.clone_ref(py),))
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
        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_0.clone_ref(py),))
            .unwrap();

        let comp_op = dag.call_method1("get", (0,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();

        let helper1 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(helper1);

        let comp_op = dag.call_method1("get", (1,)).unwrap();
        let operation = convert_operation_to_pyobject(Operation::from(PauliY::new(0))).unwrap();

        let helper2 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
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
        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_0.clone_ref(py),))
            .unwrap();
        let empty_dag = new_circuitdag(py);

        let dag_copy = dag.call_method0("__copy__").unwrap();
        let empty_dag_copy = empty_dag.call_method0("__copy__").unwrap();

        let full_dag_comparison =
            bool::extract_bound(&dag_copy.call_method1("__eq__", (dag,)).unwrap()).unwrap();
        assert!(full_dag_comparison);

        let empty_dag_comparison =
            bool::extract_bound(&empty_dag_copy.call_method1("__eq__", (empty_dag,)).unwrap())
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
        dag1.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        dag1.call_method1("add_to_back", (pauliy_0.clone_ref(py),))
            .unwrap();
        let dag2 = new_circuitdag(py);
        dag2.call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();
        dag2.call_method1("add_to_front", (paulix_0.clone_ref(py),))
            .unwrap();
        dag2.call_method1("add_to_back", (pauliy_0.clone_ref(py),))
            .unwrap();

        let comparison =
            bool::extract_bound(&dag1.call_method1("__eq__", (&dag2,)).unwrap()).unwrap();
        assert!(!comparison);
        let comparison =
            bool::extract_bound(&dag1.call_method1("__ne__", (&dag2,)).unwrap()).unwrap();
        assert!(comparison);
        let comparison = dag1.call_method1("__ge__", (dag2,));
        assert!(comparison.is_err());
    })
}

/// Test qoqo_versions function of Circuit
#[test]
fn test_qoqo_versions() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
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
            Vec::extract_bound(&dag.call_method0("_qoqo_versions").unwrap()).unwrap();
        assert_eq!(comparison_copy, vec![rver.as_str(), qver.as_str()]);
    })
}

/// Test to_ and from_bincode functions of CircuitDag
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);
        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();

        // testing 'to_bincode' and 'from_bincode' functions
        let serialised = dag.call_method0("to_bincode").unwrap();
        let new = new_circuitdag(py);
        let deserialised = new.call_method1("from_bincode", (&serialised,)).unwrap();
        let comparison =
            bool::extract_bound(&deserialised.call_method1("__eq__", (&dag,)).unwrap()).unwrap();
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

        // testing that 'from_bincode' can be called directly on a circuitdag (python staticmethod)
        let circuitdag_type = py.get_type::<CircuitDagWrapper>();
        let deserialised_py = circuitdag_type
            .call_method1("from_bincode", (&serialised,))
            .unwrap();

        let comparison =
            bool::extract_bound(&deserialised_py.call_method1("__eq__", (dag,)).unwrap()).unwrap();
        assert!(comparison);
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
        circuit
            .call_method1("add", (paulix_0.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (pauliy_0.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (cnot_01.clone_ref(py),))
            .unwrap();

        let dag = new_circuitdag(py);
        let binding = dag.call_method1("from_circuit", (circuit,)).unwrap();
        let dag = binding.downcast::<CircuitDagWrapper>().unwrap();

        let comp_op = dag.call_method1("get", (0,)).unwrap();
        let helper1 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (paulix_0,)).unwrap()).unwrap();
        assert!(helper1);

        let comp_op = dag.call_method1("get", (1,)).unwrap();
        let helper2 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (pauliy_0,)).unwrap()).unwrap();
        assert!(helper2);

        let comp_op = dag.call_method1("get", (2,)).unwrap();
        let helper3 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (cnot_01,)).unwrap()).unwrap();
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
        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_0.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();

        let circuit = new_circuit(py);
        circuit
            .call_method1("add", (paulix_0.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (pauliy_0.clone_ref(py),))
            .unwrap();
        circuit
            .call_method1("add", (cnot_01.clone_ref(py),))
            .unwrap();

        let new_circuit = dag.call_method0("to_circuit").unwrap();

        let comp_op = new_circuit.call_method1("get", (0,)).unwrap();
        let helper1 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (paulix_0,)).unwrap()).unwrap();
        assert!(helper1);

        let comp_op = new_circuit.call_method1("get", (1,)).unwrap();
        let helper2 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (pauliy_0,)).unwrap()).unwrap();
        assert!(helper2);

        let comp_op = new_circuit.call_method1("get", (2,)).unwrap();
        let helper3 =
            bool::extract_bound(&comp_op.call_method1("__eq__", (cnot_01,)).unwrap()).unwrap();
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

        let a = &dag
            .call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        let b = &dag
            .call_method1("add_to_back", (pauliz_0.clone_ref(py),))
            .unwrap();
        let c = &dag
            .call_method1("add_to_back", (pauliy_1.clone_ref(py),))
            .unwrap();
        let d = &dag
            .call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();
        let e = &dag
            .call_method1("add_to_back", (cpauliz_12.clone_ref(py),))
            .unwrap();

        let comp = dag
            .call_method1("execution_blocked", (vec![a, b, c], d))
            .unwrap();
        assert_eq!(comp.len().unwrap(), 0);

        let comp = dag
            .call_method1("execution_blocked", (vec![a, b, c], e))
            .unwrap();
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (vec![d],)).unwrap()).unwrap();
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
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (vec![a],)).unwrap()).unwrap();
        assert!(helper);

        let comp = dag
            .call_method1("execution_blocked", (Vec::<usize>::new(), e))
            .unwrap();
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (vec![a, b, c, d],)).unwrap())
                .unwrap();
        assert!(helper);
    })
}

#[test]
fn test_blocking_predecessors() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_1 = convert_operation_to_pyobject(Operation::from(PauliY::new(1))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    let pauliz_0 = convert_operation_to_pyobject(Operation::from(PauliZ::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        let a = &dag
            .call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        let b = &dag
            .call_method1("add_to_back", (pauliz_0.clone_ref(py),))
            .unwrap();
        let c = &dag
            .call_method1("add_to_back", (pauliy_1.clone_ref(py),))
            .unwrap();
        let d = &dag
            .call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();

        let comp = dag
            .call_method1("blocking_predecessors", (vec![a, b, c], d))
            .unwrap();
        assert_eq!(comp.len().unwrap(), 0);

        let comp = dag
            .call_method1("blocking_predecessors", (vec![a], b))
            .unwrap();
        assert_eq!(comp.len().unwrap(), 0);

        let comp = dag
            .call_method1("blocking_predecessors", (Vec::<usize>::new(), c))
            .unwrap();
        assert_eq!(comp.len().unwrap(), 0);

        let comp = dag
            .call_method1("blocking_predecessors", (vec![a, b], d))
            .unwrap();
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (vec![c],)).unwrap()).unwrap();
        assert!(helper);
    })
}

#[test]
fn test_new_front_layer() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let pauliy_1 = convert_operation_to_pyobject(Operation::from(PauliY::new(1))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    let pauliz_0 = convert_operation_to_pyobject(Operation::from(PauliZ::new(0))).unwrap();
    let cpauliz_12 =
        convert_operation_to_pyobject(Operation::from(ControlledPauliZ::new(1, 2))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        let a = &dag
            .call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        let b = &dag
            .call_method1("add_to_back", (pauliz_0.clone_ref(py),))
            .unwrap();
        let c = &dag
            .call_method1("add_to_back", (pauliy_1.clone_ref(py),))
            .unwrap();
        let d = &dag
            .call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();
        let e = &dag
            .call_method1("add_to_back", (cpauliz_12.clone_ref(py),))
            .unwrap();

        assert!(dag
            .call_method1("new_front_layer", (vec![a, b, c], vec![d], e))
            .is_err());

        let comp = dag
            .call_method1("new_front_layer", (vec![a, c], vec![b.clone()], b))
            .unwrap();
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (vec![d],)).unwrap()).unwrap();
        assert!(helper);

        let comp = dag
            .call_method1("new_front_layer", (vec![a], vec![b.clone()], b))
            .unwrap();
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (vec![b],)).unwrap()).unwrap();
        assert!(helper);

        let comp = dag
            .call_method1("new_front_layer", (vec![a, b, c], vec![d.clone()], d))
            .unwrap();
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (vec![e],)).unwrap()).unwrap();
        assert!(helper);

        let comp = dag
            .call_method1("new_front_layer", (vec![a, b, c, d], vec![e.clone()], e))
            .unwrap();
        let helper =
            bool::extract_bound(&comp.call_method1("__eq__", (Vec::<usize>::new(),)).unwrap())
                .unwrap();
        assert!(helper);
    })
}

#[test]
fn test_parallel_blocks() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let paulix_1 = convert_operation_to_pyobject(Operation::from(PauliX::new(1))).unwrap();
    let pauliy_1 = convert_operation_to_pyobject(Operation::from(PauliY::new(1))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    let pauliz_0 = convert_operation_to_pyobject(Operation::from(PauliZ::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliz_0.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (pauliy_1.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (paulix_1.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();

        let par_bl = dag.call_method0("parallel_blocks").unwrap();

        let vec0 = par_bl.get_item(0).unwrap();
        if let Ok(el) = vec0.call0() {
            let helper1 = bool::extract_bound(
                &el.get_item(0)
                    .unwrap()
                    .call_method1("__eq__", (0,))
                    .unwrap(),
            )
            .unwrap();
            let helper2 = bool::extract_bound(
                &el.get_item(0)
                    .unwrap()
                    .call_method1("__eq__", (2,))
                    .unwrap(),
            )
            .unwrap();
            assert!(helper1 || helper2);
        }

        let vec1 = par_bl.get_item(1).unwrap();
        if let Ok(el) = vec1.call0() {
            let helper1 = bool::extract_bound(
                &el.get_item(0)
                    .unwrap()
                    .call_method1("__eq__", (1,))
                    .unwrap(),
            )
            .unwrap();
            let helper2 = bool::extract_bound(
                &el.get_item(0)
                    .unwrap()
                    .call_method1("__eq__", (3,))
                    .unwrap(),
            )
            .unwrap();
            assert!(helper1 || helper2);
        }

        let vec2 = par_bl.get_item(2).unwrap();
        if let Ok(el) = vec2.call0() {
            let helper1 = bool::extract_bound(
                &el.get_item(0)
                    .unwrap()
                    .call_method1("__eq__", (4,))
                    .unwrap(),
            )
            .unwrap();
            assert!(helper1);
        }
    })
}

#[test]
fn test_successors() {
    pyo3::prepare_freethreaded_python();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let paulix_1 = convert_operation_to_pyobject(Operation::from(PauliX::new(1))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        let a = dag
            .call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();
        let b = dag
            .call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();
        let c = dag
            .call_method1("add_to_back", (paulix_1.clone_ref(py),))
            .unwrap();

        let vec = dag.call_method1("successors", (a,)).unwrap();

        let len_op: usize = usize::extract_bound(&vec.call_method0("__len__").unwrap()).unwrap();
        assert_eq!(len_op, 2);

        let el = vec.call_method1("__getitem__", (0,)).unwrap();
        let comp = bool::extract_bound(&el.call_method1("__eq__", (c,)).unwrap()).unwrap();
        assert!(comp);

        let el = vec.call_method1("__getitem__", (1,)).unwrap();
        let comp = bool::extract_bound(&el.call_method1("__eq__", (b,)).unwrap()).unwrap();
        assert!(comp);
    })
}

#[test]
fn test_getters_commuting_operations() {
    pyo3::prepare_freethreaded_python();
    let commut_op = convert_operation_to_pyobject(Operation::from(
        PragmaSetNumberOfMeasurements::new(3, "ro".to_string()),
    ))
    .unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        let commut_vec = dag.call_method0("commuting_operations").unwrap();
        assert_eq!(commut_vec.len().unwrap(), 0);

        dag.call_method1("add_to_back", (commut_op.clone_ref(py),))
            .unwrap();

        let commut_vec = dag.call_method0("commuting_operations").unwrap();
        assert_eq!(commut_vec.len().unwrap(), 1);
    })
}

#[test]
fn test_getters_parallel_blocks() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    let paulix_1 = convert_operation_to_pyobject(Operation::from(PauliX::new(1))).unwrap();
    let cnot_01 = convert_operation_to_pyobject(Operation::from(CNOT::new(0, 1))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        let fpb = dag.call_method0("first_parallel_block").unwrap();
        let lpb = dag.call_method0("last_parallel_block").unwrap();
        assert_eq!(fpb.len().unwrap(), 0);
        assert_eq!(lpb.len().unwrap(), 0);

        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();

        let fpb = dag.call_method0("first_parallel_block").unwrap();
        let lpb = dag.call_method0("last_parallel_block").unwrap();
        assert_eq!(fpb.len().unwrap(), 1);
        assert_eq!(lpb.len().unwrap(), 1);

        dag.call_method1("add_to_back", (cnot_01.clone_ref(py),))
            .unwrap();
        dag.call_method1("add_to_front", (paulix_1,)).unwrap();

        let fpb = dag.call_method0("first_parallel_block").unwrap();
        let lpb = dag.call_method0("last_parallel_block").unwrap();
        assert_eq!(fpb.len().unwrap(), 2);
        assert_eq!(lpb.len().unwrap(), 1);
    })
}

#[test]
fn test_getters_operation_involving_qubit() {
    pyo3::prepare_freethreaded_python();
    let paulix_0 = convert_operation_to_pyobject(Operation::from(PauliX::new(0))).unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        let foiq = dag.call_method0("first_operation_involving_qubit").unwrap();
        let loiq = dag.call_method0("last_operation_involving_qubit").unwrap();
        assert_eq!(foiq.len().unwrap(), 0);
        assert_eq!(loiq.len().unwrap(), 0);

        dag.call_method1("add_to_back", (paulix_0.clone_ref(py),))
            .unwrap();

        let foiq = dag.call_method0("first_operation_involving_qubit").unwrap();
        let loiq = dag.call_method0("last_operation_involving_qubit").unwrap();
        assert_eq!(foiq.len().unwrap(), 1);
        assert_eq!(loiq.len().unwrap(), 1);
    })
}

#[test]
fn test_getters_operations_involving_classical() {
    pyo3::prepare_freethreaded_python();
    let meas =
        convert_operation_to_pyobject(Operation::from(MeasureQubit::new(0, "ro".to_string(), 0)))
            .unwrap();
    Python::with_gil(|py| {
        let dag = new_circuitdag(py);

        dag.call_method1("add_to_back", (meas,)).unwrap();

        let foic = dag
            .call_method0("first_operation_involving_classical")
            .unwrap();
        let loic = dag
            .call_method0("last_operation_involving_classical")
            .unwrap();
        assert_eq!(foic.len().unwrap(), 1);
        assert_eq!(loic.len().unwrap(), 1);
    })
}

#[test]
fn test_convert_into_circuitdag() {
    pyo3::prepare_freethreaded_python();
    let added_op = Operation::from(PauliX::new(0));
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(added_op).unwrap();

        let added_dag = new_circuitdag(py);
        let comparison = added_dag.call_method1("convert_into_circuitdag", (operation,));
        assert!(comparison.is_err());
    })
}
