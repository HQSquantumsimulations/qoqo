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

use std::collections::HashSet;

use roqoqo::{operations::*, RoqoqoError};
use roqoqo::{Circuit, CircuitDag};

use test_case::test_case;

static DEFAULT_NODE_NUMBER: usize = 10;
static DEFAULT_EDGE_NUMBER: usize = 30;

/// Test adding an operation that doesn't involve qubits.
///
#[test_case(
    Operation::from(PragmaSetNumberOfMeasurements::new(3, "ro".to_string())),
    Operation::from(PragmaSetNumberOfMeasurements::new(5, "ti".to_string()))
)]
#[test_case(
    Operation::from(PragmaSetNumberOfMeasurements::new(4, "ri".to_string())),
    Operation::from(PragmaSetNumberOfMeasurements::new(1, "to".to_string()))
)]
fn add_operation_no_involved_qubits(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let back1 = dag.add_to_back(operation1.clone());

    assert!(operation1.involved_qubits() == InvolvedQubits::None);
    assert_eq!(dag.commuting_operations().get(0), back1.as_ref());

    dag.add_to_front(Operation::from(PauliY::new(0)));
    dag.add_to_back(Operation::from(CNOT::new(0, 1)));

    let front1 = dag.add_to_front(operation2.clone());

    assert_eq!(dag.commuting_operations().get(1), front1.as_ref());
}

#[test]
fn test_partial_eq() {
    let mut dag1: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);
    let mut dag2: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    dag1.add_to_back(Operation::from(PauliX::new(0)));
    dag2.add_to_back(Operation::from(PauliX::new(0)));
    dag1.add_to_back(Operation::from(PauliY::new(0)));
    dag2.add_to_back(Operation::from(PauliY::new(0)));
    dag1.add_to_back(Operation::from(CNOT::new(0, 1)));
    dag2.add_to_back(Operation::from(CNOT::new(0, 1)));

    assert_eq!(dag1, dag2);

    dag2.add_to_back(Operation::from(PauliZ::new(1)));

    assert_ne!(dag1, dag2);

    let mut dag1: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);
    let mut dag2: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    dag1.add_to_back(Operation::from(PauliX::new(0)));
    dag2.add_to_back(Operation::from(PauliY::new(0)));
    dag1.add_to_back(Operation::from(PauliY::new(0)));
    dag2.add_to_back(Operation::from(PauliX::new(0)));

    assert_ne!(dag1, dag2);
}

#[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliY::new(1)))]
#[test_case(Operation::from(PauliY::new(1)), Operation::from(PauliZ::new(2)))]
#[test_case(Operation::from(CNOT::new(0, 1)), Operation::from(PauliX::new(1)))]
#[test_case(Operation::from(PauliX::new(1)), Operation::from(CNOT::new(0, 1)))]
fn check_parallel_blocks_set(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);
    let mut inv_qubits_1: HashSet<usize> = HashSet::new();
    if let InvolvedQubits::Set(x) = operation1.involved_qubits() {
        for qubit in x {
            inv_qubits_1.insert(qubit);
        }
    }

    dag.add_to_back(operation1.clone());

    assert!(dag.last_parallel_block().len() == 1);
    assert!(dag.first_parallel_block().len() == 1);

    dag.add_to_front(operation2.clone());
    dag.add_to_back(operation1.clone());

    if inv_qubits_1.len() == 2 {
        assert!(dag.last_parallel_block().len() == 1);
    } else {
        assert!(dag.last_parallel_block().len() == 2);
    }
}

#[test_case(Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None)))]
#[test_case(Operation::from(PragmaRepeatedMeasurement::new(String::from("ri"), 2, None)))]
fn check_parallel_blocks_all(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    assert!(dag.last_parallel_block().len() == 0);
    assert!(dag.first_parallel_block().len() == 0);

    dag.add_to_back(operation.clone());

    assert!(dag.first_parallel_block().len() == 1);
    assert!(dag.last_parallel_block().len() == 1);

    dag.add_to_front(operation.clone());

    assert!(dag.first_parallel_block().len() == 1);
    assert!(dag.last_parallel_block().len() == 1);
}

