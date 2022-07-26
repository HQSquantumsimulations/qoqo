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

use std::collections::{HashSet, HashMap};

use roqoqo::operations::{Operation, InvolvedQubits};

use petgraph::adj::{NodeIndex, EdgeIndex};
use petgraph::graph::DiGraph;

pub struct CircuitDag {
    graph: DiGraph<NodeIndex, EdgeIndex>,
    commuting_operations: Vec<Operation>,
    first_parallel_block: HashSet<NodeIndex>,
    last_parallel_block: HashSet<NodeIndex>,
    /// None if no Operation with InvovledQubits::All in circuit
    first_all: Option<NodeIndex>,
    /// None if no Operation with InvovledQubits::All in circuit
    last_all: Option<NodeIndex>,
    first_operation_involving_qubit: HashMap<usize, NodeIndex>,
    last_operation_involving_qubit: HashMap<usize, NodeIndex>,
    first_operation_involving_classical: HashMap<(String, usize), NodeIndex>,
    last_operation_involving_classical: HashMap<(String, usize), NodeIndex>,
}

impl CircuitDag {
    fn new() -> Self{
        CircuitDag { 
            graph: DiGraph::new(), 
            commuting_operations: Vec::new(), 
            first_parallel_block: HashSet::new(), 
            last_parallel_block: HashSet::new(), 
            first_all: None, 
            last_all: None, 
            first_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(), 
            last_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(), 
            first_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new(), 
            last_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new()
        }
    }

    fn add_to_back(operation: Operation) -> (){

    }
}