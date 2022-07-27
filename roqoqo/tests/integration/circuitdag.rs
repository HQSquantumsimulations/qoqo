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

use roqoqo::CircuitDag;

use roqoqo::operations::*;

use test_case::test_case;

/// Test adding an operation that doesn't involve qubits.
/// 
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
fn add_operation_no_involved_qubits(operation: Operation) {
    let mut dag:CircuitDag = CircuitDag::new();
    
    dag.add_to_back(operation.clone());

    assert!(operation.involved_qubits() == InvolvedQubits::None);
    assert_eq!(dag.get_op(0).unwrap(), &operation);
}

/// Test graph node existance after adding an operation that involves qubits.
/// 
#[test_case(Operation::from(PauliX::new(0)))]
#[test_case(Operation::from(PauliY::new(1)))]
#[test_case(Operation::from(ControlledPauliZ::new(0,1)))]
fn check_node_existance(operation: Operation) {
    let mut dag:CircuitDag = CircuitDag::new();

    dag.add_to_back(operation.clone());

    assert!(dag.graph().node_count() == 1)
}

#[test_case(Operation::from(PauliX::new(0)))]
#[test_case(Operation::from(CNOT::new(0,1)))]
fn check_first_last_all_existance(operation: Operation) {
    let mut dag:CircuitDag = CircuitDag::new();

    assert!(dag.first_all().is_none());
    assert!(dag.last_all().is_none());

    dag.add_to_back(operation.clone());

    assert!(dag.first_all().is_some());
    assert!(dag.first_all().is_some());
}

fn check_first_last_all_order() {
    
}