#[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliY::new(1)))]
#[test_case(Operation::from(PauliY::new(1)), Operation::from(PauliZ::new(2)))]
#[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliX::new(2)))]
fn check_parallel_blocks_mixed(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    dag.add_to_front(operation1);

    assert!(dag.first_parallel_block().len() == 1);
    assert!(dag.last_parallel_block().len() == 1);

    dag.add_to_back(Operation::from(PragmaRepeatedMeasurement::new(
        String::from("ro"),
        1,
        None,
    )));
    dag.add_to_front(operation2);

    assert!(dag.last_parallel_block().len() == 1);
    assert!(dag.first_parallel_block().len() == 2);
}

#[test_case(Operation::from(PauliX::new(0)))]
#[test_case(Operation::from(CNOT::new(0, 1)))]
fn check_operation_involving_qubits_set(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    dag.add_to_front(Operation::from(PauliZ::new(0)));
    dag.add_to_back(Operation::from(CNOT::new(0, 1)));

    assert!(
        dag.last_operation_involving_qubit().get(&0)
            == dag.last_operation_involving_qubit().get(&1)
    );

    let ind = dag.add_to_back(operation.clone());

    if let InvolvedQubits::Set(qubits) = operation.involved_qubits() {
        for qubit in qubits {
            assert!(dag.last_operation_involving_qubit().contains_key(&qubit));
            assert!(dag.last_operation_involving_qubit().get(&qubit) == ind.as_ref());
        }
    }
}

#[test_case(Operation::from(CNOT::new(0, 1)))]
#[test_case(Operation::from(PauliX::new(0)))]
fn check_involved_classical_none(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    assert!(dag.first_operation_involving_classical().is_empty());
    assert!(dag.last_operation_involving_classical().is_empty());

    dag.add_to_back(operation.clone());

    assert!(dag.first_operation_involving_classical().is_empty());
    assert!(dag.last_operation_involving_classical().is_empty());
}

#[test_case(Operation::from(MeasureQubit::new(0, "ro".to_string(), 0)), Operation::from(MeasureQubit::new(1, "ro".to_string(), 1)))]
#[test_case(Operation::from(MeasureQubit::new(1, "ro".to_string(), 1)), Operation::from(MeasureQubit::new(2, "ro".to_string(), 2)))]
fn check_involved_classical_set(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let back = dag.add_to_back(operation1.clone());

    if let InvolvedClassical::Set(x) = operation1.involved_classical() {
        let el = x.iter().next().unwrap();
        assert!(dag.last_operation_involving_classical().contains_key(el));
        assert_eq!(
            dag.last_operation_involving_classical().get(el),
            back.as_ref()
        );
    }

    assert!(!dag.last_operation_involving_classical().is_empty());
    assert!(!dag.first_operation_involving_classical().is_empty());

    let front = dag.add_to_front(operation2.clone());

    if let InvolvedClassical::Set(x) = operation2.involved_classical() {
        let el = x.iter().next().unwrap();
        assert!(dag.first_operation_involving_classical().contains_key(el));
        assert_eq!(
            dag.first_operation_involving_classical().get(el),
            front.as_ref()
        );
    }

    let front2 = dag.add_to_front(operation2.clone());

    if let InvolvedClassical::Set(x) = operation2.involved_classical() {
        let el = x.iter().next().unwrap();
        assert_ne!(
            dag.first_operation_involving_classical().get(el),
            front.as_ref()
        );
        assert_eq!(
            dag.first_operation_involving_classical().get(el),
            front2.as_ref()
        );
    }
}

#[test_case(Operation::from(DefinitionComplex::new("ri".to_string(), 4, false)))]
#[test_case(Operation::from(DefinitionBit::new("ri".to_string(), 4, false)))]
fn test_is_definition_classical_populate(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let node = dag.add_to_back(operation.clone());

    assert!(!dag.first_operation_involving_classical().is_empty());
    assert!(!dag.last_operation_involving_classical().is_empty());

    assert!(dag
        .first_operation_involving_classical()
        .contains_key(&("ri".to_string(), 0)));
    assert!(dag
        .first_operation_involving_classical()
        .contains_key(&("ri".to_string(), 1)));
    assert!(dag
        .first_operation_involving_classical()
        .contains_key(&("ri".to_string(), 2)));

    for i in 0..4 {
        assert!(dag
            .first_operation_involving_classical()
            .contains_key(&("ri".to_string(), i)));
        assert_eq!(
            dag.first_operation_involving_classical()
                .get(&("ri".to_string(), i)),
            node.as_ref()
        );
        assert!(dag
            .last_operation_involving_classical()
            .contains_key(&("ri".to_string(), i)));
        assert_eq!(
            dag.last_operation_involving_classical()
                .get(&("ri".to_string(), i)),
            node.as_ref()
        );
    }
}

