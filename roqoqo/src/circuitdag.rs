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

use std::collections::{HashMap, HashSet};

use crate::operations::*;

use petgraph::adj::NodeIndex;
use petgraph::graph::Graph;

/// Represents a Direct Acyclic Graph (DAG)
#[derive(Debug)]
pub struct CircuitDag {
    // TODO add Ix usize
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
    pub fn new() -> Self {
        CircuitDag {
            graph: Graph::<Operation, ()>::new(),
            commuting_operations: Vec::<Operation>::new(),
            first_parallel_block: HashSet::<NodeIndex>::new(),
            last_parallel_block: HashSet::<NodeIndex>::new(),
            first_all: Option::<NodeIndex>::None,
            last_all: Option::<NodeIndex>::None,
            first_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(),
            last_operation_involving_qubit: HashMap::<usize, NodeIndex>::new(),
            first_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new(),
            last_operation_involving_classical: HashMap::<(String, usize), NodeIndex>::new(),
        }
    }

    /// Adds an operation to the back of the CircuitDag if necessary.
    ///
    /// # Arguments
    ///
    /// * 'operation' - The Operation to add to the end of the CircuitDag.
    pub fn add_to_back(&mut self, operation: Operation) -> () {
        // Push to commuting_operations or create the node and start the
        //  add to back process
        if let InvolvedQubits::None = operation.involved_qubits() {
            self.commuting_operations.push(operation);
        } else {
            let node = self.graph.add_node(operation);
            self.add_to_back_involved(node.index().try_into().unwrap())
        }
    }

    /// Adds an operation that involves some or all qubits to the end of the CircuitDag.
    ///
    /// # Arguments
    ///
    /// * 'operation' - The Operation to add to the end of the CircuitDag.
    fn add_to_back_involved(&mut self, node: NodeIndex) -> () {
        let node_involved_qubits: InvolvedQubits = self
            .graph
            .node_weight(node.into())
            .unwrap()
            .involved_qubits();
        // Calls the proper subfunction depending on the qubits involved
        //  in the operation
        if let InvolvedQubits::Set(x) = node_involved_qubits {
            for qubit in x {
                self.update_from_qubit(node, qubit);
            }
        } else if let InvolvedQubits::All = node_involved_qubits {
            self.update_from_all_operation(node);
        }
    }

    /// Updates the relevant attributes and the graph of CircuitDag from a single qubit involved
    /// in an Operation.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node whose Operation involves the qubit.
    /// * 'qubit' - The qubit involved in the Operation.
    fn update_from_qubit(&mut self, node: NodeIndex, qubit: usize) {
        // Update last_operation_involving qubit and last_parallel_block
        //  depending on current structure
        if let Some(&i) = self.last_operation_involving_qubit.get(&qubit) {
            self.graph.add_edge(i.into(), node.into(), ());
            self.last_parallel_block.remove(&i);
        }
        self.last_operation_involving_qubit.insert(qubit, node);
        self.last_parallel_block.insert(node);

        // Update first_operation_involving_qubit and first_parallel_block
        //  depending on last_all
        if self.last_all.is_none() {
            self.first_operation_involving_qubit.insert(qubit, node);
            self.first_parallel_block.insert(node);
        } else {
            self.first_operation_involving_qubit
                .insert(qubit, self.last_all.unwrap());
            self.first_parallel_block.insert(self.last_all.unwrap());
        }
    }

    /// Updates the relevant attributes and the graph of CircuitDag when the Operation involves all qubits.
    ///
    /// # Arguments
    ///
    /// * 'node' - The index of the node whose Operation involves all qubits.
    fn update_from_all_operation(&mut self, node: NodeIndex) {
        // Update first_all and last_all
        if self.first_all.is_none() {
            self.first_all = Some(node);
        }
        self.last_all = Some(node);

        // Set the node as the only one in last_parallel_block
        self.last_parallel_block.clear();
        self.last_parallel_block.insert(node);

        // All the latest nodes in the graph must now point to the new node and
        //  last_operation_involving_qubit is updated
        let mut temp_map: HashMap<usize, NodeIndex> =
            HashMap::with_capacity(self.last_operation_involving_qubit.capacity());
        for (&qubit, &old_node) in &self.last_operation_involving_qubit {
            self.graph.update_edge(old_node.into(), node.into(), ());
            temp_map.insert(qubit, node);
        }
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

    /// Returns a reference to the HashSet containing the nodes in the last parallel block.
    /// 
    pub fn last_parallel_block(&self) -> &HashSet<NodeIndex> {
        &self.last_parallel_block
    }

    /// Returns a reference to the HashMap where a key represents a qubit and its value represents
    /// the last node the involves that qubit.
    /// 
    pub fn last_operation_involving_qubit(&self) -> &HashMap<usize, NodeIndex> {
        &self.last_operation_involving_qubit
    }
}
