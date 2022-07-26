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

/// Test no-involved-qubits operation
#[test_case(Operation::from(DefinitionBit::new(String::from("ro"), 1, false)); "DefinitionBit")]
fn add_operation_no_involved_qubits(operation: Operation) {
    let mut dag:CircuitDag = CircuitDag::new();
    
    dag.add_to_back(operation);
}