#[test_case(Operation::from(DefinitionComplex::new("ro".to_string(), 4, false)))]
#[test_case(Operation::from(DefinitionBit::new("ri".to_string(), 3, false)))]
fn check_involved_classical_all(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let back = dag.add_to_back(Operation::from(MeasureQubit::new(0, "ro".to_string(), 0)));
    let front = dag.add_to_front(Operation::from(MeasureQubit::new(1, "ro".to_string(), 1)));

    assert_eq!(
        dag.last_operation_involving_classical()
            .get(&("ro".to_string(), 0)),
        back.as_ref()
    );
    assert_eq!(
        dag.last_operation_involving_classical()
            .get(&("ro".to_string(), 0)),
        back.as_ref()
    );
    assert_eq!(
        dag.last_operation_involving_classical()
            .get(&("ro".to_string(), 1)),
        front.as_ref()
    );
    assert_eq!(
        dag.last_operation_involving_classical()
            .get(&("ro".to_string(), 1)),
        front.as_ref()
    );
    assert_eq!(
        dag.first_operation_involving_classical()
            .get(&("ro".to_string(), 0)),
        back.as_ref()
    );
    assert_eq!(
        dag.first_operation_involving_classical()
            .get(&("ro".to_string(), 0)),
        back.as_ref()
    );
    assert_eq!(
        dag.first_operation_involving_classical()
            .get(&("ro".to_string(), 1)),
        front.as_ref()
    );
    assert_eq!(
        dag.first_operation_involving_classical()
            .get(&("ro".to_string(), 1)),
        front.as_ref()
    );

    let new_back = dag.add_to_back(operation.clone());

    if let InvolvedClassical::All(x) = operation.involved_classical() {
        assert_eq!(
            dag.last_operation_involving_classical()
                .get(&(x.clone(), 0)),
            new_back.as_ref()
        );
        assert_ne!(
            dag.last_operation_involving_classical()
                .get(&(x.clone(), 1)),
            back.as_ref()
        );
    }

    let new_front = dag.add_to_front(operation.clone());

    if let InvolvedClassical::All(x) = operation.involved_classical() {
        assert_eq!(
            dag.first_operation_involving_classical()
                .get(&(x.clone(), 0)),
            new_front.as_ref()
        );
        assert_ne!(
            dag.first_operation_involving_classical()
                .get(&(x.clone(), 1)),
            front.as_ref()
        );
    }
}

#[test_case(vec![Operation::from(CNOT::new(0,1)), Operation::from(PauliX::new(0)), Operation::from(PauliY::new(1))])]
#[test_case(vec![Operation::from(PauliZ::new(0)), Operation::from(ControlledPauliZ::new(1,2))])]
fn test_pragma_conditional(op_vec: Vec<Operation>) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);
    let mut circuit: Circuit = Circuit::new();
    for op in &op_vec {
        circuit.add_operation((*op).clone());
    }

    let operation: Operation =
        Operation::from(PragmaConditional::new("to".to_string(), 2, circuit));

    dag.add_to_back(operation.clone());

    assert!(!dag.first_operation_involving_qubit().is_empty());
    assert!(!dag.last_operation_involving_qubit().is_empty());
}

#[test_case(Operation::from(PauliX::new(0)))]
#[test_case(Operation::from(PauliZ::new(1)))]
#[test_case(Operation::from(CNOT::new(0, 1)))]
#[test_case(Operation::from(PauliY::new(2)))]
fn test_get(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let node = dag.add_to_back(operation.clone());

    assert_eq!(dag.get(node.unwrap()).unwrap(), &operation);
}

