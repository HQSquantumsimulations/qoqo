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

use roqoqo::CircuitDag;
use roqoqo::operations::*;

use test_case::test_case;

/// Test adding an operation that doesn't involve qubits.
///
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
fn add_operation_no_involved_qubits(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::new();

    dag.add_to_back(operation.clone());

    assert!(operation.involved_qubits() == InvolvedQubits::None);
    assert_eq!(dag.commuting_operations().get(0).unwrap(), &operation);

    dag.add_to_front(Operation::from(PauliY::new(0)));
    dag.add_to_back(Operation::from(CNOT::new(0, 1)));

    assert_eq!(dag.commuting_operations().get(0).unwrap(), &operation);
}

/// Test graph node existance after adding an operation that involves qubits.
///
#[test_case(Operation::from(PauliX::new(0)))]
#[test_case(Operation::from(PauliY::new(1)))]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)))]
fn check_node_existance(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::new();

    dag.add_to_back(operation.clone());

    assert!(dag.graph().node_count() == 1);

    dag.add_to_front(operation.clone());
    dag.add_to_back(Operation::from(CNOT::new(0, 1)));

    assert!(dag.graph().node_count() == 3);
}

#[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliY::new(0)))]
#[test_case(Operation::from(PauliZ::new(0)), Operation::from(CNOT::new(0, 1)))]
fn check_node_count(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::new();

    dag.add_to_back(operation1.clone());
    dag.add_to_front(operation2.clone());

    assert!(dag.graph().node_count() == 2);

    dag.add_to_back(operation1.clone());

    assert!(dag.graph().node_count() == 3);
}

#[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliY::new(0)))]
fn check_edge(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::new();

    let ind1 = dag.add_to_back(operation1.clone());
    let ind2 = dag.add_to_back(operation2.clone());

    assert!(dag.graph().contains_edge(ind1.unwrap().into(), ind2.unwrap().into()));
}

#[test_case(
    Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None)),
    true
)]
#[test_case(
    Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None)),
    false
)]
fn check_first_last_all_existence(operation: Operation, back: bool) {
    let mut dag: CircuitDag = CircuitDag::new();

    assert!(dag.first_all().is_none());
    assert!(dag.last_all().is_none());

    if back {
        dag.add_to_back(operation.clone());
    } else {
        dag.add_to_front(operation.clone());
    }

    assert!(dag.first_all().is_some());
    assert!(dag.last_all().is_some());
}

#[test_case(
    Operation::from(PragmaRepeatedMeasurement::new(String::from("ro"), 1, None)),
    Operation::from(PragmaRepeatedMeasurement::new(String::from("ri"), 2, None))
)]
fn check_first_last_all_order(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::new();

    dag.add_to_back(operation1);
    dag.add_to_front(operation2);

    assert!(dag.first_all().is_some());
    assert!(dag.last_all().is_some());

    let f_all = *dag.first_all();
    let l_all = *dag.last_all();

    assert_ne!(
        dag.graph().node_weight(f_all.unwrap().into()),
        dag.graph().node_weight(l_all.unwrap().into())
    );
}

#[test_case(Operation::from(PauliX::new(0)), Operation::from(PauliY::new(1)))]
#[test_case(Operation::from(PauliY::new(1)), Operation::from(PauliZ::new(2)))]
#[test_case(Operation::from(CNOT::new(0, 1)), Operation::from(PauliX::new(1)))]
fn check_parallel_blocks_set(operation1: Operation, operation2: Operation) {
    let mut dag: CircuitDag = CircuitDag::new();
    let mut inv_qubits_1: HashSet<usize> = HashSet::new();
    if let InvolvedQubits::Set(x) = operation1.involved_qubits() {
        for qubit in x {
            inv_qubits_1.insert(qubit);
        }
    }

    dag.add_to_back(operation1.clone());

    assert!(dag.last_parallel_block().len() == 1);
    assert!(dag.last_parallel_block().len() == 1);

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
    let mut dag: CircuitDag = CircuitDag::new();

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
    let mut dag: CircuitDag = CircuitDag::new();

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
fn check_operation_involving_qubits(operation: Operation) {
    let mut dag: CircuitDag = CircuitDag::new();

    dag.add_to_back(Operation::from(PauliZ::new(0)));
    dag.add_to_back(Operation::from(CNOT::new(0, 1)));

    assert!(
        dag.last_operation_involving_qubit().get(&0)
            == dag.last_operation_involving_qubit().get(&1)
    );

    dag.add_to_back(operation.clone());

    if let InvolvedQubits::Set(qubits) = operation.involved_qubits() {
        for qubit in qubits {
            assert!(dag.last_operation_involving_qubit().contains_key(&qubit));
        }
    }

    // TODO: same thing but check node. Needs add_to_back to return the node.
}
