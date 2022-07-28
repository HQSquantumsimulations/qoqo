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

use crate::operations::*;

use petgraph::adj::NodeIndex;
use petgraph::graph::Graph;

/// Represents a Direct Acyclic Graph (DAG) 
#[derive(Debug)]
pub struct CircuitDag {
    graph: Graph<Operation, ()>,
    commuting_operations: Vec<Operation>,
    first_parallel_block: HashSet<NodeIndex>,
    last_parallel_block: HashSet<NodeIndex>,
    /// None if no Operation with InvolvedQubits::All in circuit
    first_all: Option<NodeIndex>,
    /// None if no Operation with InvolvedQubits::All in circuit
    last_all: Option<NodeIndex>,
    first_operation_involving_qubit: HashMap<usize, NodeIndex>,
    last_operation_involving_qubit: HashMap<usize, NodeIndex>,
    first_operation_involving_classical: HashMap<(String, usize), NodeIndex>,
    last_operation_involving_classical: HashMap<(String, usize), NodeIndex>,
}

impl CircuitDag {
    /// Creates a new empty CircuitDag.
    pub fn new() -> Self{
        CircuitDag { 
            graph: Graph::<Operation, ()>::new(), 
            commuting_operations: Vec::<Operation>::new(), 
            first_parallel_block: HashSet::<NodeIndex>::new(), 
            last_parallel_block: HashSet::<NodeIndex>::new(), 
            first_all: Option::<NodeIndex>::None,
            last_all: Option::<NodeIndex>:: None, 
            first_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(), 
            last_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(), 
            first_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new(), 
            last_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new()
        }
    }

    /// Adds an operation to the back of the CircuitDag.
    /// 
    /// # Arguments
    /// 
    /// * 'operation' - The Operation to add to the end of the CircuitDag.
    pub fn add_to_back(&mut self, operation: Operation) -> (){
        let node = self.graph.add_node(operation.clone());

        if let InvolvedQubits::None = operation.involved_qubits() {
            self.commuting_operations.push(operation);
        } else {
            self.add_to_back_involved(node.index().try_into().unwrap())
        }
    }

    /// Adds an operation that involves some or all qubits to the end of the CircuitDag.
    /// 
    /// # Arguments
    /// 
    /// * 'operation' - The Operation to add to the end of the CircuitDag.
    fn add_to_back_involved(&mut self, node: NodeIndex) -> (){
        let node_involved_qubits: InvolvedQubits = self.graph.node_weight(node.into()).unwrap().involved_qubits();
        if let InvolvedQubits::Set(x) = node_involved_qubits.clone() {
            for qubit in x {
                
            }
        } else if let InvolvedQubits::All = node_involved_qubits {
            self.update_from_all_operation(node);
        }
    }

    /// Updates the relevant attributes when the Operation involves all qubits.
    /// 
    /// # Arguments
    /// 
    /// * 'node' - The index of the node whose Operation involves all qubits.
    fn update_from_all_operation(&mut self, node: NodeIndex) {

        // Update first_all and last_all
        self.update_first_last_all(node);

        // Clear and update last_parallel_block
        self.last_parallel_block.clear();
        self.last_parallel_block.insert(node);

        // All the latest nodes in the graph must now point to the new node and
        // last_operation_involving_qubit is updated
        for (_, old_node) in self.last_operation_involving_qubit.iter_mut() {
            self.graph.update_edge((*old_node).into(), node.into(), ());
            *old_node = node;
        }
    }

    /// Checks whether to update first_all or not and updates last_all.
    /// 
    /// # Arguments
    /// 
    /// * 'node' - The index of the node used to update first_all or last_all.
    fn update_first_last_all(&mut self, node: NodeIndex) {
        if self.first_all.is_none() {
            self.first_all = Some(node);
        }
        self.last_all = Some(node);
    }

    /// Returns a reference to the Operation at index.
    /// 
    /// # Arguments
    /// 
    /// * 'index' - The index of the Operation to get from CircuitDag.
    pub fn get_op(&self, index: usize) -> Option<&Operation> {
        self.commuting_operations.get(index)
    }

    /// Returns a reference to the vector of commuting operations in CircuitDag.
    /// 
    pub fn commuting_operations(&self) -> &Vec<Operation> {
        &self.commuting_operations
    }

    /// Returns a reference to the graph in CircuitDag.
    /// 
    pub fn graph(&self) -> &Graph<Operation, ()> {
        &self.graph
    }

    /// Returns a reference to the first Operation that involves all qubits in CircuitDag.
    /// 
    pub fn first_all(&self) -> &Option<u32> {
        &self.first_all
    }

    /// Returns a reference to the last Operation that involves all qubits in CircuitDag.
    /// 
    pub fn last_all(&self) -> &Option<u32> {
        &self.last_all
    }

}