#[test]
fn test_execution_blocked() {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let a = dag.add_to_back(Operation::from(PauliX::new(0))).unwrap();
    let b = dag.add_to_back(Operation::from(PauliZ::new(0))).unwrap();
    let c = dag.add_to_back(Operation::from(PauliY::new(1))).unwrap();
    let d = dag.add_to_back(Operation::from(CNOT::new(0, 1))).unwrap();
    let e = dag
        .add_to_back(Operation::from(ControlledPauliZ::new(1, 2)))
        .unwrap();

    assert!(dag.execution_blocked(&[a, b, c], &d).is_empty());
    assert_eq!(dag.execution_blocked(&[a, b, c], &e), vec![d]);
    assert!(dag.execution_blocked(&[a, b, c, d], &e).is_empty());
    assert!(dag.execution_blocked(&[d, e], &a).is_empty());
    assert_eq!(dag.execution_blocked(&[d, e], &b), vec![a]);
    assert_eq!(dag.execution_blocked(&[], &e), vec![a, b, c, d]);
}

#[test]
fn test_blocking_predecessors() {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let a = dag.add_to_back(Operation::from(PauliX::new(0))).unwrap();
    let b = dag.add_to_back(Operation::from(PauliZ::new(0))).unwrap();
    let c = dag.add_to_back(Operation::from(PauliY::new(1))).unwrap();
    let d = dag.add_to_back(Operation::from(CNOT::new(0, 1))).unwrap();

    assert!(dag.blocking_predecessors(&[a, b, c], &d).is_empty());
    assert!(dag.blocking_predecessors(&[a], &b).is_empty());
    assert!(dag.blocking_predecessors(&[], &c).is_empty());
    assert_eq!(dag.blocking_predecessors(&[a,b], &d), vec![c]);
}

#[test]
fn test_new_front_layer() {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let a = dag.add_to_back(Operation::from(PauliX::new(0))).unwrap();
    let b = dag.add_to_back(Operation::from(PauliZ::new(0))).unwrap();
    let c = dag.add_to_back(Operation::from(PauliY::new(1))).unwrap();
    let d = dag.add_to_back(Operation::from(CNOT::new(0, 1))).unwrap();
    let e = dag
        .add_to_back(Operation::from(ControlledPauliZ::new(1, 2)))
        .unwrap();

    assert_eq!(
        dag.new_front_layer(&[a, b, c], &[d], &e),
        Err(RoqoqoError::GenericError {
            msg: "The Operation to be executed is not in the current front layer.".to_string(),
        })
    );
    assert_eq!(dag.new_front_layer(&[], &[a], &a), Ok(vec![b]));
    assert_eq!(dag.new_front_layer(&[a, c], &[b], &b), Ok(vec![d]));
    assert_eq!(dag.new_front_layer(&[a], &[b], &b), Ok(vec![b]));
    assert_eq!(dag.new_front_layer(&[a, b, c], &[d], &d), Ok(vec![e]));
    assert_eq!(dag.new_front_layer(&[a, b, c, d], &[e], &e), Ok(vec![]));
}

#[test]
fn test_parallel_block_iterator() {
    let mut dag: CircuitDag = CircuitDag::with_capacity(DEFAULT_NODE_NUMBER, DEFAULT_EDGE_NUMBER);

    let a = dag.add_to_back(Operation::from(PauliX::new(0))).unwrap();
    let b = dag.add_to_back(Operation::from(PauliZ::new(0))).unwrap();
    let c = dag.add_to_back(Operation::from(PauliY::new(1))).unwrap();
    let d = dag.add_to_back(Operation::from(PauliX::new(1))).unwrap();
    let e = dag.add_to_back(Operation::from(CNOT::new(0, 1))).unwrap();

    let mut par_bl = dag.parallel_blocks();

    let vec = par_bl.next().unwrap();
    for (ind, _) in vec {
        assert!(ind == a || ind == c);
    }

    let vec = par_bl.next().unwrap();
    for (ind, _) in vec {
        assert!(ind == b || ind == d);
    }

    let vec = par_bl.next().unwrap();
    for (ind, _) in vec {
        assert!(ind == e);
    }

    assert!(par_bl.next().is_none());
}
